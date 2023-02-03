use anyhow::{Context, Result};
use id_arena::{Arena, Id};
use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::{graph::NodeIndex, graphmap::GraphMap, Directed, Graph};
use regex::Regex;
use std::cmp::max;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct ValveData {
    nid: NodeIndex,
    rate: i32,
}

fn higher_bound_max_pressure(
    valve_data_map: &HashMap<String, ValveData>,
    closed_valves: &Vec<String>,
    remaining_time: i32,
) -> i32 {
    // Best case scenario: we open the next best valve every 2 minutes: 1 minute to move to the valve, 1 minute to open it.
    closed_valves
        .iter()
        .map(|id| valve_data_map[id].rate)
        .into_iter()
        .sorted_by(|r1, r2| r2.cmp(r1))
        .take(max(0, remaining_time - 1) as usize / 2)
        .enumerate()
        .map(|(i, rate)| rate * (remaining_time - 2 * (i as i32 + 1)))
        .sum()
}

// TODO rewrite with Arena Ids instead of strings
// Recursive tree search using depth first strategy
fn depth_first(
    valve_distance_map: &HashMap<String, HashMap<String, i32>>,
    valve_data_map: &HashMap<String, ValveData>,
    curr: String,
    path: Vec<String>, // for debug
    closed_valves: Vec<String>,
    remaining_time: i32,
    curr_pressure: i32,
    best_pressure: i32,
) -> i32 {
    #[derive(Debug)]
    struct Candidate {
        next: String,
        distance: i32,
        gain: i32,
    }

    // list next valves candidate
    let mut candidates = Vec::new();
    for next in closed_valves.iter() {
        let distance = valve_distance_map[&curr][next];
        let rate = valve_data_map[next].rate;
        if remaining_time <= distance + 1 {
            continue; // no time to open the next one
        }
        let gain = rate * (remaining_time - distance - 1);
        candidates.push(Candidate {
            next: next.clone(),
            distance,
            gain,
        });
    }

    // sort candidates so we explore highest gain first
    candidates.sort_by(|c1, c2| c2.gain.cmp(&c1.gain));

    let mut best_pressure = max(curr_pressure, best_pressure);

    for candidate in candidates {
        // evaluate potential of candidate
        let closed_valves = closed_valves
            .iter()
            .filter(|&next| next != &candidate.next)
            .map(|next| next.to_owned())
            .collect_vec();
        let remaining_time = remaining_time - candidate.distance - 1;
        let curr_pressure = curr_pressure + candidate.gain;
        let potential_pressure = curr_pressure
            + higher_bound_max_pressure(&valve_data_map, &closed_valves, remaining_time);
        if potential_pressure < best_pressure {
            continue; // no need to explore
        }
        let mut path = path.clone();
        path.push(candidate.next.clone());
        // explore down this path
        best_pressure = max(
            best_pressure,
            depth_first(
                &valve_distance_map,
                &valve_data_map,
                candidate.next,
                path,
                closed_valves,
                remaining_time,
                curr_pressure,
                best_pressure,
            ),
        );
    }

    best_pressure
}

fn main() -> Result<()> {
    // part 1
    let file = File::open("data/16.txt")?;
    let re = Regex::new(
        r"Valve (?P<valve>\w{2}) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<next>\w{2}(, \w{2})*)",
    )?;

    // build graph of all reachable valves
    let mut edges = Vec::new();
    let mut graph = Graph::<String, (), Directed>::new();
    let mut valve_data_map = HashMap::<String, ValveData>::new();
    let mut target_valves = Vec::<String>::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        let caps = re
            .captures(&line)
            .with_context(|| format!("Failed to parse line: {}", &line))?;
        let valve = caps["valve"].to_owned();
        let rate = caps["rate"].parse::<i32>()?;
        let next = caps["next"]
            .split(", ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        if rate > 0 {
            target_valves.push(valve.clone());
        }
        edges.extend(next.into_iter().map(|n| (valve.clone(), n)));
        let nid = graph.add_node(valve.clone());
        valve_data_map.insert(valve, ValveData { nid, rate });
    }

    let edges = edges
        .into_iter()
        .map(|(s, e)| (valve_data_map[&s].nid, valve_data_map[&e].nid))
        .collect_vec();
    graph.extend_with_edges(edges);

    // build simple distance lookup map with "AA" and all the valves that are worth opening
    // for any (s,e) pair of target valves, target_valve_distance_map[s][e] gives the shortest distance from s to e.
    let mut valve_distance_map = HashMap::new();
    for valve in target_valves.iter().chain(["AA".to_string()].iter()) {
        let nid = valve_data_map[valve].nid;
        let distance_map = dijkstra(&graph, nid, None, |_| 1);
        let mut tmp = HashMap::new();
        for next_valve in target_valves.iter() {
            let next_nid = valve_data_map[next_valve].nid;
            tmp.insert(next_valve.clone(), distance_map[&next_nid]);
        }
        valve_distance_map.insert(valve.clone(), tmp);
    }

    // search the best pressure in a tree fashion, depth first
    let best_pressure = depth_first(
        &valve_distance_map,
        &valve_data_map,
        "AA".to_string(),
        vec!["AA".to_string()],
        target_valves,
        30,
        0,
        0,
    );
    println!("{best_pressure}");

    Ok(())
}

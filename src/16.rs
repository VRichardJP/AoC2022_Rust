use anyhow::{Context, Result};
use id_arena::{Arena, Id};
use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::{graphmap::GraphMap, Directed};
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Default)]
struct ValveData {
    rate: i32,
    next_valves: Vec<ValveId>,
    distance_map: HashMap<ValveId, i32>,
}

type ValveId = Id<ValveData>;
type ValveArena = Arena<ValveData>;

fn higher_bound_max_pressure(
    valve_area: &ValveArena,
    closed_valves: &[ValveId],
    remaining_time: i32,
) -> i32 {
    // Best case scenario: we open the next best valve every 2 minutes: 1 minute to move to the valve, 1 minute to open it.
    closed_valves
        .iter()
        .map(|&id| valve_area[id].rate)
        .sorted_by(|r1, r2| r2.cmp(r1))
        .take(max(0, remaining_time - 1) as usize / 2)
        .enumerate()
        .map(|(i, rate)| rate * (remaining_time - 2 * (i as i32 + 1)))
        .sum()
}

// Recursive tree search using depth first strategy
fn depth_first(
    valve_arena: &ValveArena,
    curr_id: ValveId,
    closed_valves: Vec<ValveId>,
    remaining_time: i32,
    curr_pressure: i32,
    best_pressure: i32,
) -> i32 {
    #[derive(Debug)]
    struct Candidate {
        next_id: ValveId,
        distance: i32,
        gain: i32,
    }

    // list next valves candidate
    let mut candidates = Vec::new();
    for &next_id in closed_valves.iter() {
        let distance = valve_arena[curr_id].distance_map[&next_id];
        if remaining_time <= distance + 1 {
            continue; // no time to open the next one
        }
        let rate = valve_arena[next_id].rate;
        let gain = rate * (remaining_time - distance - 1);
        candidates.push(Candidate {
            next_id,
            distance,
            gain,
        });
    }

    // sort candidates so we explore highest gain first
    candidates.sort_by(|c1, c2| c2.gain.cmp(&c1.gain));

    let mut best_pressure = max(curr_pressure, best_pressure);

    for candidate in candidates {
        // evaluate candidate potential
        let closed_valves = closed_valves
            .iter()
            .filter(|&next_id| *next_id != candidate.next_id)
            .map(|next_id| next_id.to_owned())
            .collect_vec();
        let remaining_time = remaining_time - candidate.distance - 1;
        let curr_pressure = curr_pressure + candidate.gain;
        let potential_pressure =
            curr_pressure + higher_bound_max_pressure(valve_arena, &closed_valves, remaining_time);
        if potential_pressure < best_pressure {
            continue; // already found better, no need to explore
        }
        // explore down this path
        best_pressure = max(
            best_pressure,
            depth_first(
                valve_arena,
                candidate.next_id,
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
    let re = Regex::new(
        r"Valve (?P<valve>\w{2}) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<next_valves>\w{2}(, \w{2})*)",
    )?;

    // parse input file
    let mut valve_arena = ValveArena::new();
    let mut valve_id_lookup_map = HashMap::<String, ValveId>::new();
    for line in fs::read_to_string("data/16.txt")?.lines() {
        let caps = re
            .captures(line)
            .with_context(|| format!("Failed to parse line: {}", &line))?;
        let valve = caps["valve"].to_owned();
        let rate = caps["rate"].parse::<i32>()?;
        let next_valves = caps["next_valves"]
            .split(", ")
            .map(|s| {
                *valve_id_lookup_map
                    .entry(s.to_string())
                    .or_insert_with(|| valve_arena.alloc(ValveData::default()))
            })
            .collect_vec();

        if let Some(id) = valve_id_lookup_map.get(valve.as_str()) {
            // finish initialization
            valve_arena[*id].rate = rate;
            valve_arena[*id].next_valves = next_valves;
        } else {
            // insert new
            let id = valve_arena.alloc(ValveData {
                rate,
                next_valves,
                ..Default::default()
            });
            valve_id_lookup_map.insert(valve, id);
        }
    }

    // build graph for shortest path search
    let mut edges = Vec::new();
    for (curr_id, valve_data) in valve_arena.iter() {
        edges.extend(
            valve_data
                .next_valves
                .iter()
                .map(|&next_id| (curr_id, next_id)),
        );
    }
    let graph = GraphMap::<ValveId, (), Directed>::from_edges(edges);

    // list valves worth opening (rate > 0)
    let start_id = *valve_id_lookup_map.get("AA").unwrap();
    let target_valves = valve_arena
        .iter()
        .filter_map(|(id, valve_data)| if valve_data.rate > 0 { Some(id) } else { None })
        .collect_vec();

    // for each (and start point), find shortest distance to other target valves (we don't care about other valves)
    valve_arena[start_id].distance_map = dijkstra(&graph, start_id, None, |_| 1);
    for &valve_id in target_valves.iter() {
        valve_arena[valve_id].distance_map = dijkstra(&graph, valve_id, None, |_| 1);
    }

    // search the best pressure in a tree fashion, depth first
    let best_pressure = depth_first(&valve_arena, start_id, target_valves, 30, 0, 0);
    println!("{best_pressure}");

    Ok(())
}

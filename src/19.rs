use anyhow::{Context, Result};

const INPUT: &str = include_str!("../data/19.txt");

#[derive(Debug, Clone, Copy)]
struct State {
    remaining_time: u32,
    // resources
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    // workers
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: u32,
    geode_robot: u32,
}

#[derive(Debug, Clone, Copy)]
struct BuildCost {
    ore_cost: u32,
    clay_cost: u32,
    obsidian_cost: u32,
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    ore_robot_cost: BuildCost,
    clay_robot_cost: BuildCost,
    obsidian_robot_cost: BuildCost,
    geode_robot_cost: BuildCost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BuildAction {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot,
}
use regex::Regex;
use BuildAction::*;

impl State {
    // initial state
    fn new(remaining_time: u32) -> Self {
        Self {
            remaining_time,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
        }
    }
}

// give an optimistic estimate of maximum geode production
// since we don't know which build order is optimal, we simply assume all robots
// can be built simultaneously without extra cost
fn get_high_bound_geode_production(state: &State, blueprint: &Blueprint) -> u32 {
    let mut state = *state;
    let mut built_ore_robot = 0;
    let mut built_clay_robot = 0;
    let mut built_obsidian_robot = 0;
    let mut built_geode_robot = 0;
    while state.remaining_time > 0 {
        state.remaining_time -= 1;
        state.ore += state.ore_robot + built_ore_robot;
        state.clay += state.clay_robot + built_clay_robot;
        state.obsidian += state.obsidian_robot + built_obsidian_robot;
        state.geode += state.geode_robot + built_geode_robot;
        built_ore_robot = state.ore / blueprint.ore_robot_cost.ore_cost;
        built_clay_robot = state.ore / blueprint.clay_robot_cost.ore_cost;
        built_obsidian_robot = std::cmp::min(
            state.ore / blueprint.obsidian_robot_cost.ore_cost,
            state.clay / blueprint.obsidian_robot_cost.clay_cost,
        );
        built_geode_robot = std::cmp::min(
            state.ore / blueprint.geode_robot_cost.ore_cost,
            state.obsidian / blueprint.geode_robot_cost.obsidian_cost,
        );
    }
    state.geode
}

// step forward 1 minute, assume the action is possible
fn step(state: &State, blueprint: &Blueprint, action: Option<BuildAction>) -> State {
    let mut next = *state;
    next.remaining_time -= 1;
    next.ore += state.ore_robot;
    next.clay += state.clay_robot;
    next.obsidian += state.obsidian_robot;
    next.geode += state.geode_robot;
    match action {
        Some(OreRobot) => {
            next.ore -= blueprint.ore_robot_cost.ore_cost;
            next.ore_robot += 1;
        }
        Some(ClayRobot) => {
            next.ore -= blueprint.clay_robot_cost.ore_cost;
            next.clay_robot += 1;
        }
        Some(ObsidianRobot) => {
            next.ore -= blueprint.obsidian_robot_cost.ore_cost;
            next.clay -= blueprint.obsidian_robot_cost.clay_cost;
            next.obsidian_robot += 1;
        }
        Some(GeodeRobot) => {
            next.ore -= blueprint.geode_robot_cost.ore_cost;
            next.obsidian -= blueprint.geode_robot_cost.obsidian_cost;
            next.geode_robot += 1;
        }
        None => (),
    };
    next
}

// list possible actions in order for a depth first search (poping element from back)
fn get_possible_actions(state: &State, blueprint: &Blueprint) -> Vec<Option<BuildAction>> {
    let mut possible_actions = Vec::new();
    // there is no point in saving ressources if we can build everything
    let mut is_saving_valid_strategy = false;
    if state.ore >= blueprint.ore_robot_cost.ore_cost {
        possible_actions.push(Some(OreRobot));
    } else if state.ore_robot > 0 {
        is_saving_valid_strategy = true;
    }
    if state.ore >= blueprint.clay_robot_cost.ore_cost {
        possible_actions.push(Some(ClayRobot));
    } else if state.ore_robot > 0 {
        is_saving_valid_strategy = true;
    }
    if state.ore >= blueprint.obsidian_robot_cost.ore_cost
        && state.clay >= blueprint.obsidian_robot_cost.clay_cost
    {
        possible_actions.push(Some(ObsidianRobot));
    } else if state.ore_robot > 0 && state.clay_robot > 0 {
        is_saving_valid_strategy = true;
    }
    if state.ore >= blueprint.geode_robot_cost.ore_cost
        && state.obsidian >= blueprint.geode_robot_cost.obsidian_cost
    {
        possible_actions.push(Some(GeodeRobot));
    } else if state.ore_robot > 0 && state.obsidian_robot > 0 {
        is_saving_valid_strategy = true;
    } 
    if is_saving_valid_strategy {
        possible_actions.push(None);
    }
    possible_actions
}

// A* search (depth first) of highest geode production
fn get_best_geode_production(state: &State, blueprint: &Blueprint) -> u32 {
    let mut to_explore = vec![(*state, Vec::new())];
    let mut best_geode_prod = 0;

    while let Some((state, history)) = to_explore.pop() {
        if get_high_bound_geode_production(&state, blueprint) <= best_geode_prod {
            // we have already found better
            continue;
        }
        let actions = get_possible_actions(&state, blueprint);
        for action in actions {
            let next = step(&state, blueprint, action);
            let mut next_history = history.clone();
            next_history.push(action);
            if next.remaining_time == 0 {
                best_geode_prod = best_geode_prod.max(next.geode);
            } else {
                to_explore.push((next, next_history));
            }
        }
    }

    best_geode_prod
}

fn main() -> Result<()> {
    // part 1
    let re = Regex::new(
        r"Blueprint (?P<id>\d+): Each ore robot costs (?P<ore_robot_ore_cost>\d+) ore. Each clay robot costs (?P<clay_robot_ore_cost>\d+) ore. Each obsidian robot costs (?P<obsidian_robot_ore_cost>\d+) ore and (?P<obsidian_robot_clay_cost>\d+) clay. Each geode robot costs (?P<geode_robot_ore_cost>\d+) ore and (?P<geode_robot_obsidian_cost>\d+) obsidian.",
    )?;
    let mut blueprints = Vec::new();
    for line in INPUT.lines() {
        let caps = re
            .captures(line)
            .with_context(|| format!("failed to parse line: {line}"))?;
        let id = caps["id"].parse::<u32>()?;
        let ore_robot_cost = BuildCost {
            ore_cost: caps["ore_robot_ore_cost"].parse::<u32>()?,
            clay_cost: 0,
            obsidian_cost: 0,
        };
        let clay_robot_cost = BuildCost {
            ore_cost: caps["clay_robot_ore_cost"].parse::<u32>()?,
            clay_cost: 0,
            obsidian_cost: 0,
        };
        let obsidian_robot_cost = BuildCost {
            ore_cost: caps["obsidian_robot_ore_cost"].parse::<u32>()?,
            clay_cost: caps["obsidian_robot_clay_cost"].parse::<u32>()?,
            obsidian_cost: 0,
        };
        let geode_robot_cost = BuildCost {
            ore_cost: caps["geode_robot_ore_cost"].parse::<u32>()?,
            clay_cost: 0,
            obsidian_cost: caps["geode_robot_obsidian_cost"].parse::<u32>()?,
        };
        let blueprint = Blueprint {
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        };
        blueprints.push((id, blueprint));
    }

    let mut sum_quality = 0;
    for (id, blueprint) in blueprints.iter() {
        let state = State::new(24);
        let best_geo_production = get_best_geode_production(&state, &blueprint);
        let quality = id * best_geo_production;
        sum_quality += quality;
    }
    println!("{sum_quality}");

    // part 2
    let mut product_quality = 1;
    for (_, blueprint) in blueprints.iter().take(3) {
        let state = State::new(32);
        let best_geo_production = get_best_geode_production(&state, &blueprint);
        product_quality *= best_geo_production;
    }
    println!("{product_quality}");


    Ok(())
}

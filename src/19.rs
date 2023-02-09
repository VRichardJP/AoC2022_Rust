use anyhow::{Context, Result};
use regex::Regex;
use std::ops::{Index, IndexMut};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
enum Mineral {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
use Mineral::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Minerals<T>
where
    T: Clone + Copy + PartialEq + Eq + Default,
{
    ore: T,
    clay: T,
    obsidian: T,
    geode: T,
}

impl<T> Index<Mineral> for Minerals<T>
where
    T: Clone + Copy + PartialEq + Eq + Default,
{
    type Output = T;

    fn index(&self, index: Mineral) -> &Self::Output {
        match index {
            Ore => &self.ore,
            Clay => &self.clay,
            Obsidian => &self.obsidian,
            Geode => &self.geode,
        }
    }
}

impl<T> IndexMut<Mineral> for Minerals<T>
where
    T: Clone + Copy + PartialEq + Eq + Default,
{
    fn index_mut(&mut self, index: Mineral) -> &mut Self::Output {
        match index {
            Ore => &mut self.ore,
            Clay => &mut self.clay,
            Obsidian => &mut self.obsidian,
            Geode => &mut self.geode,
        }
    }
}

type UInt = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    remaining_time: UInt,
    resources: Minerals<UInt>,
    robots: Minerals<UInt>,
}

#[derive(Debug)]
struct OreRobotCost {
    ore_cost: UInt,
}

#[derive(Debug)]
struct ClayRobotCost {
    ore_cost: UInt,
}

#[derive(Debug)]
struct ObsidianRobotCost {
    ore_cost: UInt,
    clay_cost: UInt,
}

#[derive(Debug)]
struct GeodeRobotCost {
    ore_cost: UInt,
    obsidian_cost: UInt,
}

// type Blueprint = Minerals<Minerals<UInt>>;
#[derive(Debug)]
struct Blueprint {
    ore_robot: OreRobotCost,
    clay_robot: ClayRobotCost,
    obsidian_robot: ObsidianRobotCost,
    geode_robot: GeodeRobotCost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BuildRobot(Mineral);

impl State {
    // initial state
    fn new(remaining_time: UInt, initial_robots: Minerals<UInt>) -> Self {
        Self {
            remaining_time,
            resources: Minerals::<UInt>::default(),
            robots: initial_robots,
        }
    }
}

// give an optimistic estimate of maximum geode production
// since we don't know which build order is optimal, we simply assume all robots
// can be built simultaneously without extra cost
fn get_high_bound_geode_production(state: &State, blueprint: &Blueprint) -> UInt {
    let mut state = *state;
    let mut built_robots = Minerals::<UInt>::default();
    while state.remaining_time > 0 {
        state.remaining_time -= 1;
        // mining minerals
        for mineral in Mineral::iter() {
            state.resources[mineral] += state.robots[mineral] + built_robots[mineral];
        }
        // building more mining robots
        // assume each type of robot can be built at the same time
        built_robots[Ore] = state.resources[Ore] / blueprint.ore_robot.ore_cost;
        built_robots[Clay] = state.resources[Ore] / blueprint.clay_robot.ore_cost;
        built_robots[Obsidian] = std::cmp::min(
            state.resources[Ore] / blueprint.obsidian_robot.ore_cost,
            state.resources[Clay] / blueprint.obsidian_robot.clay_cost,
        );
        built_robots[Geode] = std::cmp::min(
            state.resources[Ore] / blueprint.geode_robot.ore_cost,
            state.resources[Obsidian] / blueprint.geode_robot.obsidian_cost,
        );
    }
    state.resources[Geode]
}

// step forward 1 minute, assume the action is possible
fn step(state: &State, blueprint: &Blueprint, action: Option<BuildRobot>) -> State {
    let mut next = *state;
    // tick
    next.remaining_time -= 1;
    // mining minerals
    for mineral in Mineral::iter() {
        next.resources[mineral] += state.robots[mineral];
    }
    // execute robot build action (if any)
    match action {
        Some(BuildRobot(Ore)) => {
            assert!(next.resources[Ore] >= blueprint.ore_robot.ore_cost);
            next.resources[Ore] -= blueprint.ore_robot.ore_cost;
            next.robots[Ore] += 1;
        }
        Some(BuildRobot(Clay)) => {
            assert!(next.resources[Ore] >= blueprint.clay_robot.ore_cost);
            next.resources[Ore] -= blueprint.clay_robot.ore_cost;
            next.robots[Clay] += 1;
        }
        Some(BuildRobot(Obsidian)) => {
            assert!(next.resources[Ore] >= blueprint.obsidian_robot.ore_cost);
            assert!(next.resources[Clay] >= blueprint.obsidian_robot.clay_cost);
            next.resources[Ore] -= blueprint.obsidian_robot.ore_cost;
            next.resources[Clay] -= blueprint.obsidian_robot.clay_cost;
            next.robots[Obsidian] += 1;
        }
        Some(BuildRobot(Geode)) => {
            assert!(next.resources[Ore] >= blueprint.geode_robot.ore_cost);
            assert!(next.resources[Obsidian] >= blueprint.geode_robot.obsidian_cost);
            next.resources[Ore] -= blueprint.geode_robot.ore_cost;
            next.resources[Obsidian] -= blueprint.geode_robot.obsidian_cost;
            next.robots[Geode] += 1;
        }
        None => (),
    };
    next
}

// list possible actions in order for a depth first search (poping element from back)
fn get_possible_actions(state: &State, blueprint: &Blueprint) -> Vec<Option<BuildRobot>> {
    let mut possible_actions = Vec::new();
    // waiting is only a valid strategy we lack resources to build more robots
    let mut is_saving_valid_strategy = false;
    if state.resources[Ore] >= blueprint.ore_robot.ore_cost {
        // have enough resources
        possible_actions.push(Some(BuildRobot(Ore)));
    } else if state.robots[Ore] > 0 {
        // not enough resources yet, but could just wait
        is_saving_valid_strategy = true;
    }
    if state.resources[Ore] >= blueprint.clay_robot.ore_cost {
        // have enough resources
        possible_actions.push(Some(BuildRobot(Clay)));
    } else if state.robots[Ore] > 0 {
        // not enough resources yet, but could just wait
        is_saving_valid_strategy = true;
    }
    if state.resources[Ore] >= blueprint.obsidian_robot.ore_cost
        && state.resources[Clay] >= blueprint.obsidian_robot.clay_cost
    {
        // have enough resources
        possible_actions.push(Some(BuildRobot(Obsidian)));
    } else if state.robots[Ore] > 0 && state.robots[Clay] > 0 {
        // not enough resources yet, but could just wait
        is_saving_valid_strategy = true;
    }
    if state.resources[Ore] >= blueprint.geode_robot.ore_cost
        && state.resources[Obsidian] >= blueprint.geode_robot.obsidian_cost
    {
        // have enough resources
        possible_actions.push(Some(BuildRobot(Geode)));
    } else if state.robots[Ore] > 0 && state.robots[Obsidian] > 0 {
        // not enough resources yet, but could just wait
        is_saving_valid_strategy = true;
    }
    if is_saving_valid_strategy {
        possible_actions.push(None);
    }
    possible_actions
}

// A* search (depth first) of highest geode production
fn get_best_geode_production(state: &State, blueprint: &Blueprint) -> UInt {
    let mut to_explore = vec![*state];
    let mut best_geode_prod = 0;

    while let Some(state) = to_explore.pop() {
        if get_high_bound_geode_production(&state, blueprint) <= best_geode_prod {
            // we have already found better
            continue;
        }
        let actions = get_possible_actions(&state, blueprint);
        for action in actions {
            let next = step(&state, blueprint, action);
            if next.remaining_time == 0 {
                best_geode_prod = best_geode_prod.max(next.resources[Geode]);
            } else {
                to_explore.push(next);
            }
        }
    }

    best_geode_prod
}

const INPUT: &str = include_str!("../data/19.txt");

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
        let id = caps["id"].parse::<UInt>()?;
        let ore_robot = OreRobotCost {
            ore_cost: caps["ore_robot_ore_cost"].parse::<UInt>()?,
        };
        let clay_robot = ClayRobotCost {
            ore_cost: caps["clay_robot_ore_cost"].parse::<UInt>()?,
        };
        let obsidian_robot = ObsidianRobotCost {
            ore_cost: caps["obsidian_robot_ore_cost"].parse::<UInt>()?,
            clay_cost: caps["obsidian_robot_clay_cost"].parse::<UInt>()?,
        };
        let geode_robot = GeodeRobotCost {
            ore_cost: caps["geode_robot_ore_cost"].parse::<UInt>()?,
            obsidian_cost: caps["geode_robot_obsidian_cost"].parse::<UInt>()?,
        };
        let blueprint = Blueprint {
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        };
        blueprints.push((id, blueprint));
    }

    let mut sum_quality = 0;
    for (id, blueprint) in blueprints.iter() {
        let state = State::new(
            24,
            Minerals {
                ore: 1,
                ..Default::default()
            },
        );
        let best_geo_production = get_best_geode_production(&state, blueprint);
        let quality = id * best_geo_production;
        sum_quality += quality;
    }
    println!("{sum_quality}");

    // part 2
    let mut product_quality = 1;
    for (_, blueprint) in blueprints.iter().take(3) {
        let state = State::new(
            32,
            Minerals {
                ore: 1,
                ..Default::default()
            },
        );
        let best_geo_production = get_best_geode_production(&state, blueprint);
        product_quality *= best_geo_production;
    }
    println!("{product_quality}");

    Ok(())
}

use anyhow::{Context, Result};
use regex::Regex;
use std::cmp::{max, min};

#[derive(Debug, Default, Clone, Copy)]
struct State {
    time: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    ore_robot: i32,
    clay_robot: i32,
    obsidian_robot: i32,
    geode_robot: i32,
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    ore_robot_ore_cost: i32,
    clay_robot_ore_cost: i32,
    obsidian_robot_ore_cost: i32,
    obsidian_robot_clay_cost: i32,
    geode_robot_ore_cost: i32,
    geode_robot_obsidian_cost: i32,
}

const INPUT: &str = include_str!("../data/19.txt");

/// give high bound of geode production from given state
fn high_bound_geode_production(mut state: State, blueprint: &Blueprint) -> i32 {
    // since we don't know which robot is better to build, we do as if it was possible to build
    // simultaneously any robot without impact on the ressources available for other robots
    let mut built_ore_robot = 0;
    let mut built_clay_robot = 0;
    let mut built_obsidian_robot = 0;
    let mut built_geode_robot = 0;
    while state.time != 0 {
        // precompute state after mining
        let mut next = state;
        next.time -= 1;
        next.ore += state.ore_robot + built_ore_robot;
        next.clay += state.clay_robot + built_clay_robot;
        next.obsidian += state.obsidian_robot + built_obsidian_robot;
        next.geode += state.geode_robot + built_geode_robot;
        // build whichever robot we can
        built_ore_robot = state.ore / blueprint.ore_robot_ore_cost;
        built_clay_robot = state.ore / blueprint.clay_robot_ore_cost;
        built_obsidian_robot = min(
            state.ore / blueprint.obsidian_robot_ore_cost,
            state.clay / blueprint.obsidian_robot_clay_cost,
        );
        built_geode_robot = min(
            state.ore / blueprint.geode_robot_ore_cost,
            state.obsidian / blueprint.geode_robot_obsidian_cost,
        );
        // step
        state = next;
    }
    state.geode
}

/// return maximum geode production that can be achieve from given state
/// or best_prod, whichever is bigger
fn max_geode_production(state: State, blueprint: &Blueprint, mut best_prod: i32) -> i32 {
    if state.time == 0 {
        // time out
        return best_prod.max(state.geode);
    }

    // check if the state can produce more than what we have found so far
    if high_bound_geode_production(state, blueprint) <= best_prod {
        // we have found better already
        return best_prod;
    }

    // precompute state after mining
    let mut next = state;
    next.time -= 1;
    next.ore += state.ore_robot;
    next.clay += state.clay_robot;
    next.obsidian += state.obsidian_robot;
    next.geode += state.geode_robot;

    // Explore the several choices we have
    // Remarks:
    // 1. Since we can only produce 1 robot per turn, there is no need to have more
    // ore/clay/obsidian robots than the maximum cost in ore/clay/obsidian robot
    // 2. As far as geode production is concerned, a path is only worth exploring if
    // it is possible to build more geode robots (unused)

    // A. do nothing
    best_prod = max_geode_production(next, blueprint, best_prod);
    // B. build geode robot
    if state.ore >= blueprint.geode_robot_ore_cost
        && state.obsidian >= blueprint.geode_robot_obsidian_cost
    {
        let mut next = next;
        next.ore -= blueprint.geode_robot_ore_cost;
        next.obsidian -= blueprint.geode_robot_obsidian_cost;
        next.geode_robot += 1;
        best_prod = max_geode_production(next, blueprint, best_prod);
    }
    // C. build obsidian robot
    if state.ore >= blueprint.obsidian_robot_ore_cost
        && state.clay >= blueprint.obsidian_robot_clay_cost
        && state.obsidian_robot < blueprint.geode_robot_obsidian_cost
    {
        let mut next = next;
        next.ore -= blueprint.obsidian_robot_ore_cost;
        next.clay -= blueprint.obsidian_robot_clay_cost;
        next.obsidian_robot += 1;
        best_prod = max_geode_production(next, blueprint, best_prod);
    }
    // D. build clay robot
    if state.ore >= blueprint.clay_robot_ore_cost
        && state.clay_robot < blueprint.obsidian_robot_clay_cost
    {
        let mut next = next;
        next.ore -= blueprint.clay_robot_ore_cost;
        next.clay_robot += 1;
        best_prod = max_geode_production(next, blueprint, best_prod);
    }
    // E. build ore robot
    if state.ore >= blueprint.ore_robot_ore_cost
        && state.ore_robot
            < max(
                max(
                    blueprint.geode_robot_ore_cost,
                    blueprint.obsidian_robot_ore_cost,
                ),
                blueprint.clay_robot_ore_cost,
            )
    {
        let mut next = next;
        next.ore -= blueprint.ore_robot_ore_cost;
        next.ore_robot += 1;
        best_prod = max_geode_production(next, blueprint, best_prod);
    }

    best_prod
}

fn main() -> Result<()> {
    let re = Regex::new(
        r"Blueprint (?P<id>\d+): Each ore robot costs (?P<ore_robot_ore_cost>\d+) ore. Each clay robot costs (?P<clay_robot_ore_cost>\d+) ore. Each obsidian robot costs (?P<obsidian_robot_ore_cost>\d+) ore and (?P<obsidian_robot_clay_cost>\d+) clay. Each geode robot costs (?P<geode_robot_ore_cost>\d+) ore and (?P<geode_robot_obsidian_cost>\d+) obsidian.",
    )?;
    let mut blueprints = Vec::new();
    for line in INPUT.lines() {
        let caps = re
            .captures(line)
            .with_context(|| format!("failed to parse line: {line}"))?;
        let id = caps["id"].parse::<i32>()?;
        let blueprint = Blueprint {
            ore_robot_ore_cost: caps["ore_robot_ore_cost"].parse::<i32>()?,
            clay_robot_ore_cost: caps["clay_robot_ore_cost"].parse::<i32>()?,
            obsidian_robot_ore_cost: caps["obsidian_robot_ore_cost"].parse::<i32>()?,
            obsidian_robot_clay_cost: caps["obsidian_robot_clay_cost"].parse::<i32>()?,
            geode_robot_ore_cost: caps["geode_robot_ore_cost"].parse::<i32>()?,
            geode_robot_obsidian_cost: caps["geode_robot_obsidian_cost"].parse::<i32>()?,
        };
        blueprints.push((id, blueprint));
    }

    // part 1
    let mut sum = 0;
    for (id, blueprint) in blueprints.iter() {
        let state = State {
            time: 24,
            ore_robot: 1,
            ..Default::default()
        };
        let quality = id * max_geode_production(state, blueprint, 0);
        sum += quality;
    }
    println!("{sum}");

    // part 2
    let mut mul = 1;
    for (_, blueprint) in blueprints.into_iter().take(3) {
        let state = State {
            time: 32,
            ore_robot: 1,
            ..Default::default()
        };
        let geode = max_geode_production(state, &blueprint, 0);
        mul *= geode;
    }
    println!("{mul}");

    Ok(())
}

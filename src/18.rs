use std::collections::HashSet;

use anyhow::Result;
use itertools::Itertools;

const INPUT: &str = include_str!("../data/18.txt");

type Coord = (i32, i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Lava,
    Inside,
    Outside,
    Unknown,
}
use Cell::*;

#[derive(Debug)]
struct World {
    raw: Vec<Cell>,
    offset_x: i32,
    offset_y: i32,
    offset_z: i32,
    size_x: usize,
    size_y: usize,
    size_z: usize,
}

impl World {
    fn new(min_xyz: Coord, max_xyz: Coord) -> Self {
        assert!(min_xyz.0 <= max_xyz.0);
        assert!(min_xyz.1 <= max_xyz.1);
        assert!(min_xyz.2 <= max_xyz.2);
        let offset_x = min_xyz.0;
        let offset_y = min_xyz.1;
        let offset_z = min_xyz.2;
        let size_x = (max_xyz.0 - min_xyz.0 + 1) as usize;
        let size_y = (max_xyz.1 - min_xyz.1 + 1) as usize;
        let size_z = (max_xyz.2 - min_xyz.2 + 1) as usize;
        Self {
            raw: vec![Unknown; size_x * size_y * size_z],
            offset_x,
            offset_y,
            offset_z,
            size_x,
            size_y,
            size_z,
        }
    }

    fn get(&self, xyz: Coord) -> Option<&Cell> {
        if xyz.0 < self.offset_x
            || ((xyz.0 - self.offset_x) as usize) >= self.size_x
            || xyz.1 < self.offset_y
            || ((xyz.1 - self.offset_y) as usize) >= self.size_y
            || xyz.2 < self.offset_z
            || ((xyz.2 - self.offset_z) as usize) >= self.size_z
        {
            return None;
        }
        Some(
            &self.raw[(xyz.0 - self.offset_x) as usize * self.size_y * self.size_z
                + (xyz.1 - self.offset_y) as usize * self.size_z
                + (xyz.2 - self.offset_z) as usize],
        )
    }

    fn get_mut(&mut self, xyz: Coord) -> Option<&mut Cell> {
        if xyz.0 < self.offset_x
            || ((xyz.0 - self.offset_x) as usize) >= self.size_x
            || xyz.1 < self.offset_y
            || ((xyz.1 - self.offset_y) as usize) >= self.size_y
            || xyz.2 < self.offset_z
            || ((xyz.2 - self.offset_z) as usize) >= self.size_z
        {
            return None;
        }
        Some(
            &mut self.raw[(xyz.0 - self.offset_x) as usize * self.size_y * self.size_z
                + (xyz.1 - self.offset_y) as usize * self.size_z
                + (xyz.2 - self.offset_z) as usize],
        )
    }
}

// fill space from cube location until outside/inside is known
fn fill(world: &mut World, cube: Coord) {
    if Some(&Unknown) != world.get(cube) {
        // nothing to do
        return;
    }
    let mut to_fill = HashSet::new();
    let mut to_explore = Vec::new();
    to_explore.push(cube);
    to_fill.insert(cube);
    while let Some(xyz) = to_explore.pop() {
        let neighbors = [
            (xyz.0 - 1, xyz.1, xyz.2),
            (xyz.0, xyz.1 - 1, xyz.2),
            (xyz.0, xyz.1, xyz.2 - 1),
            (xyz.0 + 1, xyz.1, xyz.2),
            (xyz.0, xyz.1 + 1, xyz.2),
            (xyz.0, xyz.1, xyz.2 + 1),
        ];

        // propagate fill to unexplored neighbors
        for neighbor in neighbors {
            if to_fill.contains(&neighbor) {
                // already explored
                continue;
            }
            let cell = world.get(neighbor).copied();
            match cell {
                Some(Lava) => continue,
                Some(Unknown) => {
                    // need to explore
                    to_explore.push(neighbor);
                    to_fill.insert(neighbor);
                }
                Some(inside_outside) => {
                    // propagate known state on all explored cells
                    for xyz in to_fill {
                        *world.get_mut(xyz).unwrap() = inside_outside;
                    }
                    return;
                }
                None => {
                    // reached the outside of the droplet bounding box -> we are outside
                    for xyz in to_fill {
                        *world.get_mut(xyz).unwrap() = Outside;
                    }
                    return;
                }
            };
        }
    }
    // ran out of cubes to fill, yet we have not reached the outside -> we are trapped inside
    for xyz in to_fill {
        *world.get_mut(xyz).unwrap() = Inside;
    }
}

fn main() -> Result<()> {
    // part 1
    let mut droplet = Vec::new();
    for line in INPUT.lines() {
        let xyz: Coord = line
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        droplet.push(xyz);
    }

    // straight forward implementation
    let mut surface = 0;
    for xyz in droplet.iter() {
        let neighbors = [
            (xyz.0 - 1, xyz.1, xyz.2),
            (xyz.0, xyz.1 - 1, xyz.2),
            (xyz.0, xyz.1, xyz.2 - 1),
            (xyz.0 + 1, xyz.1, xyz.2),
            (xyz.0, xyz.1 + 1, xyz.2),
            (xyz.0, xyz.1, xyz.2 + 1),
        ];

        surface += neighbors
            .into_iter()
            .filter(|xyz| !droplet.contains(xyz))
            .count();
    }
    println!("{surface}");

    // part 2
    let min = (
        droplet.iter().map(|xyz| xyz.0).min().unwrap() - 1,
        droplet.iter().map(|xyz| xyz.1).min().unwrap() - 1,
        droplet.iter().map(|xyz| xyz.2).min().unwrap() - 1,
    );
    let max = (
        droplet.iter().map(|xyz| xyz.0).max().unwrap() + 1,
        droplet.iter().map(|xyz| xyz.1).max().unwrap() + 1,
        droplet.iter().map(|xyz| xyz.2).max().unwrap() + 1,
    );
    let mut world = World::new(min, max);

    for line in INPUT.lines() {
        let xyz: Coord = line
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        *world.get_mut(xyz).unwrap() = Lava;
    }

    let mut surface = 0;
    for xyz in droplet.iter() {
        let neighbors = [
            (xyz.0 - 1, xyz.1, xyz.2),
            (xyz.0, xyz.1 - 1, xyz.2),
            (xyz.0, xyz.1, xyz.2 - 1),
            (xyz.0 + 1, xyz.1, xyz.2),
            (xyz.0, xyz.1 + 1, xyz.2),
            (xyz.0, xyz.1, xyz.2 + 1),
        ];

        for neighbor in neighbors {
            fill(&mut world, neighbor);
            if let Some(Outside) = world.get(neighbor) {
                surface += 1;
            }
        }
    }
    println!("{surface}");

    Ok(())
}

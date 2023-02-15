use std::{collections::HashMap, fmt::Display};

use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Elf,
}
use Tile::*;

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Elf => write!(f, "#"),
        }
    }
}

// infinite world structure
#[derive(Debug, Clone)]
struct World {
    raw: Vec<Tile>,
    origin_x: i32,
    origin_y: i32,
    size_i: usize,
    size_j: usize,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size_i {
            for j in 0..self.size_j {
                write!(f, "{}", self.raw[i * self.size_j + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl World {
    fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        assert!(max_x >= min_x);
        assert!(max_y >= min_y);
        let size_i = (max_x - min_x) as usize + 1;
        let size_j = (max_y - min_y) as usize + 1;
        Self {
            raw: vec![Empty; size_i * size_j],
            origin_x: min_x,
            origin_y: min_y,
            size_i,
            size_j,
        }
    }

    fn get(&self, x: i32, y: i32) -> &Tile {
        let i = x - self.origin_x;
        let j = y - self.origin_y;
        if i >= 0 && (i as usize) < self.size_i && j >= 0 && (j as usize) < self.size_j {
            &self.raw[i as usize * self.size_j + j as usize]
        } else {
            &Empty
        }
    }

    fn get_mut(&mut self, x: i32, y: i32) -> &mut Tile {
        let i = x - self.origin_x;
        let j = y - self.origin_y;
        if i >= 0 && (i as usize) < self.size_i && j >= 0 && (j as usize) < self.size_j {
            &mut self.raw[i as usize * self.size_j + j as usize]
        } else {
            // resize the world to fit the new point
            let next_origin_x = self.origin_x.min(x);
            let next_origin_y = self.origin_y.min(y);
            let next_size_i = (self.size_i as i32)
                .max(x - self.origin_x + 1)
                .max((self.origin_x + self.size_i as i32) - x)
                as usize;
            let next_size_j = (self.size_j as i32)
                .max(y - self.origin_y + 1)
                .max((self.origin_y + self.size_j as i32) - y)
                as usize;
            // copy old data to the new world
            let mut next_raw = vec![Empty; next_size_i * next_size_j];
            for i in 0..self.size_i {
                let start = i * self.size_j;
                let end = start + self.size_j;
                let next_start = (i + (self.origin_x - next_origin_x) as usize) * next_size_j
                    + ((self.origin_y - next_origin_y) as usize);
                let next_end = next_start + self.size_j;
                next_raw[next_start..next_end].copy_from_slice(&self.raw[start..end]);
            }
            *self = World {
                raw: next_raw,
                origin_x: next_origin_x,
                origin_y: next_origin_y,
                size_i: next_size_i,
                size_j: next_size_j,
            };
            self.get_mut(x, y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_resize() {
        let world_str = ["###..", "#####", ".....", "#...#", "....#"];

        let mut world = World::new(0, 0, 0, 0);
        for (x, line) in world_str.iter().enumerate() {
            for (y, c) in line.chars().enumerate() {
                *world.get_mut(x as i32, y as i32) = match c {
                    '#' => Elf,
                    '.' => Empty,
                    _ => panic!("Unknown char '{c}'"),
                };
            }
        }
        *world.get_mut(7, 7) = Elf;
        *world.get_mut(10, 3) = Elf;
        *world.get_mut(-1, -1) = Elf;
        *world.get_mut(-5, 0) = Elf;
        println!("{world}");
        assert_eq!(*world.get(0, 0), Elf);
        assert_eq!(*world.get(1, 0), Elf);
        assert_eq!(*world.get(2, 0), Empty);
        assert_eq!(*world.get(3, 0), Elf);
        assert_eq!(*world.get(4, 0), Empty);
        assert_eq!(*world.get(4, 4), Elf);
        assert_eq!(*world.get(7, 7), Elf);
        assert_eq!(*world.get(10, 3), Elf);
        assert_eq!(*world.get(-1, -1), Elf);
        assert_eq!(*world.get(-5, 0), Elf);
        assert_eq!(*world.get(-1, 0), Empty);
        assert_eq!(*world.get(0, -1), Empty);
        assert_eq!(*world.get(-10, -10), Empty);
    }
}

type Coord = (i32, i32);

const NORTH: Coord = (-1, 0);
const EAST: Coord = (0, 1);
const WEST: Coord = (0, -1);
const SOUTH: Coord = (1, 0);
const NORTH_EAST: Coord = (-1, 1);
const NORTH_WEST: Coord = (-1, -1);
const SOUTH_EAST: Coord = (1, 1);
const SOUTH_WEST: Coord = (1, -1);

const ALL_DIRECTIONS: [Coord; 8] = [
    NORTH, EAST, WEST, SOUTH, NORTH_EAST, NORTH_WEST, SOUTH_EAST, SOUTH_WEST,
];
const CHECKS_ORDER: [[Coord; 3]; 4] = [
    [NORTH, NORTH_EAST, NORTH_WEST],
    [SOUTH, SOUTH_EAST, SOUTH_WEST],
    [WEST, NORTH_WEST, SOUTH_WEST],
    [EAST, NORTH_EAST, SOUTH_EAST],
];

fn move_elves(world: &mut World, k: usize) -> bool {
    // first phase - each elf may propose to move to a different tile
    let mut proposals = HashMap::<Coord, Option<Coord>>::new();
    for i in 0..world.size_i {
        let x = world.origin_x + i as i32;
        for j in 0..world.size_j {
            let y = world.origin_y + j as i32;

            if world.get(x, y) != &Elf {
                // not an elf
                continue;
            }

            if ALL_DIRECTIONS
                .into_iter()
                .all(|(dx, dy)| world.get(x + dx, y + dy) == &Empty)
            {
                // if the elf has no neighbor, don't move
                continue;
            }

            for check in CHECKS_ORDER
                .into_iter()
                .cycle()
                .skip(k % CHECKS_ORDER.len())
                .take(CHECKS_ORDER.len())
            {
                if check
                    .iter()
                    .all(|(dx, dy)| world.get(x + dx, y + dy) == &Empty)
                {
                    // the elf wants to move in that direction
                    let (dx, dy) = check[0]; // e.g. North
                    proposals
                        .entry((x + dx, y + dy))
                        .and_modify(|p| {
                            if p.is_some() {
                                // too many elves want to move here -> no one will move
                                *p = None;
                            }
                        })
                        .or_insert(Some((x, y)));
                    break;
                }
            }
        }
    }

    let finished = proposals.is_empty();

    // second phase - move elves according to each proposal
    for (target, from) in proposals
        .into_iter()
        .filter_map(|(target, from)| from.map(|from| (target, from)))
    {
        assert!(world.get(from.0, from.1) == &Elf);
        *world.get_mut(from.0, from.1) = Empty;
        *world.get_mut(target.0, target.1) = Elf;
    }

    !finished
}

const INPUT: &str = include_str!("../data/23.txt");

fn main() -> Result<()> {
    // part 1
    let initial_world = {
        // initialize empty world with approximate size
        let mut world = World::new(
            0,
            0,
            INPUT.lines().count() as i32,
            INPUT.lines().next().unwrap().len() as i32,
        );
        // fill the world
        for (x, line) in INPUT.lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                *world.get_mut(x as i32, y as i32) = match c {
                    '.' => Empty,
                    '#' => Elf,
                    _ => bail!("Unknown char '{c}'"),
                };
            }
        }
        world
    };

    // move elves for 10 rounds
    let mut world = initial_world.clone();
    for k in 0..10 {
        if !move_elves(&mut world, k) {
            break;
        }
    }

    // find bounding box
    let mut max_x = world.origin_x;
    let mut min_x = world.origin_x + world.size_i as i32;
    let mut max_y = world.origin_y;
    let mut min_y = world.origin_y + world.size_j as i32;
    let mut nb_elves = 0;
    for i in 0..world.size_i {
        let x = world.origin_x + i as i32;
        for j in 0..world.size_j {
            let y = world.origin_y + j as i32;
            if world.get(x, y) == &Elf {
                nb_elves += 1;
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                min_x = min_x.min(x);
                min_y = min_y.min(y);
            }
        }
    }
    assert!(max_x >= min_x);
    assert!(max_y >= min_y);
    let rectangle_size = (max_x - min_x + 1) * (max_y - min_y + 1);
    let nb_empty = rectangle_size - nb_elves;
    println!("{nb_empty}");

    // part 2
    let mut world = initial_world;
    let mut k = 0;
    while move_elves(&mut world, k) {
        k += 1;
    }
    let round = k + 1;
    println!("{round}");

    Ok(())
}

use std::ops::{Index, IndexMut};

use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorldCell {
    Off,
    Empty,
    Wall,
}
use WorldCell::*;

#[derive(Debug)]
struct World {
    raw: Vec<WorldCell>,
    rows: usize,
    cols: usize,
}

type Coord = (usize, usize);

impl World {
    fn new(rows: usize, cols: usize) -> Self {
        World {
            raw: vec![Off; rows * cols],
            rows,
            cols,
        }
    }
}

impl Index<usize> for World {
    type Output = [WorldCell];

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.rows);
        let start = index * self.cols;
        let end = (index + 1) * self.cols;
        &self.raw[start..end]
    }
}

impl IndexMut<usize> for World {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.rows);
        let start = index * self.cols;
        let end = (index + 1) * self.cols;
        &mut self.raw[start..end]
    }
}

impl World {
    fn get(&self, p: Coord) -> &WorldCell {
        assert!(p.0 < self.rows);
        assert!(p.1 < self.cols);
        &self[p.0][p.1]
    }
}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}
use Direction::*;

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn rotate_counter_clockwise(&self) -> Self {
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Forward(usize),
    RotateClockwise,
    RotateCounterClockwise,
}
use Instruction::*;

mod parser {
    use nom::{
        branch::alt,
        character::complete::{char, digit1},
        combinator::{complete, map, map_res},
        multi::many1,
        IResult,
    };

    use crate::Instruction::{self, *};
    use anyhow::Result;

    fn _instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        many1(alt((
            map_res(digit1, |s: &str| s.parse::<usize>().map(Forward)),
            alt((
                map(char('R'), |_| RotateClockwise),
                map(char('L'), |_| RotateCounterClockwise),
            )),
        )))(input)
    }

    pub fn instructions(input: &str) -> Result<Vec<Instruction>> {
        let (_, instructions) = complete(_instructions)(input).map_err(|e| e.to_owned())?;
        Ok(instructions)
    }
}

struct Position {
    coord: Coord,
    direction: Direction,
}

impl Position {
    fn password(&self) -> usize {
        let d = match self.direction {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        };
        1000 * (self.coord.0 + 1) + 4 * (self.coord.1 + 1) + d
    }
}

fn execute(world: &World, p: Position, instr: Instruction) -> Position {
    match instr {
        Forward(n) => {
            let step = match p.direction {
                Right => (0, 1),
                Down => (1, 0),
                Left => (0, -1),
                Up => (-1, 0),
            };
            let mut p = p;
            let mut i = 0;
            while i < n {
                // find next empty cell
                let mut next = p.coord;
                next = (
                    (next.0 as i32 + step.0).rem_euclid(world.rows as i32) as usize,
                    (next.1 as i32 + step.1).rem_euclid(world.cols as i32) as usize,
                );
                // skip off world
                while world.get(next) == &Off {
                    next = (
                        (next.0 as i32 + step.0).rem_euclid(world.rows as i32) as usize,
                        (next.1 as i32 + step.1).rem_euclid(world.cols as i32) as usize,
                    );
                }
                match world.get(next) {
                    Off => unreachable!("Should have skipped off world"),
                    Empty => {
                        // step forward
                        p.coord = next;
                        i += 1;
                    }
                    Wall => break, // cannot pass wall
                }
            }
            p
        }
        RotateClockwise => Position {
            coord: p.coord,
            direction: p.direction.rotate_clockwise(),
        },
        RotateCounterClockwise => Position {
            coord: p.coord,
            direction: p.direction.rotate_counter_clockwise(),
        },
    }
}

const INPUT: &str = include_str!("../data/22.txt");

fn main() -> Result<()> {
    // part 1
    // read once to get world dimensions
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    for line in INPUT.lines() {
        if line.is_empty() {
            break;
        }
        rows += 1;
        cols = cols.max(line.len());
    }

    // fill world
    let mut world = World::new(rows, cols);
    for (row, line) in INPUT.lines().enumerate().take(rows) {
        for (col, c) in line.chars().enumerate() {
            let cell = match c {
                ' ' => Off,
                '.' => Empty,
                '#' => Wall,
                _ => panic!("Unknown cell: {c}"),
            };
            world[row][col] = cell;
        }
    }

    // parse instructions
    let instructions = parser::instructions(
        INPUT
            .lines()
            .nth(rows + 1)
            .context("Failed to find instruction line")?,
    )?;

    // initial position
    let mut p = Position {
        coord: (0, 0),
        direction: Right,
    };
    while world[p.coord.0][p.coord.1] != Empty {
        // move right
        p.coord.1 += 1;
    }

    // execute instructions
    for instr in instructions {
        p = execute(&world, p, instr);
    }

    let password = p.password();
    println!("{password}");

    Ok(())
}

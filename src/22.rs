use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorldCell {
    Off,
    Empty,
    Wall,
}
use WorldCell::*;

impl Display for WorldCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Off => write!(f, " "),
            Empty => write!(f, "."),
            Wall => write!(f, "#"),
        }
    }
}

#[derive(Debug)]
struct World {
    raw: Vec<WorldCell>,
    rows: usize,
    cols: usize,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", self[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}
use Direction::*;

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Right => write!(f, ">"),
            Down => write!(f, "v"),
            Left => write!(f, "<"),
            Up => write!(f, "^"),
        }
    }
}

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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NeighborFace {
    right: (usize, Direction),
    down: (usize, Direction),
    left: (usize, Direction),
    up: (usize, Direction),
}

impl NeighborFace {
    fn get(&self, direction: Direction) -> (usize, Direction) {
        match direction {
            Right => self.right,
            Down => self.down,
            Left => self.left,
            Up => self.up,
        }
    }
}

// hardcoded cube folding
// TODO: find a way to compute the cube
const TILES: [[Option<usize>; 3]; 4] = [
    [None, Some(0), Some(3)],
    [None, Some(4), None],
    [Some(2), Some(5), None],
    [Some(1), None, None],
];
const FACE_COORDS: [Coord; 6] = [
    (0, 1), // 0
    (3, 0), // 1
    (2, 0), // 2
    (0, 2), // 3
    (1, 1), // 4
    (2, 1), // 5
];
// hardcoded face connections and direction after moving on the neighbor face
const NEIGHBOR_FACES: [NeighborFace; 6] = [
    // 0
    NeighborFace {
        right: (3, Right),
        down: (4, Down),
        left: (2, Right),
        up: (1, Right),
    },
    // 1
    NeighborFace {
        right: (5, Up),
        down: (3, Down),
        left: (0, Down),
        up: (2, Up),
    },
    // 2
    NeighborFace {
        right: (5, Right),
        down: (1, Down),
        left: (0, Right),
        up: (4, Right),
    },
    // 3
    NeighborFace {
        right: (5, Left),
        down: (4, Left),
        left: (0, Left),
        up: (1, Up),
    },
    // 4
    NeighborFace {
        right: (3, Up),
        down: (5, Down),
        left: (2, Down),
        up: (0, Up),
    },
    // 5
    NeighborFace {
        right: (3, Left),
        down: (1, Left),
        left: (2, Left),
        up: (4, Up),
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    impl Direction {
        fn opposite(&self) -> Direction {
            match self {
                Right => Left,
                Down => Up,
                Left => Right,
                Up => Down,
            }
        }
    }

    #[test]
    fn face_connection() {
        // make sure tile layout is correct
        for (n, (i, j)) in FACE_COORDS.iter().enumerate() {
            assert_eq!(TILES[*i][*j], Some(n));
        }

        // make sure all faces are connected from all sides
        let mut connected = [[false; 4]; 6];
        for face in NEIGHBOR_FACES {
            connected[face.right.0][face.right.1 as usize] = true;
            connected[face.down.0][face.down.1 as usize] = true;
            connected[face.left.0][face.left.1 as usize] = true;
            connected[face.up.0][face.up.1 as usize] = true;
        }
        println!("{connected:?}\n");
        assert!(connected.iter().flatten().all(|b| *b));

        // make sure faces connections are consistent
        for (id, face) in NEIGHBOR_FACES.iter().enumerate() {
            println!("{id}: {face:?}");
            println!("right: {:?}", NEIGHBOR_FACES[face.right.0]);
            assert_eq!(
                id,
                NEIGHBOR_FACES[face.right.0].get(face.right.1.opposite()).0
            );
            assert_eq!(
                Right,
                NEIGHBOR_FACES[face.right.0]
                    .get(face.right.1.opposite())
                    .1
                    .opposite()
            );
            println!("down: {:?}", NEIGHBOR_FACES[face.down.0]);
            assert_eq!(
                id,
                NEIGHBOR_FACES[face.down.0].get(face.down.1.opposite()).0
            );
            assert_eq!(
                Down,
                NEIGHBOR_FACES[face.down.0]
                    .get(face.down.1.opposite())
                    .1
                    .opposite()
            );
            println!("left: {:?}", NEIGHBOR_FACES[face.left.0]);
            assert_eq!(
                id,
                NEIGHBOR_FACES[face.left.0].get(face.left.1.opposite()).0
            );
            assert_eq!(
                Left,
                NEIGHBOR_FACES[face.left.0]
                    .get(face.left.1.opposite())
                    .1
                    .opposite()
            );
            println!("up: {:?}", NEIGHBOR_FACES[face.up.0]);
            assert_eq!(id, NEIGHBOR_FACES[face.up.0].get(face.up.1.opposite()).0);
            assert_eq!(
                Up,
                NEIGHBOR_FACES[face.up.0]
                    .get(face.up.1.opposite())
                    .1
                    .opposite()
            );
            println!();
        }
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

fn execute_cube(world: &World, p: Position, instr: Instruction) -> Position {
    let tile_size = world.rows / TILES.len();
    assert_eq!(tile_size, world.cols / TILES[0].len());
    match instr {
        Forward(n) => {
            let mut curr = p;
            for _ in 0..n {
                // find next cell
                let mut next = curr;
                let step = match curr.direction {
                    Right => (0, 1),
                    Down => (1, 0),
                    Left => (0, -1),
                    Up => (-1, 0),
                };
                next.coord = (
                    (next.coord.0 as i32 + step.0).rem_euclid(world.rows as i32) as usize,
                    (next.coord.1 as i32 + step.1).rem_euclid(world.cols as i32) as usize,
                );
                let curr_face = TILES[curr.coord.0 / tile_size][curr.coord.1 / tile_size].unwrap();
                let next_face = TILES[next.coord.0 / tile_size][next.coord.1 / tile_size];
                if next_face.is_none() || next_face.unwrap() != curr_face {
                    // would walk outside the current face
                    let (i, j) = FACE_COORDS[curr_face];
                    // find offset of position on current face edge
                    let offset = match curr.direction {
                        Right => {
                            assert_eq!((j + 1) * tile_size, curr.coord.1 + 1);
                            curr.coord.0 - i * tile_size
                        }
                        Down => {
                            assert_eq!((i + 1) * tile_size, curr.coord.0 + 1);
                            curr.coord.1 - j * tile_size
                        }
                        Left => {
                            assert_eq!(j * tile_size, curr.coord.1);
                            curr.coord.0 - i * tile_size
                        }
                        Up => {
                            assert_eq!(i * tile_size, curr.coord.0);
                            curr.coord.1 - j * tile_size
                        }
                    };
                    // find the neighbor face to land on
                    let (next_face, next_direction) = NEIGHBOR_FACES[curr_face].get(curr.direction);

                    // because of orientation change, offset may be reversed
                    let offset = match (curr.direction, next_direction) {
                        (Right, Right) => offset,
                        (Right, Down) => tile_size - offset - 1,
                        (Right, Left) => tile_size - offset - 1,
                        (Right, Up) => offset,
                        (Down, Right) => tile_size - offset - 1,
                        (Down, Down) => offset,
                        (Down, Left) => offset,
                        (Down, Up) => tile_size - offset - 1,
                        (Left, Right) => tile_size - offset - 1,
                        (Left, Down) => offset,
                        (Left, Left) => offset,
                        (Left, Up) => tile_size - offset - 1,
                        (Up, Right) => offset,
                        (Up, Down) => tile_size - offset - 1,
                        (Up, Left) => tile_size - offset - 1,
                        (Up, Up) => offset,
                    };

                    // apply offset on new face
                    let (i, j) = FACE_COORDS[next_face];
                    next.coord = match next_direction {
                        Right => (i * tile_size + offset, j * tile_size),
                        Down => (i * tile_size, j * tile_size + offset),
                        Left => (i * tile_size + offset, (j + 1) * tile_size - 1),
                        Up => ((i + 1) * tile_size - 1, j * tile_size + offset),
                    };
                    next.direction = next_direction;
                }
                // finally check if we can walk on the next cell
                match world.get(next.coord) {
                    Off => unreachable!("Should have skipped off world"),
                    Empty => {
                        curr = next;
                    }
                    Wall => break, // cannot pass wall
                };
            }
            curr
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
    let mut initial_position = Position {
        coord: (0, 0),
        direction: Right,
    };
    while world[initial_position.coord.0][initial_position.coord.1] != Empty {
        // move right
        initial_position.coord.1 += 1;
    }

    // execute instructions
    let mut p = initial_position;
    for instr in instructions.iter() {
        p = execute(&world, p, *instr);
    }

    let password = p.password();
    println!("{password}");

    // part 2
    // virtually fold the world in a cube by assigning cube faces to world blocks
    let mut p = initial_position;
    for instr in instructions {
        p = execute_cube(&world, p, instr);
    }

    let password = p.password();
    println!("{password}");

    Ok(())
}

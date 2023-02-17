use std::{ops::{Add, AddAssign, Index, IndexMut}, collections::HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(i32, i32);

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Orientation {
    Up, Right, Down, Left
}
use Orientation::*;

impl Orientation {
    fn unit(&self) -> Coord {
        match self {
            Up => Coord(-1, 0),
            Right => Coord(0, 1),
            Down => Coord(1, 0),
            Left => Coord(0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    position: Coord,
    orientation: Orientation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    // number of blizzards on the cell
    Blizzard(u8)
}
use Cell::*;

#[derive(Debug, Clone)]
struct WorldMap {
    raw: Vec<Cell>,
    rows: usize,
    cols: usize,
}

impl Index<Coord> for WorldMap {
    type Output = Cell;

    fn index(&self, p: Coord) -> &Self::Output {
        &self.raw[p.0 as usize * self.cols + p.1 as usize]
    }
}

impl IndexMut<Coord> for WorldMap {
    fn index_mut(&mut self, p: Coord) -> &mut Self::Output {
        &mut self.raw[p.0 as usize * self.cols + p.1 as usize]
    }
}

impl WorldMap {
    fn get(&self, p: Coord) -> Option<Cell> {
        if p.0 < 0 || p.1 < 0 || p.0 as usize >= self.rows || p.1 as usize >= self.cols {
            None
        } else {
            Some(self[p])
        }
    }
}

#[derive(Debug, Clone)]
struct World {
    world_map: WorldMap,
    blizzards: Vec<Blizzard>,
}

const INPUT: &str = include_str!("../data/24.txt");

impl World {
    fn step(&mut self) {
        for blizzard in self.blizzards.iter_mut() {
            // remove old blizzard position from the map
            self.world_map[blizzard.position] = match self.world_map[blizzard.position] {
                Empty | Wall => panic!("corrupted map data"),
                Blizzard(n) => {
                    if n > 1 {
                        Blizzard(n-1)
                    } else {
                        Empty
                    }
                },
            };
            // move blizzard
            blizzard.position += blizzard.orientation.unit();
            // wrap around if necessary
            if blizzard.position.0 == 0 {
                blizzard.position.0 = (self.world_map.rows - 2) as i32;
            } else if blizzard.position.1 == 0 {
                blizzard.position.1 = (self.world_map.cols - 2) as i32;
            } else if blizzard.position.0 == (self.world_map.rows - 1) as i32{
                blizzard.position.0 = 1;
            } else if blizzard.position.1 == (self.world_map.cols - 1) as i32{
                blizzard.position.1 = 1;
            }
            // add new blizzard position on the map
            self.world_map[blizzard.position] = match self.world_map[blizzard.position] {
                Empty => Blizzard(1),
                Wall => panic!("corrupted map data"),
                Blizzard(n) => Blizzard(n+1),
            };
        }
    }
}

fn get_fastest_time(world: &mut World, start: Coord, goal: Coord) -> usize {
    let mut time = 0;
    // all positions that can be reached at t
    let mut reached = HashSet::new();
    reached.insert(start);
    loop {
        time += 1;
        // update blizzards
        world.step();
        // list all reachable positions
        let mut next_reached = HashSet::new();
        for p in reached {
            for motion in [Coord(0,0), Up.unit(), Right.unit(), Down.unit(), Left.unit()] {
                let next_p = p + motion;
                // check if we could move to the new position
                if let Some(Empty) = world.world_map.get(next_p) {
                    if next_p == goal {
                        // reached the goal!
                        return time;
                    }
                    next_reached.insert(next_p);
                }
            }
        }
        reached = next_reached;
    }
}

fn main() {
    let mut rows = 0;
    let mut cols = 0;
    let mut raw_world_map = Vec::new();
    let mut blizzards = Vec::new();
    for (i, line) in INPUT.lines().enumerate() {
        rows += 1;
        let mut col = 0;
        for (j, char) in line.chars().enumerate() {
            col += 1;
            // update world map
            let cell = match char {
                '#' => Wall,
                '.' => Empty,
                '^' => Blizzard(1), 
                '>' => Blizzard(1), 
                'v' => Blizzard(1), 
                '<' => Blizzard(1), 
                _ => panic!("Unexpected character: '{char}'"),
            };
            raw_world_map.push(cell);
            // update blizzards
            match char {
                '^' => blizzards.push(Blizzard {position: Coord(i as i32, j as i32), orientation: Up }),
                '>' => blizzards.push(Blizzard {position: Coord(i as i32, j as i32), orientation: Right }),
                'v' => blizzards.push(Blizzard {position: Coord(i as i32, j as i32), orientation: Down }),
                '<' => blizzards.push(Blizzard {position: Coord(i as i32, j as i32), orientation: Left }),
                _ => ()
            }
        }
        if cols != 0 {
            assert!(cols == col);
        } else {
            cols = col;
        }
    }

    let mut world = World {
        world_map: WorldMap {
            raw: raw_world_map,
            rows,
            cols
        },
        blizzards,
    };

    let entrance = Coord(0,1);
    let exit = Coord((rows-1) as i32, (cols-2) as i32);

    // part 1
    let mut best_time = get_fastest_time(&mut world, entrance, exit);
    println!("{best_time}");

    // part 2 
    // go back to entrance, then go to exit gain
    best_time += get_fastest_time(&mut world, exit, entrance);
    best_time += get_fastest_time(&mut world, entrance, exit);
    println!("{best_time}");
}
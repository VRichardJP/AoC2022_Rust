use itertools::Itertools;
use std::{
    collections::{
        hash_map::{DefaultHasher, Entry},
        HashMap,
    },
    fmt::Display,
    hash::{Hash, Hasher},
    ops::{Add, Index, IndexMut, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord(i32, i32);

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Coord {
    fn dist(a: Coord, b: Coord) -> i32 {
        (b.0 - a.0).abs() + (b.1 - a.1).abs()
    }

    fn wrap(&self, rect: Coord) -> Self {
        Coord(self.0.rem_euclid(rect.0), self.1.rem_euclid(rect.1))
    }

    fn get_direction(&self) -> Option<Direction> {
        if self.0 < 0 && self.1 == 0 {
            Some(Up)
        } else if self.0 == 0 && self.1 > 0 {
            Some(Right)
        } else if self.0 > 0 && self.1 == 0 {
            Some(Down)
        } else if self.0 == 0 && self.1 < 0 {
            Some(Left)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}
use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AllDirections<T>([T; 4]);

impl<T> Index<Direction> for AllDirections<T> {
    type Output = T;

    fn index(&self, index: Direction) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<Direction> for AllDirections<T> {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

const UNIT_DIRECTIONS: AllDirections<Coord> = AllDirections([
    Coord(-1, 0), // up
    Coord(0, 1),  // right
    Coord(1, 0),  // down
    Coord(0, -1), // left
]);

#[derive(Clone, Copy, Hash)]
struct Blizzard {
    coord: Coord,
    direction: Direction,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Wall,
    Empty,
    Expedition, // for printing only, otherwise behave the same as Empty
    Blizzards(AllDirections<u8>),
}
use Cell::*;

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Empty => write!(f, "."),
            Wall => write!(f, "#"),
            Expedition => write!(f, "E"),
            Blizzards(all_directions) => {
                let count: u8 = all_directions.0.iter().sum();
                if count > 1 {
                    write!(f, "{count}")
                } else {
                    if all_directions[Up] > 0 {
                        write!(f, "^")
                    } else if all_directions[Right] > 0 {
                        write!(f, ">")
                    } else if all_directions[Down] > 0 {
                        write!(f, "v")
                    } else if all_directions[Left] > 0 {
                        write!(f, "<")
                    } else {
                        panic!("found blizzard cell with no blizzard")
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Wait,
    Move(Direction),
}
use Action::*;

#[derive(Clone, Hash)]
struct WorldMap {
    raw: Vec<Cell>,
    rows: usize,
    cols: usize,
}

impl Display for WorldMap {
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

impl Index<usize> for WorldMap {
    type Output = [Cell];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.cols;
        let end = start + self.cols;
        &self.raw[start..end]
    }
}

impl IndexMut<usize> for WorldMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.cols;
        let end = start + self.cols;
        &mut self.raw[start..end]
    }
}

impl WorldMap {
    fn new(raw: Vec<Cell>, rows: usize, cols: usize) -> Self {
        Self { raw, rows, cols }
    }

    fn get(&self, coord: Coord) -> Option<&Cell> {
        if coord.0 < 0
            || coord.0 as usize >= self.rows
            || coord.1 < 0
            || coord.1 as usize >= self.cols
        {
            None
        } else {
            Some(&self[coord.0 as usize][coord.1 as usize])
        }
    }

    fn get_mut(&mut self, coord: Coord) -> Option<&mut Cell> {
        if coord.0 < 0
            || coord.0 as usize >= self.rows
            || coord.1 < 0
            || coord.1 as usize >= self.cols
        {
            None
        } else {
            Some(&mut self[coord.0 as usize][coord.1 as usize])
        }
    }
}

#[derive(Clone, Hash)]
struct World {
    world_map: WorldMap,
    blizzards: Vec<Blizzard>,
}

impl World {
    fn new(raw_map: Vec<Cell>, blizzards: Vec<Blizzard>, rows: usize, cols: usize) -> Self {
        let world_map = WorldMap::new(raw_map, rows, cols);
        Self {
            world_map,
            blizzards,
        }
    }

    /// update position of all blizzards
    fn step_blizzard(&mut self) {
        let world_boundaries = Coord(self.world_map.rows as i32, self.world_map.cols as i32);
        for blizzard in self.blizzards.iter_mut() {
            // remove blizzard from old cell
            let cell = self.world_map.get_mut(blizzard.coord).unwrap();
            match cell {
                Blizzards(directions) => {
                    assert!(directions[blizzard.direction] > 0);
                    directions[blizzard.direction] -= 1;
                    if directions.0.iter().sum::<u8>() == 0 {
                        // no blizzard left on the cell
                        *cell = Empty;
                    }
                }
                _ => panic!("no blizzard on cell"),
            }
            // get next blizzard position
            let mut next_coord = blizzard.coord;
            loop {
                // keep going and wrap around world
                next_coord =
                    (next_coord + UNIT_DIRECTIONS[blizzard.direction]).wrap(world_boundaries);
                if self.world_map.get(next_coord).unwrap() != &Wall {
                    break;
                }
            }
            // move blizzard to the new cell
            blizzard.coord = next_coord;
            let cell = self.world_map.get_mut(blizzard.coord).unwrap();
            match cell {
                Empty | Expedition => {
                    let mut all_directions = AllDirections([0; 4]);
                    all_directions[blizzard.direction] = 1;
                    *cell = Blizzards(all_directions);
                }
                Blizzards(all_directions) => {
                    all_directions[blizzard.direction] += 1;
                }
                Wall => panic!("blizzard hit the wall"),
            }
        }
    }

    /// list possible actions for the expedition on the current round
    fn possible_actions(&self, expedition: Coord) -> Vec<Action> {
        let mut actions = Vec::new();
        // can wait if no blizzard hits the expedition cell
        match self.world_map.get(expedition).unwrap() {
            Empty | Expedition => actions.push(Wait),
            Blizzards(_) => (),
            Wall => panic!("expedition is on a wall!"),
        }
        // check all other directions
        for direction in [Up, Right, Down, Left] {
            let next = expedition + UNIT_DIRECTIONS[direction];
            if let Some(&Empty) = self.world_map.get(next) {
                actions.push(Move(direction));
            }
        }
        actions
    }
}

fn compute_hash(world: &World, expedition: &Coord) -> u64 {
    let mut hasher = DefaultHasher::new();
    world.hash(&mut hasher);
    expedition.hash(&mut hasher);
    hasher.finish()
}

/// ad-hoc A* state
struct SearchState {
    world: World,
    expedition: Coord,
    time: usize,
    #[cfg(debug_assertions)]
    path: Vec<Coord>, // for debug only
}

/// returns the fastest path using A* depth-first path search
fn search_fastest_path(world: World, start: Coord, goal: Coord) -> Option<SearchState> {
    let init_state = SearchState {
        world,
        expedition: start,
        time: 0,
        #[cfg(debug_assertions)]
        path: vec![start],
    };

    let mut best: Option<SearchState> = None;
    let mut to_explore = vec![init_state];
    let mut explored_best = HashMap::new();
    while let Some(mut state) = to_explore.pop() {
        #[cfg(debug_assertions)]
        {
            assert!(state.world.world_map.get(state.expedition).unwrap() == &Empty);
            *state.world.world_map.get_mut(state.expedition).unwrap() = Expedition; // tmp
            println!();
            println!("Time: {}, path: {:?}", state.time, state.path);
            print!("{}", state.world.world_map);
            *state.world.world_map.get_mut(state.expedition).unwrap() = Empty;
        }

        if state.expedition == goal {
            #[cfg(debug_assertions)]
            println!("Reached the goal!");
            if best.is_some() {
                if best.as_ref().unwrap().time > state.time {
                    best = Some(state);
                }
            } else {
                best = Some(state);
            }
            continue;
        }

        // check if state is worth exploring
        let path_min_time = state.time + Coord::dist(state.expedition, goal) as usize;
        if best.is_some() && best.as_ref().unwrap().time <= path_min_time {
            #[cfg(debug_assertions)]
            println!(
                "Not worth exploring. Path min time: {}, current best time: {}",
                path_min_time,
                best.as_ref().unwrap().time
            );
            continue;
        }

        // step simulation forward
        state.world.step_blizzard();
        let next_time = state.time + 1;

        // list possible actions for current expedition
        let possible_actions = state.world.possible_actions(state.expedition);

        let mut next_expeditions = possible_actions
            .into_iter()
            .map(|action| match action {
                Wait => state.expedition,
                Move(direction) => state.expedition + UNIT_DIRECTIONS[direction],
            })
            .collect_vec();

        // explore first expeditions that are closest from goal fist
        next_expeditions.sort_by_key(|expedition| -Coord::dist(*expedition, goal));
        for next_expedition in next_expeditions {
            // only explore new states that are worth exploring
            let hash = compute_hash(&state.world, &next_expedition);
            let explore = match explored_best.entry(hash) {
                Entry::Occupied(mut e) => {
                    if *e.get() > next_time {
                        // current path is faster than previously investigated one
                        e.insert(next_time);
                        true
                    } else {
                        #[cfg(debug_assertions)]
                        println!(
                            "Skip loop: {:?}. Current time: {}, previous best: {}",
                            (next_expedition - state.expedition)
                                .get_direction()
                                .map_or(Wait, |direction| Move(direction)),
                            next_time,
                            *e.get(),
                        );
                        false
                    }
                }
                Entry::Vacant(e) => {
                    // never reached that state before
                    e.insert(next_time);
                    true
                }
            };
            if explore {
                #[cfg(debug_assertions)]
                {
                    println!(
                        "Will explore: {:?}",
                        (next_expedition - state.expedition)
                            .get_direction()
                            .map_or(Wait, |direction| Move(direction))
                    );
                    let mut next_path = state.path.clone();
                    next_path.push(next_expedition);
                    to_explore.push(SearchState {
                        world: state.world.clone(),
                        expedition: next_expedition,
                        time: next_time,
                        path: next_path,
                    });
                }
                #[cfg(not(debug_assertions))]
                {
                    to_explore.push(SearchState {
                        world: state.world.clone(),
                        expedition: next_expedition,
                        time: next_time,
                    });
                }
            }
        }
    }
    best
}

const INPUT: &str = include_str!("../data/24.txt");

fn main() {
    // parse input
    let mut rows = 0;
    let mut cols = None;
    let mut raw_world = Vec::new();
    let mut blizzards = Vec::new();
    for (i, line) in INPUT.lines().enumerate() {
        rows += 1;
        let chars = line.chars();
        let mut col_count = 0;
        for (j, c) in chars.enumerate() {
            col_count += 1;
            // update world map
            let cell = match c {
                '#' => Wall,
                '.' => Empty,
                '^' => {
                    let mut all_directions = AllDirections([0; 4]);
                    all_directions[Up] = 1;
                    Blizzards(all_directions)
                }
                '>' => {
                    let mut all_directions = AllDirections([0; 4]);
                    all_directions[Right] = 1;
                    Blizzards(all_directions)
                }
                'v' => {
                    let mut all_directions = AllDirections([0; 4]);
                    all_directions[Down] = 1;
                    Blizzards(all_directions)
                }
                '<' => {
                    let mut all_directions = AllDirections([0; 4]);
                    all_directions[Left] = 1;
                    Blizzards(all_directions)
                }
                _ => panic!("Invalid world map character: '{c}'"),
            };
            raw_world.push(cell);
            // update blizzards
            match c {
                '^' => blizzards.push(Blizzard {
                    coord: Coord(i as i32, j as i32),
                    direction: Up,
                }),
                '>' => blizzards.push(Blizzard {
                    coord: Coord(i as i32, j as i32),
                    direction: Right,
                }),
                'v' => blizzards.push(Blizzard {
                    coord: Coord(i as i32, j as i32),
                    direction: Down,
                }),
                '<' => blizzards.push(Blizzard {
                    coord: Coord(i as i32, j as i32),
                    direction: Left,
                }),
                _ => (),
            };
        }
        if let Some(cols) = cols {
            assert!(cols == col_count);
        } else {
            cols = Some(col_count);
        }
    }
    let cols = cols.unwrap();
    let world = World::new(raw_world, blizzards, rows, cols);

    let entrance = Coord(0, 1);
    let exit = Coord(
        (world.world_map.rows - 1) as i32,
        (world.world_map.cols - 2) as i32,
    );

    // part 1
    // find fastest path from entrance to exit
    let fastest_path = search_fastest_path(world.clone(), entrance, exit).unwrap();
    println!("{}", fastest_path.time);

    // part 2
    // go from entrance to exit, then back to entrance, then back to exit again
    let mut total_time = 0;
    let state = search_fastest_path(world, entrance, exit).unwrap();
    total_time += state.time;
    let state = search_fastest_path(state.world, exit, entrance).unwrap();
    total_time += state.time;
    let state = search_fastest_path(state.world, entrance, exit).unwrap();
    total_time += state.time;
    println!("{total_time}");
}

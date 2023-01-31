use anyhow::Result;
use itertools::Itertools;
use std::{
    cmp::{max, min},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum WorldCell {
    Air,
    Rock,
    Sand,
}
use WorldCell::*;

impl Display for WorldCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Air => '.',
            Rock => '#',
            Sand => 'o',
        };
        write!(f, "{}", c)
    }
}

struct World {
    raw_data: Vec<WorldCell>,
    offset_x: i32,
    offset_y: i32,
    size_x: usize,
    size_y: usize,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size_y {
            for j in 0..self.size_x {
                write!(f, "{}", self.raw_data[i * self.size_x + j])?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

impl World {
    fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        assert!(min_x < max_x);
        assert!(min_y < max_y);
        let size_x = (max_x - min_x + 1) as usize;
        let size_y = (max_y - min_y + 1) as usize;
        World {
            raw_data: vec![Air; size_x * size_y],
            offset_x: min_x,
            offset_y: min_y,
            size_x,
            size_y,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&WorldCell> {
        if x < self.offset_x
            || y < self.offset_y
            || x >= self.offset_x + self.size_x as i32
            || y >= self.offset_y + self.size_y as i32
        {
            return None;
        }
        let i = (y - self.offset_y) as usize;
        let j = (x - self.offset_x) as usize;
        self.raw_data.get(i * self.size_x + j)
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut WorldCell> {
        if x < self.offset_x || y < self.offset_y {
            return None;
        }
        let i = (y - self.offset_y) as usize;
        let j = (x - self.offset_x) as usize;
        self.raw_data.get_mut(i * self.size_x + j)
    }

    fn drop_sand(&mut self, x: i32, y: i32) -> Option<(i32, i32)> {
        let (mut x, mut y) = (x, y);
        match self.get(x, y) {
            None => return None,
            Some(Air) => (),
            Some(Rock) | Some(Sand) => return None,
        }
        loop {
            // try down
            let down = (x, y + 1);
            match self.get(down.0, down.1) {
                None => return None, // fell off the world
                Some(Air) => {
                    // fall down
                    (x, y) = down;
                    continue;
                }
                // blocked
                _ => (),
            }

            // try down left
            let down_left = (x - 1, y + 1);
            match self.get(down_left.0, down_left.1) {
                None => return None, // fell off the world
                Some(Air) => {
                    // fall down left
                    (x, y) = down_left;
                    continue;
                }
                // blocked
                _ => (),
            }

            // try down right
            let down_right = (x + 1, y + 1);
            match self.get(down_right.0, down_right.1) {
                None => return None, // fell off the world
                Some(Air) => {
                    // fall down right
                    (x, y) = down_right;
                    continue;
                }
                // blocked
                _ => (),
            }

            // stopped falling
            *self.get_mut(x, y).unwrap() = Sand;
            return Some((x, y));
        }
    }
}

fn main() -> Result<()> {
    // part 1
    let file = File::open("data/14.txt")?;
    let mut polygons = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        let polygon = line
            .split(" -> ")
            .map(|s| s.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .collect::<Vec<(i32, i32)>>();
        polygons.push(polygon);
    }

    // find world boundaries
    let min_x = polygons
        .iter()
        .flatten()
        .map(|(x, _)| x)
        .min()
        .unwrap()
        .to_owned();
    let min_y = polygons
        .iter()
        .flatten()
        .map(|(_, y)| y)
        .min()
        .unwrap()
        .to_owned();
    let max_x = polygons
        .iter()
        .flatten()
        .map(|(x, _)| x)
        .max()
        .unwrap()
        .to_owned();
    let max_y = polygons
        .iter()
        .flatten()
        .map(|(_, y)| y)
        .max()
        .unwrap()
        .to_owned();

    // make sure drop point in inside
    let min_y = min(0, min_y);
    let max_y = max(0, max_y);
    let min_x = min(500, min_x);
    let max_x = max(500, max_x);

    let mut world_map = World::new(min_x, min_y, max_x, max_y);

    // draw rocks
    for polygon in polygons {
        for (p0, p1) in polygon.iter().tuple_windows() {
            if p0.0 != p1.0 {
                assert!(p0.1 == p1.1);
                let y = p0.1;
                let start = min(p0.0, p1.0);
                let last = max(p0.0, p1.0);
                for x in start..=last {
                    let cell = world_map.get_mut(x, y).unwrap();
                    *cell = Rock;
                }
            } else {
                assert!(p0.0 == p1.0);
                let x = p0.0;
                let start = min(p0.1, p1.1);
                let last = max(p0.1, p1.1);
                for y in start..=last {
                    let cell = world_map.get_mut(x, y).unwrap();
                    *cell = Rock;
                }
            }
        }
    }

    // simulate sand falling
    let mut count = 0;
    while world_map.drop_sand(500, 0).is_some() {
        count += 1;
    }

    println!("{count}");

    let file = File::open("data/14.txt")?;
    let mut polygons = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        let polygon = line
            .split(" -> ")
            .map(|s| s.split_once(',').unwrap())
            .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
            .collect::<Vec<(i32, i32)>>();
        polygons.push(polygon);
    }

    // find world boundaries
    let min_x = polygons
        .iter()
        .flatten()
        .map(|(x, _)| x)
        .min()
        .unwrap()
        .to_owned();
    let min_y = polygons
        .iter()
        .flatten()
        .map(|(_, y)| y)
        .min()
        .unwrap()
        .to_owned();
    let max_x = polygons
        .iter()
        .flatten()
        .map(|(x, _)| x)
        .max()
        .unwrap()
        .to_owned();
    let max_y = polygons
        .iter()
        .flatten()
        .map(|(_, y)| y)
        .max()
        .unwrap()
        .to_owned();

    // make sure drop point in inside
    let min_y = min(0, min_y);
    let max_y = max(0, max_y);
    let min_x = min(500, min_x);
    let max_x = max(500, max_x);

    // add large enough bedrock
    let min_x = min_x - 500;
    let max_x = max_x + 500;
    let max_y = max_y + 2;
    polygons.push(vec![(min_x, max_y), (max_x, max_y)]);

    let mut world_map = World::new(min_x, min_y, max_x, max_y);

    // draw rocks
    for polygon in polygons {
        for (p0, p1) in polygon.iter().tuple_windows() {
            if p0.0 != p1.0 {
                assert!(p0.1 == p1.1);
                let y = p0.1;
                let start = min(p0.0, p1.0);
                let last = max(p0.0, p1.0);
                for x in start..=last {
                    let cell = world_map.get_mut(x, y).unwrap();
                    *cell = Rock;
                }
            } else {
                assert!(p0.0 == p1.0);
                let x = p0.0;
                let start = min(p0.1, p1.1);
                let last = max(p0.1, p1.1);
                for y in start..=last {
                    let cell = world_map.get_mut(x, y).unwrap();
                    *cell = Rock;
                }
            }
        }
    }

    // simulate sand falling
    let mut count = 0;
    while world_map.drop_sand(500, 0).is_some() {
        count += 1;
    }

    println!("{count}");

    Ok(())
}

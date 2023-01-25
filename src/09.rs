use anyhow::anyhow;
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
use Direction::*;

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'U' => Ok(UP),
            'D' => Ok(DOWN),
            'L' => Ok(LEFT),
            'R' => Ok(RIGHT),
            c => Err(anyhow!("not a valid direction: {}", c)),
        }
    }
}

#[derive(Debug)]
struct Rope<const N: usize> {
    knots: [(i32, i32); N],
}

impl<const N: usize> Default for Rope<N> {
    fn default() -> Self {
        Self { knots: [(0, 0); N] }
    }
}

impl<const N: usize> Rope<N> {
    fn move_head(&mut self, direction: Direction) {
        // move head
        match direction {
            UP => self.knots[0].0 -= 1,
            DOWN => self.knots[0].0 += 1,
            LEFT => self.knots[0].1 -= 1,
            RIGHT => self.knots[0].1 += 1,
        };
        // propagate to tail
        for i in 1..N {
            let diff = (
                self.knots[i - 1].0 - self.knots[i].0,
                self.knots[i - 1].1 - self.knots[i].1,
            );
            if diff.0.abs() <= 1 && diff.1.abs() <= 1 {
                break; // no more propagation
            }
            let motion = (diff.0.clamp(-1, 1), diff.1.clamp(-1, 1));
            self.knots[i].0 += motion.0;
            self.knots[i].1 += motion.1;
        }
    }

    fn get_tail(&self) -> (i32, i32) {
        *self.knots.last().unwrap()
    }
}

fn main() -> Result<()> {
    // part 1
    let file = File::open("data/09.txt")?;
    let mut rope = Rope::<2>::default();
    let mut visited = HashSet::<(i32, i32)>::new();
    visited.insert(rope.get_tail());
    for line in BufReader::new(file).lines() {
        let line = line?;
        let (direction, steps) = line
            .split_once(' ')
            .with_context(|| format!("invalid line format: {line}"))?;
        let direction =
            Direction::try_from(direction.chars().nth(0).context("empty direction string")?)?;
        let steps = steps.parse::<u32>()?;
        for _ in 0..steps {
            rope.move_head(direction);
            visited.insert(rope.get_tail());
        }
    }
    println!("{}", visited.len());

    // part 2
    let file = File::open("data/09.txt")?;
    let mut rope = Rope::<10>::default();
    let mut visited = HashSet::<(i32, i32)>::new();
    visited.insert(rope.get_tail());
    for line in BufReader::new(file).lines() {
        let line = line?;
        let (direction, steps) = line
            .split_once(' ')
            .with_context(|| format!("invalid line format: {line}"))?;
        let direction =
            Direction::try_from(direction.chars().nth(0).context("empty direction string")?)?;
        let steps = steps.parse::<u32>()?;
        for _ in 0..steps {
            rope.move_head(direction);
            visited.insert(rope.get_tail());
        }
    }
    println!("{}", visited.len());

    Ok(())
}

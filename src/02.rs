use anyhow::{anyhow, bail, Result};
use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    ROCK = 0,
    PAPER = 1,
    SCISSOR = 2,
}
use Shape::*;

impl TryFrom<i32> for Shape {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(ROCK),
            1 => Ok(PAPER),
            2 => Ok(SCISSOR),
            v => Err(anyhow!("Invalid value: {}", v)),
        }
    }
}

impl Shape {
    fn score(self) -> i32 {
        match self {
            ROCK => 1,
            PAPER => 2,
            SCISSOR => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RoundResult {
    LOSS,
    DRAW,
    WIN,
}
use RoundResult::*;

impl RoundResult {
    fn score(self) -> i32 {
        match self {
            LOSS => 0,
            DRAW => 3,
            WIN => 6,
        }
    }
}

fn get_round_result(me: Shape, opponent: Shape) -> RoundResult {
    const ROUND_RESULT: [[RoundResult; 3]; 3] =
        [[DRAW, LOSS, WIN], [WIN, DRAW, LOSS], [LOSS, WIN, DRAW]];
    ROUND_RESULT[me as usize][opponent as usize]
}

fn main() -> Result<()> {
    // part 1
    let file = File::open("data/02.txt")?;
    let mut sum = 0;
    for line in BufReader::new(file).lines() {
        let chars: Vec<char> = line?.chars().collect();
        let opponent = match chars[0] {
            'A' => ROCK,
            'B' => PAPER,
            'C' => SCISSOR,
            c => bail!("Unexpected character {}", c),
        };

        let me = match chars[2] {
            'X' => ROCK,
            'Y' => PAPER,
            'Z' => SCISSOR,
            c => bail!("Unexpected character {}", c),
        };

        sum += me.score() + get_round_result(me, opponent).score();
    }
    println!("{}", sum);

    // part 2
    let file = File::open("data/02.txt")?;
    let mut sum = 0;
    for line in BufReader::new(file).lines() {
        let chars: Vec<char> = line?.chars().collect();
        let opponent = match chars[0] {
            'A' => ROCK,
            'B' => PAPER,
            'C' => SCISSOR,
            c => bail!("Unexpected character {}", c),
        };

        let outcome = match chars[2] {
            'X' => LOSS,
            'Y' => DRAW,
            'Z' => WIN,
            c => bail!("Unexpected character {}", c),
        };

        let me = match outcome {
            LOSS => Shape::try_from(((opponent as i32) - 1).rem_euclid(3))?,
            DRAW => opponent,
            WIN => Shape::try_from(((opponent as i32) + 1).rem_euclid(3))?,
        };

        sum += me.score() + get_round_result(me, opponent).score();
    }
    println!("{}", sum);

    Ok(())
}

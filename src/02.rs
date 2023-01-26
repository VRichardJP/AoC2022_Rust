use anyhow::{anyhow, bail, Result};
use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Rock = 0,
    Paper = 1,
    Scissor = 2,
}
use Shape::*;

impl TryFrom<i32> for Shape {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Rock),
            1 => Ok(Paper),
            2 => Ok(Scissor),
            v => Err(anyhow!("Invalid value: {}", v)),
        }
    }
}

impl Shape {
    fn score(self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RoundResult {
    Loss,
    Draw,
    Win,
}
use RoundResult::*;

impl RoundResult {
    fn score(self) -> i32 {
        match self {
            Loss => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

fn get_round_result(me: Shape, opponent: Shape) -> RoundResult {
    const ROUND_RESULT: [[RoundResult; 3]; 3] =
        [[Draw, Loss, Win], [Win, Draw, Loss], [Loss, Win, Draw]];
    ROUND_RESULT[me as usize][opponent as usize]
}

fn main() -> Result<()> {
    // part 1
    let file = File::open("data/02.txt")?;
    let mut sum = 0;
    for line in BufReader::new(file).lines() {
        let chars: Vec<char> = line?.chars().collect();
        let opponent = match chars[0] {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissor,
            c => bail!("Unexpected character {}", c),
        };

        let me = match chars[2] {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissor,
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
            'A' => Rock,
            'B' => Paper,
            'C' => Scissor,
            c => bail!("Unexpected character {}", c),
        };

        let outcome = match chars[2] {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            c => bail!("Unexpected character {}", c),
        };

        let me = match outcome {
            Loss => Shape::try_from(((opponent as i32) - 1).rem_euclid(3))?,
            Draw => opponent,
            Win => Shape::try_from(((opponent as i32) + 1).rem_euclid(3))?,
        };

        sum += me.score() + get_round_result(me, opponent).score();
    }
    println!("{}", sum);

    Ok(())
}

use anyhow::{Context, Result};
use regex::Regex;

const INPUT: &str = include_str!("../data/05.txt");

fn main() -> Result<()> {
    // Initial setup:
    //                 [B] [L]     [J]
    //             [B] [Q] [R]     [D] [T]
    //             [G] [H] [H] [M] [N] [F]
    //         [J] [N] [D] [F] [J] [H] [B]
    //     [Q] [F] [W] [S] [V] [N] [F] [N]
    // [W] [N] [H] [M] [L] [B] [R] [T] [Q]
    // [L] [T] [C] [R] [R] [J] [W] [Z] [L]
    // [S] [J] [S] [T] [T] [M] [D] [B] [H]
    //  1   2   3   4   5   6   7   8   9

    // part 1
    let mut stacks = vec![
        vec!['S', 'L', 'W'],
        vec!['J', 'T', 'N', 'Q'],
        vec!['S', 'C', 'H', 'F', 'J'],
        vec!['T', 'R', 'M', 'W', 'N', 'G', 'B'],
        vec!['T', 'R', 'L', 'S', 'D', 'H', 'Q', 'B'],
        vec!['M', 'J', 'B', 'V', 'F', 'H', 'R', 'L'],
        vec!['D', 'W', 'R', 'N', 'J', 'M'],
        vec!['B', 'Z', 'T', 'F', 'H', 'N', 'D', 'J'],
        vec!['H', 'L', 'Q', 'N', 'B', 'F', 'T'],
    ];
    let re = Regex::new(r"move (?P<move>\d*) from (?P<from>\d) to (?P<to>\d)")?;
    for line in INPUT.lines() {
        let caps = re
            .captures(line)
            .with_context(|| format!("Failed to parse line: {}", &line))?;
        let mut nb = caps["move"].parse::<i32>()?;
        let from = (caps["from"].parse::<i32>()? - 1) as usize;
        let to = (caps["to"].parse::<i32>()? - 1) as usize;
        while nb > 0 {
            let e = stacks[from].pop().context("Empty stack")?;
            stacks[to].push(e);
            nb -= 1;
        }
    }
    let top = stacks.iter().map(|v| v.last().unwrap()).collect::<String>();
    println!("{top}");

    // part 2
    let mut stacks = vec![
        vec!['S', 'L', 'W'],
        vec!['J', 'T', 'N', 'Q'],
        vec!['S', 'C', 'H', 'F', 'J'],
        vec!['T', 'R', 'M', 'W', 'N', 'G', 'B'],
        vec!['T', 'R', 'L', 'S', 'D', 'H', 'Q', 'B'],
        vec!['M', 'J', 'B', 'V', 'F', 'H', 'R', 'L'],
        vec!['D', 'W', 'R', 'N', 'J', 'M'],
        vec!['B', 'Z', 'T', 'F', 'H', 'N', 'D', 'J'],
        vec!['H', 'L', 'Q', 'N', 'B', 'F', 'T'],
    ];
    let re = Regex::new(r"move (?P<move>\d*) from (?P<from>\d) to (?P<to>\d)")?;
    for line in INPUT.lines() {
        let caps = re
            .captures(line)
            .with_context(|| format!("Failed to parse line: {}", &line))?;
        let nb = caps["move"].parse::<i32>()?;
        let from = (caps["from"].parse::<i32>()? - 1) as usize;
        let to = (caps["to"].parse::<i32>()? - 1) as usize;
        let (stack_from, moved) = stacks[from].split_at((stacks[from].len() as i32 - nb) as usize);
        let stack_from = stack_from.to_vec();
        let moved = moved.to_vec();
        stacks[from] = stack_from;
        stacks[to].extend(moved);
    }
    let top = stacks.iter().map(|v| v.last().unwrap()).collect::<String>();
    println!("{top}");

    Ok(())
}

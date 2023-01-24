use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    // part 1
    let file = File::open("data/04.txt")?;
    let mut count: i32 = 0;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let (first, second) = line
            .split_once(',')
            .with_context(|| format!("Invalid line format: {}", &line))?;
        let (first_start, first_end) = first
            .split_once('-')
            .with_context(|| format!("Invalid task format: {}", &first))?;
        let (second_start, second_end) = second
            .split_once('-')
            .with_context(|| format!("Invalid task format: {}", &second))?;

        let first_start = first_start.parse::<i32>()?;
        let first_end = first_end.parse::<i32>()?;
        let second_start = second_start.parse::<i32>()?;
        let second_end = second_end.parse::<i32>()?;

        if (first_start <= second_start && second_end <= first_end)
            || (second_start <= first_start && first_end <= second_end)
        {
            count += 1;
        }
    }
    println!("{}", count);

    // part 2
    let file = File::open("data/04.txt")?;
    let mut count: i32 = 0;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let (first, second) = line
            .split_once(',')
            .with_context(|| format!("Invalid line format: {}", &line))?;
        let (first_start, first_end) = first
            .split_once('-')
            .with_context(|| format!("Invalid task format: {}", &first))?;
        let (second_start, second_end) = second
            .split_once('-')
            .with_context(|| format!("Invalid task format: {}", &second))?;

        let first_start = first_start.parse::<i32>()?;
        let first_end = first_end.parse::<i32>()?;
        let second_start = second_start.parse::<i32>()?;
        let second_end = second_end.parse::<i32>()?;

        if !((first_start < second_start && first_end < second_start)
            || (second_start < first_start && second_end < first_start))
        {
            count += 1;
        }
    }
    println!("{}", count);

    Ok(())
}

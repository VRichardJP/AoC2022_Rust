use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    // part 1
    let file = File::open("data/01.txt")?;
    let mut max = 0;
    let mut curr = 0;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if line.is_empty() {
            max = std::cmp::max(max, curr);
            curr = 0;
        } else {
            curr += line.parse::<i32>()?;
        }
    }
    println!("{}", max);

    // part 2
    let file = File::open("data/01.txt")?;
    let mut top3 = [0; 3];
    let mut curr = 0;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if line.is_empty() {
            let mut tmp = top3.to_vec();
            tmp.push(curr);
            tmp.sort_by(|a, b| b.partial_cmp(a).unwrap());
            top3 = tmp[0..3].try_into()?;
            curr = 0;
        } else {
            curr += line.parse::<i32>()?;
        }
    }
    let sum: i32 = top3.iter().sum();
    println!("{}", sum);

    Ok(())
}

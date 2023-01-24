use anyhow::{Context, Result};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    // part 1
    let file = File::open("data/03.txt")?;
    let mut sum: i32 = 0;
    'outer: for line in BufReader::new(file).lines().filter_map(|line| line.ok()) {
        let chars = line.chars().collect_vec();
        let (first, second) = chars.split_at(chars.len() / 2);
        for c1 in first.iter() {
            for c2 in second.iter() {
                if c1 == c2 {
                    if c1.is_lowercase() {
                        sum += *c1 as i32 - 'a' as i32 + 1;
                    } else {
                        sum += *c1 as i32 - 'A' as i32 + 27;
                    }
                    continue 'outer;
                }
            }
        }
    }
    println!("{}", sum);

    // part 2
    let file = File::open("data/03.txt")?;
    let mut sum: i32 = 0;

    'outer: for lines in BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .chunks(3)
        .into_iter()
    {
        let (l1, l2, l3) = lines.collect_tuple().context("Not a valid tuple")?;
        let chars1 = l1.chars().collect_vec();
        let chars2 = l2.chars().collect_vec();
        let chars3 = l3.chars().collect_vec();
        for c1 in chars1.iter() {
            for c2 in chars2.iter() {
                if c1 != c2 {
                    continue;
                }
                for c3 in chars3.iter() {
                    if c1 == c3 {
                        if c1.is_lowercase() {
                            sum += *c1 as i32 - 'a' as i32 + 1;
                        } else {
                            sum += *c1 as i32 - 'A' as i32 + 27;
                        }
                        continue 'outer;
                    }
                }
            }
        }
    }
    println!("{}", sum);

    Ok(())
}

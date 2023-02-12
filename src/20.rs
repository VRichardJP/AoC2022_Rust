use anyhow::{Context, Result};
use itertools::Itertools;

const INPUT: &str = include_str!("../data/20.txt");

fn main() -> Result<()> {
    // part 1
    let mut data = Vec::new();
    for (i, line) in INPUT.lines().enumerate() {
        let n = line.parse::<i32>()?;
        data.push((i, n))
    }

    let size = data.len() as i32;
    for i in 0..data.len() {
        let (src, (_, n)) = data.iter().find_position(|&(j, _)| *j == i).unwrap();
        let dst = (src as i32 + n).rem_euclid(size - 1) as usize;
        let v = data.remove(src);
        data.insert(dst, v);
    }

    let (zero_idx, _) = data
        .iter()
        .find_position(|&(_, k)| *k == 0)
        .context("No 0 in the list")?;
    let sum = data[(zero_idx + 1000) % data.len()].1
        + data[(zero_idx + 2000) % data.len()].1
        + data[(zero_idx + 3000) % data.len()].1;
    println!("{sum}");

    // part 2
    let key = 811589153_i64;
    let mut data = Vec::new();
    for (i, line) in INPUT.lines().enumerate() {
        let n = line.parse::<i64>()?;
        data.push((i, n * key))
    }

    let size = data.len() as i64;
    for _ in 0..10 {
        for i in 0..data.len() {
            let (src, (_, n)) = data.iter().find_position(|&(j, _)| *j == i).unwrap();
            let dst = (src as i64 + n).rem_euclid(size - 1) as usize;
            let v = data.remove(src);
            data.insert(dst, v);
        }
    }

    let (zero_idx, _) = data
        .iter()
        .find_position(|&(_, k)| *k == 0)
        .context("No 0 in the list")?;
    let sum = data[(zero_idx + 1000) % data.len()].1
        + data[(zero_idx + 2000) % data.len()].1
        + data[(zero_idx + 3000) % data.len()].1;
    println!("{sum}");

    Ok(())
}

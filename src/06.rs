use anyhow::Result;
use itertools::Itertools;

const INPUT: &str = include_str!("../data/06.txt");

fn main() -> Result<()> {
    // part 1
    let mut i = 4;
    for window in INPUT.as_bytes().windows(4) {
        if window.iter().all_unique() {
            break;
        }
        i += 1;
    }
    println!("{i}");

    // part 1
    let mut i = 14;
    for window in INPUT.as_bytes().windows(14) {
        if window.iter().all_unique() {
            break;
        }
        i += 1;
    }
    println!("{i}");

    Ok(())
}

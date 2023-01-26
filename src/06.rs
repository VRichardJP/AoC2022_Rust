use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    // part 1
    let bytes = std::fs::read("data/06.txt")?;
    let mut i = 4;
    for window in bytes.windows(4) {
        if window.iter().all_unique() {
            break;
        }
        i += 1;
    }
    println!("{i}");

    // part 1
    let bytes = std::fs::read("data/06.txt")?;
    let mut i = 14;
    for window in bytes.windows(14) {
        if window.iter().all_unique() {
            break;
        }
        i += 1;
    }
    println!("{i}");

    Ok(())
}

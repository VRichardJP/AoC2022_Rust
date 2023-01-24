use anyhow::Result;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    // part 1
    let file = File::open("data/08.txt")?;
    let mut forest_matrix = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line?.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
        forest_matrix.push(line);
    }
    let forest_matrix = forest_matrix;

    // assume input is a valid matrix
    let rows = forest_matrix.len();
    let cols = forest_matrix[0].len();

    let mut nb_visible = 0;
    for i in 0..rows {
        for j in 0..cols {
            let height = forest_matrix[i][j];
            let visible_from_top = (0..i).all(|k| forest_matrix[k][j] < height);
            let visible_from_bottom = (i + 1..rows).all(|k| forest_matrix[k][j] < height);
            let visible_from_left = (0..j).all(|k| forest_matrix[i][k] < height);
            let visible_from_right = (j + 1..cols).all(|k| forest_matrix[i][k] < height);
            if visible_from_top || visible_from_bottom || visible_from_left || visible_from_right {
                nb_visible += 1;
            }
        }
    }
    println!("{nb_visible}");

    // part 2
    let mut highest_score = 0;
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let height = forest_matrix[i][j];
            let from_top = (1..i)
                .rev()
                .take_while(|&k| forest_matrix[k][j] < height)
                .count()
                + 1;
            let from_bottom = (i + 1..rows - 1)
                .take_while(|&k| forest_matrix[k][j] < height)
                .count()
                + 1;
            let from_left = (1..j)
                .rev()
                .take_while(|&k| forest_matrix[i][k] < height)
                .count()
                + 1;
            let from_right = (j + 1..cols - 1)
                .take_while(|&k| forest_matrix[i][k] < height)
                .count()
                + 1;
            let score = from_top * from_bottom * from_left * from_right;
            highest_score = std::cmp::max(highest_score, score);
        }
    }
    println!("{highest_score}");

    Ok(())
}

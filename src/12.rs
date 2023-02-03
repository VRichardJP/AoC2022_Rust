use anyhow::{Context, Result};
use itertools::Itertools;
use petgraph::{algo::dijkstra, graphmap::DiGraphMap};
use std::ops::{Index, IndexMut};

struct Matrix<T> {
    raw_data: Vec<T>,
    row_length: usize,
}

impl<T> Matrix<T> {
    fn new(raw_data: Vec<T>, row_length: usize) -> Self {
        assert!(raw_data.len() % row_length == 0);
        Matrix {
            raw_data,
            row_length,
        }
    }

    fn rows(&self) -> usize {
        self.raw_data.len() / self.row_length
    }

    fn cols(&self) -> usize {
        self.row_length
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.row_length;
        let end = start + self.row_length;
        &self.raw_data[start..end]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.row_length;
        let end = start + self.row_length;
        &mut self.raw_data[start..end]
    }
}

const INPUT: &str = include_str!("../data/12.txt");

fn main() -> Result<()> {
    // part 1
    // convert into a matrix of char
    let row_length = INPUT.lines().next().context("empty heightmap")?.len();
    let flat_heightmap = INPUT
        .lines()
        .flat_map(|line| line.chars().collect_vec())
        .collect_vec();
    let heightmap = Matrix::new(flat_heightmap.clone(), row_length);

    let rows = heightmap.rows();
    let cols = heightmap.cols();

    // find start location
    let start = {
        let mut start = (0, 0);
        'outer: for i in 0..rows {
            for j in 0..cols {
                if heightmap[i][j] == 'S' {
                    start = (i, j);
                    break 'outer;
                }
            }
        }
        start
    };

    // find target location
    let goal = {
        let mut goal = (0, 0);
        'outer: for i in 0..rows {
            for j in 0..cols {
                if heightmap[i][j] == 'E' {
                    goal = (i, j);
                    break 'outer;
                }
            }
        }
        goal
    };

    // rewrite heighmap as matrix of int
    let flat_heightmap = flat_heightmap
        .into_iter()
        .map(|c| {
            let c = match c {
                'E' => 'z',
                'S' => 'a',
                _ => c,
            };
            c as i32 - 'a' as i32
        })
        .collect_vec();
    let heightmap = Matrix::new(flat_heightmap, row_length);

    // list possible transitions
    let mut edges = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            let moves = [(-1, 0), (0, 1), (1, 0), (0, -1)];
            for m in moves.into_iter() {
                let m_i = i as i32 + m.0;
                let m_j = j as i32 + m.1;
                if m_i < 0 || m_i >= rows as i32 || m_j < 0 || m_j >= cols as i32 {
                    continue; // outside map
                }
                let (m_i, m_j) = (m_i as usize, m_j as usize);
                if heightmap[i][j] + 1 >= heightmap[m_i][m_j] {
                    edges.push(((i, j), (m_i, m_j)));
                }
            }
        }
    }

    // create and explore graph from start to goal
    let graph = DiGraphMap::<_, ()>::from_edges(edges);
    let distance_map = dijkstra(&graph, start, Some(goal), |_| 1);
    println!("{:?}", distance_map[&goal]);

    // part 2
    // convert into a matrix of char
    let row_length = INPUT.lines().next().context("empty heightmap")?.len();
    let flat_heightmap = INPUT
        .lines()
        .flat_map(|line| line.chars().collect_vec())
        .collect_vec();
    let heightmap = Matrix::new(flat_heightmap.clone(), row_length);

    let rows = heightmap.rows();
    let cols = heightmap.cols();

    // find start location candidates
    let starts = {
        let mut starts = Vec::new();
        for i in 0..rows {
            for j in 0..cols {
                if heightmap[i][j] == 'S' || heightmap[i][j] == 'a' {
                    starts.push((i, j));
                }
            }
        }
        starts
    };

    // find target location
    let goal = {
        let mut goal = (0, 0);
        'outer: for i in 0..rows {
            for j in 0..cols {
                if heightmap[i][j] == 'E' {
                    goal = (i, j);
                    break 'outer;
                }
            }
        }
        goal
    };

    // rewrite heighmap as matrix of int
    let flat_heightmap = flat_heightmap
        .into_iter()
        .map(|c| {
            let c = match c {
                'E' => 'z',
                'S' => 'a',
                _ => c,
            };
            c as i32 - 'a' as i32
        })
        .collect_vec();
    let heightmap = Matrix::new(flat_heightmap, row_length);

    // list possible transitions
    let mut edges = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            let moves = [(-1, 0), (0, 1), (1, 0), (0, -1)];
            for m in moves.into_iter() {
                let m_i = i as i32 + m.0;
                let m_j = j as i32 + m.1;
                if m_i < 0 || m_i >= rows as i32 || m_j < 0 || m_j >= cols as i32 {
                    continue; // outside map
                }
                let (m_i, m_j) = (m_i as usize, m_j as usize);
                if heightmap[i][j] + 1 >= heightmap[m_i][m_j] {
                    // going reverse
                    edges.push(((m_i, m_j), (i, j)));
                }
            }
        }
    }

    // create and explore graph from goal to everywhere
    let graph = DiGraphMap::<_, ()>::from_edges(edges);
    let distance_map = dijkstra(&graph, goal, None, |_| 1);
    let min_distance = distance_map
        .iter()
        .filter_map(|(coord, distance)| starts.contains(coord).then_some(distance))
        .min()
        .context("no path found")?;
    println!("{min_distance}");

    Ok(())
}

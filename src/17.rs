use std::{
    collections::{
        hash_map::{DefaultHasher, Entry},
        HashMap,
    },
    fmt::Display,
    hash::{Hash, Hasher},
};

use anyhow::{bail, Result};

#[derive(Default, Clone)]
struct Chamber {
    // each row content is represented by the first 7 bits
    // the 8th is outside the chamber and should always be 0
    rows: Vec<u8>,
    // current highest block
    tower_height: usize,
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter().rev() {
            write!(f, "|")?;
            for i in 0..7 {
                if (row >> (7 - i)) & 1 != 0 {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+")
    }
}

// compact bottom to top representation of each rock in a 4x7 matrix
// include initial left shift
// the 8th is outside the chamber and should always be 0
const ROCKS: [[u8; 4]; 5] = [
    [
        // horizontal bar
        0b00111100, 0b00000000, 0b00000000, 0b00000000,
    ],
    [
        // plus
        0b00010000, 0b00111000, 0b00010000, 0b00000000,
    ],
    [
        // turned L
        0b00111000, 0b00001000, 0b00001000, 0b00000000,
    ],
    [
        // vertical bar
        0b00100000, 0b00100000, 0b00100000, 0b00100000,
    ],
    [
        // square
        0b00110000, 0b00110000, 0b00000000, 0b00000000,
    ],
];

const INPUT: &str = include_str!("../data/17.txt");

fn main() -> Result<()> {
    // part 1
    let mut jet_shift = INPUT.chars().cycle();
    let mut rocks = ROCKS.iter().cycle();
    let mut chamber = Chamber::default();

    for _ in 0..2022 {
        // add a new rock in the chamber
        let mut rock = rocks.next().unwrap().to_owned();
        // position of the rock in the chamber
        let mut rock_height = chamber.tower_height + 3;
        // make sure the chamber is high enough to contain the rock
        chamber.rows.resize(rock_height + rock.len(), 0);

        // until the rock is stopped
        loop {
            // try to shift the rock left/right
            rock = match jet_shift.next().unwrap() {
                '<' => {
                    // check if we can shift left
                    if rock.iter().all(|bits| bits & 0b10000000 == 0) {
                        let mut shifted_rock = rock;
                        for bits in &mut shifted_rock {
                            *bits <<= 1;
                        }
                        // check if any collision would occur
                        let chamber_buf =
                            &chamber.rows[rock_height..rock_height + shifted_rock.len()];
                        let has_collision = chamber_buf
                            .iter()
                            .zip(shifted_rock.iter())
                            .any(|(b0, b1)| b0 & b1 != 0);
                        if has_collision {
                            rock // can't shift
                        } else {
                            shifted_rock // ok
                        }
                    } else {
                        rock // can't shift
                    }
                }
                '>' => {
                    // check if we can shift right
                    if rock.iter().all(|bits| bits & 0b00000010 == 0) {
                        let mut shifted_rock = rock;
                        for bits in &mut shifted_rock {
                            *bits >>= 1;
                            *bits &= 0b11111110; // reset 8th bit (outside the chamber)
                        }
                        // check if any collision would occur
                        let chamber_buf =
                            &chamber.rows[rock_height..rock_height + shifted_rock.len()];
                        let has_collision = chamber_buf
                            .iter()
                            .zip(shifted_rock.iter())
                            .any(|(b0, b1)| b0 & b1 != 0);
                        if has_collision {
                            rock // can't shift
                        } else {
                            shifted_rock // ok
                        }
                    } else {
                        rock // can't shift
                    }
                }
                c => bail!("unexpected char: {c}"),
            };

            // check if the rock can fall down
            if rock_height == 0 {
                break; // reached bottom
            }
            let chamber_buf = &chamber.rows[rock_height - 1..rock_height - 1 + rock.len()];
            let has_collision = chamber_buf
                .iter()
                .zip(rock.iter())
                .any(|(b0, b1)| b0 & b1 != 0);
            if has_collision {
                break; // hit something
            }

            // the rock felt 1 unit down
            rock_height -= 1;
        }

        // fix the rock
        for (row, rock_bits) in chamber.rows[rock_height..rock_height + rock.len()]
            .iter_mut()
            .zip(rock.iter())
        {
            *row |= rock_bits;
        }

        // update tower height
        for i in (0..chamber.rows.len()).rev() {
            if chamber.rows[i] != 0 {
                chamber.tower_height = i + 1;
                break;
            }
        }
    }
    println!("{}", chamber.tower_height);

    // part 2
    let jet_shift = INPUT.as_bytes();
    let mut jet_shift_idx = 0;
    let mut rock_idx = 0;
    let mut chamber = Chamber::default();
    // hashmap of tower top level hash and (k, tower_height) values
    let mut states = HashMap::<u64, (usize, usize)>::new();
    // if a loop is detected, simply skip as many iterations as we can and count them at the end of the simulation
    let mut skipped_height = None;
    let mut k = 0;
    const STOP_K: usize = 1000000000000;
    while k < STOP_K {
        // try to detect loop in the simulation
        // if a loop is found we can trivially predict the height of the tower over the looped section
        if skipped_height.is_none() {
            // extract tower top level
            // only the top level shape is relevant for the simulation (new rocks cannot reach what is below)
            let mut top_level_height = 0;
            for i in 0..7 {
                let mut h = 0;
                while chamber.tower_height > h
                    && chamber.rows[chamber.tower_height - h - 1] >> (7 - i) == 0
                {
                    h += 1;
                }
                top_level_height = top_level_height.max(h);
            }
            let top_level =
                &chamber.rows[(chamber.tower_height - top_level_height)..chamber.tower_height];
            // calculate hash of simulation state: current rock, shift instruction and tower top level shape
            let mut hasher = DefaultHasher::new();
            rock_idx.hash(&mut hasher);
            jet_shift_idx.hash(&mut hasher);
            top_level.hash(&mut hasher);
            let state_hash = hasher.finish();
            // check if this state has ever been reached
            if let Entry::Vacant(e) = states.entry(state_hash) {
                // new state
                e.insert((k, chamber.tower_height));
            } else {
                // already reached the same state before: simulation is looping
                let (loop_k_start, loop_height_start) = states[&state_hash];
                let (loop_k_end, loop_height_end) = (k, chamber.tower_height);
                let loop_k_length = loop_k_end - loop_k_start;
                let loop_height = loop_height_end - loop_height_start;
                let skipped_looped_nb = (STOP_K - k) / loop_k_length;
                // fast forwarding many loops
                skipped_height = Some(skipped_looped_nb * loop_height);
                k += skipped_looped_nb * loop_k_length;
            }
        }

        // add a new rock in the chamber
        let mut rock = ROCKS[rock_idx];
        rock_idx = (rock_idx + 1) % ROCKS.len();
        // position of the rock in the chamber
        let mut rock_height = chamber.tower_height + 3;
        // make sure the chamber is high enough to contain the rock
        chamber.rows.resize(rock_height + rock.len(), 0);

        // until the rock is stopped
        loop {
            let shift = jet_shift[jet_shift_idx] as char;
            jet_shift_idx = (jet_shift_idx + 1) % jet_shift.len();

            // try to shift the rock left/right
            rock = match shift {
                '<' => {
                    // check if we can shift left
                    if rock.iter().all(|bits| bits & 0b10000000 == 0) {
                        let mut shifted_rock = rock;
                        for bits in &mut shifted_rock {
                            *bits <<= 1;
                        }
                        // check if any collision would occur
                        let chamber_buf =
                            &chamber.rows[rock_height..rock_height + shifted_rock.len()];
                        let has_collision = chamber_buf
                            .iter()
                            .zip(shifted_rock.iter())
                            .any(|(b0, b1)| b0 & b1 != 0);
                        if has_collision {
                            rock // can't shift
                        } else {
                            shifted_rock // ok
                        }
                    } else {
                        rock // can't shift
                    }
                }
                '>' => {
                    // check if we can shift right
                    if rock.iter().all(|bits| bits & 0b00000010 == 0) {
                        let mut shifted_rock = rock;
                        for bits in &mut shifted_rock {
                            *bits >>= 1;
                            *bits &= 0b11111110; // reset 8th bit (outside the chamber)
                        }
                        // check if any collision would occur
                        let chamber_buf =
                            &chamber.rows[rock_height..rock_height + shifted_rock.len()];
                        let has_collision = chamber_buf
                            .iter()
                            .zip(shifted_rock.iter())
                            .any(|(b0, b1)| b0 & b1 != 0);
                        if has_collision {
                            rock // can't shift
                        } else {
                            shifted_rock // ok
                        }
                    } else {
                        rock // can't shift
                    }
                }
                c => bail!("unexpected char: {c}"),
            };

            // check if the rock can fall down
            if rock_height == 0 {
                break; // reached bottom
            }
            let chamber_buf = &chamber.rows[rock_height - 1..rock_height - 1 + rock.len()];
            let has_collision = chamber_buf
                .iter()
                .zip(rock.iter())
                .any(|(b0, b1)| b0 & b1 != 0);
            if has_collision {
                break; // hit something
            }

            // the rock felt 1 unit down
            rock_height -= 1;
        }

        // fix the rock
        for (row, rock_bits) in chamber.rows[rock_height..rock_height + rock.len()]
            .iter_mut()
            .zip(rock.iter())
        {
            *row |= rock_bits;
        }

        // update tower height
        for i in (0..chamber.rows.len()).rev() {
            if chamber.rows[i] != 0 {
                chamber.tower_height = i + 1;
                break;
            }
        }

        k += 1;
    }
    let total_height = chamber.tower_height + skipped_height.unwrap_or(0);
    println!("{total_height}");

    Ok(())
}

use std::fmt::Display;

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

    Ok(())
}

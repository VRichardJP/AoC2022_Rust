use anyhow::{bail, Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Cpu {
    clock: usize,
    x: i32,
    // instruction and elapsed cycles on it
    instr: Option<(Instruction, usize)>,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}
use Instruction::*;

impl Default for Cpu {
    fn default() -> Self {
        Self {
            clock: 0,
            x: 1,
            instr: None,
        }
    }
}

impl Cpu {
    fn get_signal_strength(&self) -> i32 {
        self.clock as i32 * self.x
    }

    fn tick(&mut self) -> Result<()> {
        self.clock += 1;

        let mut instr = self.instr.context("cpu has no instruction")?;
        instr.1 += 1;

        self.instr = match instr {
            (Noop, 1) => None,
            (Addx(v), 2) => {
                self.x += v;
                None
            }
            _ => Some(instr),
        };

        Ok(())
    }
}

static INSPECTION: [usize; 6] = [20, 60, 100, 140, 180, 220];

fn main() -> Result<()> {
    // part 1
    let file = File::open("data/10.txt")?;
    let mut lines = BufReader::new(file).lines();
    let mut cpu = Cpu::default();
    let mut sum = 0;
    loop {
        if cpu.clock > 220 {
            break;
        }

        if cpu.instr.is_none() {
            // load new instruction
            let line = lines.next().context("no new instruction")??;

            let instr = if line.starts_with("noop") {
                Noop
            } else if line.starts_with("addx") {
                let (_, v) = line
                    .split_once(' ')
                    .with_context(|| format!("invalid instruction: {}", line))?;
                let v = v.parse::<i32>()?;
                Addx(v)
            } else {
                bail!("unknown instruction: {}", line);
            };

            cpu.instr = Some((instr, 0));
        }

        // execute current instruction
        cpu.tick()?;

        // accumulate signal strength
        if INSPECTION.contains(&cpu.clock) {
            sum += cpu.get_signal_strength();
        }
    }
    println!("{sum}");

    // part 2
    let file = File::open("data/10.txt")?;
    let mut lines = BufReader::new(file).lines();
    let mut crt: [[bool; 40]; 6] = [[false; 40]; 6];
    let mut idx: usize = 0;
    let mut cpu = Cpu::default();
    loop {
        if cpu.clock >= 240 {
            break;
        }

        // update CRT
        let i = idx / 40;
        let j = idx % 40;
        crt[i][j] = (j as i32 - cpu.x).abs() <= 1;
        idx += 1;

        if cpu.instr.is_none() {
            // load new instruction
            let line = lines.next().context("no new instruction")??;

            let instr = if line.starts_with("noop") {
                Noop
            } else if line.starts_with("addx") {
                let (_, v) = line
                    .split_once(' ')
                    .with_context(|| format!("invalid instruction: {}", line))?;
                let v = v.parse::<i32>()?;
                Addx(v)
            } else {
                bail!("unknown instruction: {}", line);
            };

            cpu.instr = Some((instr, 0));
        }

        // execute current instruction
        cpu.tick()?;
    }

    for row in crt.iter() {
        for pixel in row.iter() {
            if *pixel {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    Ok(())
}

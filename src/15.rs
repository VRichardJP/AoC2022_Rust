use anyhow::{Context, Result};
use itertools::Itertools;
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Signal {
    sensor: (i32, i32),
    beacon: (i32, i32),
}

#[derive(Debug, Clone, Copy)]
struct Segment1D(i32, i32);

impl Segment1D {
    fn length(&self) -> i32 {
        self.1 - self.0
    }

    fn intersection(&self, other: &Self) -> Option<Segment1D> {
        let left = max(self.0, other.0);
        let right = min(self.1, other.1);
        if left <= right {
            Some(Segment1D(left, right))
        } else {
            None
        }
    }

    fn difference(&self, other: &Self) -> Vec<Segment1D> {
        let mut v = Vec::new();
        match self.intersection(other) {
            None => v.push(*self),
            Some(inter) => {
                if inter.0 != self.0 {
                    v.push(Segment1D(self.0, inter.0));
                }
                if inter.1 != self.1 {
                    v.push(Segment1D(inter.1, self.1));
                }
            }
        };
        v
    }
}

impl Signal {
    fn dist(&self) -> i32 {
        (self.beacon.0 - self.sensor.0).abs() + (self.beacon.1 - self.sensor.1).abs()
    }
}

fn main() -> Result<()> {
    // part 1
    let file = File::open("data/15.txt")?;
    let re = Regex::new(
        r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)",
    )?;
    let mut signals = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        let caps = re
            .captures(&line)
            .with_context(|| format!("Failed to parse line: {}", &line))?;
        let sensor_x = caps["sensor_x"].parse::<i32>()?;
        let sensor_y = caps["sensor_y"].parse::<i32>()?;
        let beacon_x = caps["beacon_x"].parse::<i32>()?;
        let beacon_y = caps["beacon_y"].parse::<i32>()?;
        signals.push(Signal {
            sensor: (sensor_x, sensor_y),
            beacon: (beacon_x, beacon_y),
        });
    }

    // find segments scanned on the row y=2000000
    const TARGET_ROW_Y: i32 = 2000000;
    let mut segments: Vec<Segment1D> = Vec::new();
    for signal in signals.iter() {
        // check if the circle (sensor, signal.dist) intersect with target row
        let d = signal.dist() - (signal.sensor.1 - TARGET_ROW_Y).abs();
        if d < 0 {
            continue; // too far from target row
        }
        segments.push(Segment1D(signal.sensor.0 - d, signal.sensor.0 + d + 1));
    }

    // rewrite segments list so they are all disjoints
    segments = {
        let mut disjoint_segments = Vec::new();
        for segment in segments {
            let mut tmp = vec![segment];
            for added in disjoint_segments.iter() {
                tmp = tmp
                    .into_iter()
                    .flat_map(|s| s.difference(added))
                    .collect_vec();
            }
            disjoint_segments.extend(tmp);
        }
        disjoint_segments
    };

    // list beacons on target row
    let mut beacons_on_target_row = HashSet::<(i32, i32)>::new();
    for signal in signals.iter() {
        if signal.beacon.1 == TARGET_ROW_Y {
            beacons_on_target_row.insert(signal.beacon);
        }
    }

    // remove beacon from segments
    segments = {
        let mut no_beacon_segment = Vec::new();
        for segment in segments {
            let mut tmp = vec![segment];
            for beacon in beacons_on_target_row.iter() {
                tmp = tmp
                    .into_iter()
                    .flat_map(|s| s.difference(&Segment1D(beacon.0, beacon.0 + 1)))
                    .collect_vec();
            }
            no_beacon_segment.extend(tmp);
        }
        no_beacon_segment
    };

    let no_beacon_count: i32 = segments.iter().map(|s| s.length()).sum();
    println!("{no_beacon_count}");

    Ok(())
}

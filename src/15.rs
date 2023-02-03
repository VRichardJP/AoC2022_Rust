use anyhow::{Context, Result};
use itertools::Itertools;
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::HashSet,
};

#[derive(Debug)]
struct Signal {
    sensor: (i64, i64),
    beacon: (i64, i64),
}

#[derive(Debug, Clone, Copy)]
struct Segment1D(i64, i64);

impl Segment1D {
    fn length(&self) -> u64 {
        (self.1 - self.0) as u64
    }

    fn is_empty(&self) -> bool {
        self.length() == 0
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
    fn dist(&self) -> i64 {
        (self.beacon.0 - self.sensor.0).abs() + (self.beacon.1 - self.sensor.1).abs()
    }
}

const INPUT: &str = include_str!("../data/15.txt");

fn main() -> Result<()> {
    // part 1
    let re = Regex::new(
        r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)",
    )?;
    let mut signals = Vec::new();
    for line in INPUT.lines() {
        let caps = re
            .captures(line)
            .with_context(|| format!("Failed to parse line: {}", &line))?;
        let sensor_x = caps["sensor_x"].parse::<i64>()?;
        let sensor_y = caps["sensor_y"].parse::<i64>()?;
        let beacon_x = caps["beacon_x"].parse::<i64>()?;
        let beacon_y = caps["beacon_y"].parse::<i64>()?;
        signals.push(Signal {
            sensor: (sensor_x, sensor_y),
            beacon: (beacon_x, beacon_y),
        });
    }

    // find segments scanned on the row y=2000000
    const TARGET_ROW_Y: i64 = 2000000;
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
    let mut beacons_on_target_row = HashSet::<(i64, i64)>::new();
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

    let no_beacon_count: u64 = segments.iter().map(|s| s.length()).sum();
    println!("{no_beacon_count}");

    // part 2
    let re = Regex::new(
        r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)",
    )?;
    let mut signals = Vec::new();
    for line in INPUT.lines() {
        let caps = re
            .captures(line)
            .with_context(|| format!("Failed to parse line: {}", &line))?;
        let sensor_x = caps["sensor_x"].parse::<i64>()?;
        let sensor_y = caps["sensor_y"].parse::<i64>()?;
        let beacon_x = caps["beacon_x"].parse::<i64>()?;
        let beacon_y = caps["beacon_y"].parse::<i64>()?;
        signals.push(Signal {
            sensor: (sensor_x, sensor_y),
            beacon: (beacon_x, beacon_y),
        });
    }

    // find the only position that has not been scanned
    let mut distress_beacon = None;
    for target_row_y in 0..4000000 {
        let mut unscanned_x = vec![Segment1D(0, 4000000)];
        for signal in signals.iter() {
            // check if the circle (sensor, signal.dist) intersect with target row
            let d = signal.dist() - (signal.sensor.1 - target_row_y).abs();
            if d < 0 {
                continue; // too far from target row
            }
            let scanned = Segment1D(signal.sensor.0 - d, signal.sensor.0 + d + 1);
            unscanned_x = unscanned_x
                .into_iter()
                .flat_map(|s| s.difference(&scanned))
                .filter(|s| !s.is_empty())
                .collect_vec();
            if unscanned_x.is_empty() {
                break; // everything has been scanned
            }
        }

        if !unscanned_x.is_empty() {
            // found something
            assert!(unscanned_x.len() == 1);
            assert!(unscanned_x[0].length() == 1);
            distress_beacon = Some(Segment1D(unscanned_x[0].0, target_row_y));
            break; // assume to be the only 1
        }
    }

    let distress_beacon = distress_beacon.context("did not find distress beacon")?;
    let tuning_frequency = distress_beacon.0 * 4000000 + distress_beacon.1;
    println!("{tuning_frequency}");

    Ok(())
}

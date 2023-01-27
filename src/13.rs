use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, i32},
    combinator::{cut, map},
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug, Clone)]
enum PacketData {
    PInt(i32),
    PList(Vec<PacketData>),
}
use PacketData::*;

fn parse_list(input: &str) -> IResult<&str, Vec<PacketData>> {
    preceded(
        char('['),
        cut(terminated(
            separated_list0(char(','), parse_packet),
            char(']'),
        )),
    )(input)
}

fn parse_packet(input: &str) -> IResult<&str, PacketData> {
    alt((map(i32, PInt), map(parse_list, PList)))(input)
}

fn parse_root(input: &str) -> IResult<&str, PacketData> {
    map(parse_list, PList)(input)
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PInt(i), PInt(j)) => i.cmp(j),
            (PInt(i), PList(_)) => PList(vec![PInt(*i)]).cmp(other),
            (PList(_), PInt(j)) => self.cmp(&PList(vec![PInt(*j)])),
            (PList(vl), PList(vr)) => {
                let mut vl = vl.iter();
                let mut vr = vr.iter();
                loop {
                    match (vl.next(), vr.next()) {
                        (None, None) => break Ordering::Equal,
                        (None, Some(_)) => break Ordering::Less,
                        (Some(_), None) => break Ordering::Greater,
                        (Some(left_item), Some(right_item)) => {
                            let res = left_item.cmp(right_item);
                            if res != Ordering::Equal {
                                break res;
                            }
                        }
                    };
                }
            }
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Less | Ordering::Equal)
        )
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(
            self.partial_cmp(other),
            Some(Ordering::Greater | Ordering::Equal)
        )
    }
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for PacketData {}

fn main() -> Result<()> {
    // part 1
    let file = File::open("data/13.txt")?;
    let lines = BufReader::new(file).lines();
    let mut sum = 0;
    for (i, (left, right)) in lines
        .filter_map(|l| l.ok())
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.take(2).collect_tuple::<(_, _)>().unwrap())
        .enumerate()
    {
        let (_, left_packet) = parse_root(&left)
            // Note: nom errors keep reference to the original &str, which cannot leave the function scope.
            // Hence the .to_owned() call to duplicate the contained strings.
            .map_err(|err| err.to_owned())?;
        let (_, right_packet) = parse_root(&right)
            // Note: nom errors keep reference to the original &str, which cannot leave the function scope.
            // Hence the .to_owned() call to duplicate the contained strings.
            .map_err(|err| err.to_owned())?;

        if left_packet < right_packet {
            sum += i + 1;
        }
    }
    println!("{sum}");

    // part 2
    let file = File::open("data/13.txt")?;
    let lines = BufReader::new(file).lines();
    let mut packets = Vec::new();
    for (left, right) in lines
        .filter_map(|l| l.ok())
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.take(2).collect_tuple::<(_, _)>().unwrap())
    {
        let (_, left_packet) = parse_root(&left)
            // Note: nom errors keep reference to the original &str, which cannot leave the function scope.
            // Hence the .to_owned() call to duplicate the contained strings.
            .map_err(|err| err.to_owned())?;
        let (_, right_packet) = parse_root(&right)
            // Note: nom errors keep reference to the original &str, which cannot leave the function scope.
            // Hence the .to_owned() call to duplicate the contained strings.
            .map_err(|err| err.to_owned())?;
        packets.push(left_packet);
        packets.push(right_packet);
    }
    let divider_2 = PList(vec![PList(vec![PInt(2)])]);
    let divider_6 = PList(vec![PList(vec![PInt(6)])]);

    packets.push(divider_2.clone());
    packets.push(divider_6.clone());
    packets.sort();

    let index_2 = packets
        .iter()
        .position(|p| *p == divider_2)
        .context("Failed to find packet [[2]]")?
        + 1;
    let index_6 = packets
        .iter()
        .position(|p| *p == divider_6)
        .context("Failed to find packet [[6]]")?
        + 1;
    let decoder_key = index_2 * index_6;
    println!("{decoder_key}");

    Ok(())
}

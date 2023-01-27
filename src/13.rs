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

#[derive(Debug)]
enum PacketData {
    Int(i32),
    List(Vec<PacketData>),
}

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
    alt((map(i32, PacketData::Int), map(parse_list, PacketData::List)))(input)
}

fn parse_root(input: &str) -> IResult<&str, PacketData> {
    map(parse_list, PacketData::List)(input)
}

fn is_right_order(left: &PacketData, right: &PacketData) -> Option<bool> {
    match (left, right) {
        (PacketData::Int(i), PacketData::Int(j)) => {
            match i.cmp(j) {
                Ordering::Less => Some(true),
                Ordering::Greater => Some(false),
                Ordering::Equal => None, // undecided
            }
        }
        (PacketData::Int(i), PacketData::List(_)) => {
            is_right_order(&PacketData::List(vec![PacketData::Int(*i)]), right)
        }
        (PacketData::List(_), PacketData::Int(j)) => {
            is_right_order(left, &PacketData::List(vec![PacketData::Int(*j)]))
        }
        (PacketData::List(vl), PacketData::List(vr)) => {
            let mut vl = vl.iter();
            let mut vr = vr.iter();

            loop {
                match (vl.next(), vr.next()) {
                    (None, None) => break None,
                    (None, Some(_)) => break Some(true),
                    (Some(_), None) => break Some(false),
                    (Some(left_item), Some(right_item)) => {
                        let res = is_right_order(left_item, right_item);
                        if res.is_some() {
                            break res;
                        }
                    }
                };
            }
        }
    }
}

fn main() -> Result<()> {
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

        if is_right_order(&left_packet, &right_packet).with_context(|| {
            format!("failed to determine order between {left_packet:?} and {right_packet:?}")
        })? {
            sum += i + 1;
        }
    }
    println!("{sum}");

    Ok(())
}

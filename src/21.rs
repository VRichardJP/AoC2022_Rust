use anyhow::{bail, Context, Result};
use id_arena::{Arena, Id};
use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
};

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
use Op::*;

impl TryFrom<char> for Op {
    type Error = anyhow::Error;

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        match c {
            '+' => Ok(Op::Add),
            '-' => Ok(Op::Sub),
            '*' => Ok(Op::Mul),
            '/' => Ok(Op::Div),
            _ => bail!("Unknown operator: {c}"),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
        }
    }
}

impl Op {
    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Add => left + right,
            Sub => left - right,
            Mul => left * right,
            Div => left / right,
        }
    }

    fn rev(&self) -> Self {
        match self {
            Add => Sub,
            Sub => Add,
            Mul => Div,
            Div => Mul,
        }
    }
}

mod parser {
    use crate::Op;
    use anyhow::{anyhow, Result};
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, anychar, char, digit1},
        combinator::{complete, eof, map_res},
        error::{convert_error, ParseError},
        sequence::delimited,
        Finish, IResult,
    };
    use std::fmt::Display;

    #[derive(Debug, Clone, Copy)]
    pub enum Monkey<'a> {
        Val(i64),
        Op(Op, &'a str, &'a str),
    }

    impl<'a> Display for Monkey<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Monkey::Val(v) => write!(f, "{v}"),
                Monkey::Op(op, name1, name2) => write!(f, "{name1} {op} {name2}"),
            }
        }
    }

    fn _value<'a, E>(input: &'a str) -> IResult<&'a str, Monkey<'a>, E>
    where
        E: ParseError<&'a str> + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
    {
        map_res(digit1, |s: &str| s.parse::<i64>().map(Monkey::Val))(input)
    }

    fn _op<'a, E>(input: &'a str) -> IResult<&'a str, Monkey<'a>, E>
    where
        E: ParseError<&'a str> + nom::error::FromExternalError<&'a str, anyhow::Error>,
    {
        let (input, name1) = alpha1(input)?;
        let (input, op) = delimited(char(' '), map_res(anychar, Op::try_from), char(' '))(input)?;
        let (input, name2) = alpha1(input)?;
        let monkey_job = Monkey::Op(op, name1, name2);
        Ok((input, monkey_job))
    }

    fn _monkey_job<'a, E>(input: &'a str) -> IResult<&'a str, (&'a str, Monkey<'a>), E>
    where
        E: ParseError<&'a str>
            + nom::error::FromExternalError<&'a str, std::num::ParseIntError>
            + nom::error::FromExternalError<&'a str, anyhow::Error>,
    {
        let (input, name) = alpha1(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, op) = alt((_value, _op))(input)?;
        eof(input)?;
        Ok((input, (name, op)))
    }

    pub fn monkey_job(input: &str) -> Result<(&str, Monkey)> {
        let (_, (name, job)) = complete(_monkey_job::<nom::error::VerboseError<&str>>)(input)
            .finish()
            .map_err(|e| anyhow!(convert_error(input, e)))?;
        Ok((name, job))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Monkey {
    Unknown,
    Val(i64),
    Op(Op, MonkeyId, MonkeyId),
}

type MonkeyId = Id<Monkey>;

fn eval(monkeys: &Arena<Monkey>, id: MonkeyId) -> i64 {
    match &monkeys[id] {
        Monkey::Unknown => panic!("Unknown value"),
        Monkey::Val(v) => *v,
        Monkey::Op(op, left, right) => op.apply(eval(monkeys, *left), eval(monkeys, *right)),
    }
}

/// walk up to the root id to evaluate unknown down child
fn solve(
    monkeys: &Arena<Monkey>,
    required_by: &HashMap<MonkeyId, Vec<MonkeyId>>,
    root_id: MonkeyId,
    id: MonkeyId,
) -> i64 {
    assert!(!required_by[&id].is_empty());
    let parent_id = required_by[&id][0];
    // walk up the tree
    match monkeys[parent_id] {
        Monkey::Unknown => panic!("Found unknown value while walking up"),
        Monkey::Val(_) => panic!("Found leaf while walking up"),
        Monkey::Op(op, left, right) => {
            if parent_id == root_id {
                // reached the root: left value = right value
                if left == id {
                    return eval(monkeys, right);
                } else {
                    return eval(monkeys, left);
                }
            }
            // reverse the parent operation
            let parent_val = solve(monkeys, required_by, root_id, parent_id);
            if id == left {
                // e.g parent = X * right => X = parent / right
                let right_val = eval(monkeys, right);
                op.rev().apply(parent_val, right_val)
            } else {
                // e.g parent = left - X => X = left - parent
                // but parent = left + X => X = parent - left
                let left_val = eval(monkeys, left);
                match op {
                    Add | Mul => op.rev().apply(parent_val, left_val),
                    Sub | Div => op.apply(left_val, parent_val),
                }
            }
        }
    }
}

const INPUT: &str = include_str!("../data/21.txt");

fn main() -> Result<()> {
    // part 1
    let mut monkeys = Arena::new();
    let mut monkey_ids = HashMap::<String, MonkeyId>::new();
    for line in INPUT.lines() {
        let (name, monkey_job) =
            parser::monkey_job(line).with_context(|| format!("Failed to parse: {line}"))?;
        let monkey = match monkey_job {
            parser::Monkey::Val(v) => Monkey::Val(v),
            parser::Monkey::Op(op, name1, name2) => {
                // create new monkey placeholder with temporVary value if necessary
                let id1 = *monkey_ids
                    .entry(name1.to_string())
                    .or_insert_with(|| monkeys.alloc(Monkey::Unknown));
                let id2 = *monkey_ids
                    .entry(name2.to_string())
                    .or_insert_with(|| monkeys.alloc(Monkey::Unknown));
                Monkey::Op(op, id1, id2)
            }
        };
        match monkey_ids.entry(name.to_string()) {
            Entry::Occupied(e) => {
                // update default placeholder
                monkeys[*e.get()] = monkey;
            }
            Entry::Vacant(e) => {
                e.insert(monkeys.alloc(monkey));
            }
        };
    }
    let n = eval(&monkeys, monkey_ids["root"]);
    println!("{n}");

    // part 2
    // assume "humn" appears exactly once, so we can walk through the tree from "humn" to root, while evaluating all the other branches
    // to make sure, we mark the human node as Unknown
    monkeys[monkey_ids["humn"]] = Monkey::Unknown;
    let mut required_by = HashMap::<MonkeyId, Vec<MonkeyId>>::new();
    for (id, monkey) in monkeys.iter() {
        match monkey {
            Monkey::Unknown | Monkey::Val(_) => (),
            Monkey::Op(_, left, right) => {
                required_by.entry(*left).or_default().push(id);
                required_by.entry(*right).or_default().push(id);
            }
        }
    }
    let n = solve(
        &monkeys,
        &required_by,
        monkey_ids["root"],
        monkey_ids["humn"],
    );
    println!("{n}");

    Ok(())
}

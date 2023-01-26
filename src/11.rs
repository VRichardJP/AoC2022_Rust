use anyhow::Result;
use itertools::Itertools;
use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;
use std::collections::VecDeque;
use std::vec::Vec;

struct Monkey<T> {
    inspected: usize,
    items: VecDeque<T>,
    operation: Box<dyn Fn(&T) -> T>,
    test: Box<dyn Fn(&T) -> usize>,
}

fn main() -> Result<()> {
    // part 1
    let mut monkeys = Vec::new();
    monkeys.push(Monkey::<u32> {
        inspected: 0,
        items: VecDeque::from([65, 58, 93, 57, 66]),
        operation: Box::new(|v| v * 7),
        test: Box::new(|v| if v % 19 == 0 { 6 } else { 4 }),
    });
    monkeys.push(Monkey::<u32> {
        inspected: 0,
        items: VecDeque::from([76, 97, 58, 72, 57, 92, 82]),
        operation: Box::new(|v| v + 4),
        test: Box::new(|v| if v % 3 == 0 { 7 } else { 5 }),
    });
    monkeys.push(Monkey::<u32> {
        inspected: 0,
        items: VecDeque::from([90, 89, 96]),
        operation: Box::new(|v| v * 5),
        test: Box::new(|v| if v % 13 == 0 { 5 } else { 1 }),
    });
    monkeys.push(Monkey::<u32> {
        inspected: 0,
        items: VecDeque::from([72, 63, 72, 99]),
        operation: Box::new(|v| v * v),
        test: Box::new(|v| if v % 17 == 0 { 0 } else { 4 }),
    });
    monkeys.push(Monkey::<u32> {
        inspected: 0,
        items: VecDeque::from([65]),
        operation: Box::new(|v| v + 1),
        test: Box::new(|v| if v % 2 == 0 { 6 } else { 2 }),
    });
    monkeys.push(Monkey::<u32> {
        inspected: 0,
        items: VecDeque::from([97, 71]),
        operation: Box::new(|v| v + 8),
        test: Box::new(|v| if v % 11 == 0 { 7 } else { 3 }),
    });
    monkeys.push(Monkey::<u32> {
        inspected: 0,
        items: VecDeque::from([83, 68, 88, 55, 87, 67]),
        operation: Box::new(|v| v + 2),
        test: Box::new(|v| if v % 5 == 0 { 2 } else { 1 }),
    });
    monkeys.push(Monkey::<u32> {
        inspected: 0,
        items: VecDeque::from([64, 81, 50, 96, 82, 53, 62, 92]),
        operation: Box::new(|v| v + 5),
        test: Box::new(|v| if v % 7 == 0 { 3 } else { 0 }),
    });

    for _ in 0..20 {
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys[m].items.pop_front() {
                let worry_level = (monkeys[m].operation)(&item) / 3;
                let target = (monkeys[m].test)(&worry_level);
                monkeys[m].inspected += 1;
                assert!(target != m); // cannot send yourself
                monkeys[target].items.push_back(worry_level);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    let monkey_business = monkeys[0].inspected * monkeys[1].inspected;
    println!("{monkey_business}");

    // part 2
    let mut monkeys = Vec::new();
    monkeys.push(Monkey::<BigUint> {
        inspected: 0,
        items: VecDeque::from(
            [65, 58, 93, 57, 66]
                .iter()
                .map(|v| v.to_biguint().unwrap())
                .collect_vec(),
        ),
        operation: Box::new(|v| v * 7.to_biguint().unwrap()),
        test: Box::new(|v| {
            if v % 19.to_biguint().unwrap() == Zero::zero() {
                6
            } else {
                4
            }
        }),
    });
    monkeys.push(Monkey::<BigUint> {
        inspected: 0,
        items: VecDeque::from(
            [76, 97, 58, 72, 57, 92, 82]
                .iter()
                .map(|v| v.to_biguint().unwrap())
                .collect_vec(),
        ),
        operation: Box::new(|v| v + 4.to_biguint().unwrap()),
        test: Box::new(|v| {
            if v % 3.to_biguint().unwrap() == Zero::zero() {
                7
            } else {
                5
            }
        }),
    });
    monkeys.push(Monkey::<BigUint> {
        inspected: 0,
        items: VecDeque::from(
            [90, 89, 96]
                .iter()
                .map(|v| v.to_biguint().unwrap())
                .collect_vec(),
        ),
        operation: Box::new(|v| v * 5.to_biguint().unwrap()),
        test: Box::new(|v| {
            if v % 13.to_biguint().unwrap() == Zero::zero() {
                5
            } else {
                1
            }
        }),
    });
    monkeys.push(Monkey::<BigUint> {
        inspected: 0,
        items: VecDeque::from(
            [72, 63, 72, 99]
                .iter()
                .map(|v| v.to_biguint().unwrap())
                .collect_vec(),
        ),
        operation: Box::new(|v| v * v),
        test: Box::new(|v| {
            if v % 17.to_biguint().unwrap() == Zero::zero() {
                0
            } else {
                4
            }
        }),
    });
    monkeys.push(Monkey::<BigUint> {
        inspected: 0,
        items: VecDeque::from([65].iter().map(|v| v.to_biguint().unwrap()).collect_vec()),
        operation: Box::new(|v| v + 1.to_biguint().unwrap()),
        test: Box::new(|v| {
            if v % 2.to_biguint().unwrap() == Zero::zero() {
                6
            } else {
                2
            }
        }),
    });
    monkeys.push(Monkey::<BigUint> {
        inspected: 0,
        items: VecDeque::from(
            [97, 71]
                .iter()
                .map(|v| v.to_biguint().unwrap())
                .collect_vec(),
        ),
        operation: Box::new(|v| v + 8.to_biguint().unwrap()),
        test: Box::new(|v| {
            if v % 11.to_biguint().unwrap() == Zero::zero() {
                7
            } else {
                3
            }
        }),
    });
    monkeys.push(Monkey::<BigUint> {
        inspected: 0,
        items: VecDeque::from(
            [83, 68, 88, 55, 87, 67]
                .iter()
                .map(|v| v.to_biguint().unwrap())
                .collect_vec(),
        ),
        operation: Box::new(|v| v + 2.to_biguint().unwrap()),
        test: Box::new(|v| {
            if v % 5.to_biguint().unwrap() == Zero::zero() {
                2
            } else {
                1
            }
        }),
    });
    monkeys.push(Monkey::<BigUint> {
        inspected: 0,
        items: VecDeque::from(
            [64, 81, 50, 96, 82, 53, 62, 92]
                .iter()
                .map(|v| v.to_biguint().unwrap())
                .collect_vec(),
        ),
        operation: Box::new(|v| v + 5.to_biguint().unwrap()),
        test: Box::new(|v| {
            if v % 7.to_biguint().unwrap() == Zero::zero() {
                3
            } else {
                0
            }
        }),
    });

    for _ in 0..10_000 {
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys[m].items.pop_front() {
                let worry_level = (monkeys[m].operation)(&item);
                let target = (monkeys[m].test)(&worry_level);
                monkeys[m].inspected += 1;
                assert!(target != m); // cannot send yourself
                monkeys[target].items.push_back(worry_level);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    let monkey_business = monkeys[0].inspected * monkeys[1].inspected;
    println!("{monkey_business}");

    Ok(())
}

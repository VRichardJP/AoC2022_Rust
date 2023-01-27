use anyhow::Result;
use itertools::{Itertools, izip};
use std::collections::VecDeque;
use std::ops::{Add, Mul};
use std::vec::Vec;

struct Monkey<T> {
    inspected: usize,
    items: VecDeque<T>,
    operation: Box<dyn Fn(&T) -> T>,
    test: Box<dyn Fn(&T) -> usize>,
}

const MONKEY_NB: usize = 8;
const MONKEY_DIVS: [i32; MONKEY_NB] = [19, 3, 13, 17, 2, 11, 5, 7];

// store remainder for each monkey
#[derive(Debug, Clone, Copy)]
struct ItemRem([i32; MONKEY_NB]);

impl ItemRem {
    fn new(v: i32) -> Self {
        let mut rems = [0; MONKEY_NB];
        for (r, p) in rems.iter_mut().zip(MONKEY_DIVS) {
            *r = v % p;
        }
        Self(rems)
    }
}

impl Add<i32> for ItemRem {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        let mut rems = self.0;
        for (r, p) in rems.iter_mut().zip(MONKEY_DIVS) {
            *r += rhs;
            *r %= p;
        }
        Self(rems)
    }
}

impl Mul<i32> for ItemRem {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        let mut rems = self.0;
        for (r, p) in rems.iter_mut().zip(MONKEY_DIVS) {
            *r *= rhs;
            *r %= p;
        }
        Self(rems)
    }
}

impl Mul<Self> for ItemRem {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut rems = self.0;
        for (r1, r2, p) in izip!(rems.iter_mut(), rhs.0, MONKEY_DIVS) {
            *r1 *= r2;
            *r1 %= p;
        }
        Self(rems)
    }
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
    monkeys.push(Monkey::<ItemRem> {
        inspected: 0,
        items: VecDeque::from(
            [65, 58, 93, 57, 66]
                .into_iter()
                .map(ItemRem::new)
                .collect_vec(),
        ),
        operation: Box::new(|&v| v * 7),
        test: Box::new(|v| if v.0[0] == 0 { 6 } else { 4 }),
    });
    monkeys.push(Monkey::<ItemRem> {
        inspected: 0,
        items: VecDeque::from(
            [76, 97, 58, 72, 57, 92, 82]
                .into_iter()
                .map(ItemRem::new)
                .collect_vec(),
        ),
        operation: Box::new(|&v| v + 4),
        test: Box::new(|v| if v.0[1] == 0 { 7 } else { 5 }),
    });
    monkeys.push(Monkey::<ItemRem> {
        inspected: 0,
        items: VecDeque::from([90, 89, 96].into_iter().map(ItemRem::new).collect_vec()),
        operation: Box::new(|&v| v * 5),
        test: Box::new(|v| if v.0[2] == 0 { 5 } else { 1 }),
    });
    monkeys.push(Monkey::<ItemRem> {
        inspected: 0,
        items: VecDeque::from([72, 63, 72, 99].into_iter().map(ItemRem::new).collect_vec()),
        operation: Box::new(|&v| v * v),
        test: Box::new(|v| if v.0[3] == 0 { 0 } else { 4 }),
    });
    monkeys.push(Monkey::<ItemRem> {
        inspected: 0,
        items: VecDeque::from([65].into_iter().map(ItemRem::new).collect_vec()),
        operation: Box::new(|&v| v + 1),
        test: Box::new(|v| if v.0[4] == 0 { 6 } else { 2 }),
    });
    monkeys.push(Monkey::<ItemRem> {
        inspected: 0,
        items: VecDeque::from([97, 71].into_iter().map(ItemRem::new).collect_vec()),
        operation: Box::new(|&v| v + 8),
        test: Box::new(|v| if v.0[5] == 0 { 7 } else { 3 }),
    });
    monkeys.push(Monkey::<ItemRem> {
        inspected: 0,
        items: VecDeque::from(
            [83, 68, 88, 55, 87, 67]
                .into_iter()
                .map(ItemRem::new)
                .collect_vec(),
        ),
        operation: Box::new(|&v| v + 2),
        test: Box::new(|v| if v.0[6] == 0 { 2 } else { 1 }),
    });
    monkeys.push(Monkey::<ItemRem> {
        inspected: 0,
        items: VecDeque::from(
            [64, 81, 50, 96, 82, 53, 62, 92]
                .into_iter()
                .map(ItemRem::new)
                .collect_vec(),
        ),
        operation: Box::new(|&v| v + 5),
        test: Box::new(|v| if v.0[7] == 0 { 3 } else { 0 }),
    });

    for _ in 0..10000 {
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

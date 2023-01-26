use anyhow::Result;
use itertools::Itertools;
use std::collections::VecDeque;
use std::ops::{Add, Mul};
use std::vec::Vec;

struct Monkey<T> {
    inspected: usize,
    items: VecDeque<T>,
    operation: Box<dyn Fn(&T) -> T>,
    test: Box<dyn Fn(&T) -> usize>,
}

// store remainder for each monkey
#[derive(Debug, Clone, Copy)]
struct ItemRem<const N: usize> {
    rems: [i32; N],
    divs: [i32; N],
}

impl<const N: usize> ItemRem<N> {
    fn new(v: i32, divs: [i32; N]) -> Self {
        let mut rems = [0; N];
        for i in 0..N {
            rems[i] = v % divs[i];
        }
        Self { rems, divs }
    }
}

impl<const N: usize> Add<i32> for ItemRem<N> {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        let mut tmp = self;
        for i in 0..N {
            tmp.rems[i] += rhs;
            tmp.rems[i] %= tmp.divs[i];
        }
        tmp
    }
}

impl<const N: usize> Mul<i32> for ItemRem<N> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        let mut tmp = self;
        for i in 0..N {
            tmp.rems[i] *= rhs;
            tmp.rems[i] %= tmp.divs[i];
        }
        tmp
    }
}

impl<const N: usize> Mul<Self> for ItemRem<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.divs == rhs.divs);
        let mut tmp = self;
        for i in 0..N {
            tmp.rems[i] *= rhs.rems[i];
            tmp.rems[i] %= tmp.divs[i];
        }
        tmp
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
    const N: usize = 8;
    const MONKEY_DIVS: [i32; N] = [19, 3, 13, 17, 2, 11, 5, 7];
    let mut monkeys = Vec::new();
    monkeys.push(Monkey::<ItemRem<N>> {
        inspected: 0,
        items: VecDeque::from(
            [65, 58, 93, 57, 66]
                .into_iter()
                .map(|v| ItemRem::new(v, MONKEY_DIVS))
                .collect_vec(),
        ),
        operation: Box::new(|&v| v * 7),
        test: Box::new(|v| if v.rems[0] == 0 { 6 } else { 4 }),
    });
    monkeys.push(Monkey::<ItemRem<N>> {
        inspected: 0,
        items: VecDeque::from(
            [76, 97, 58, 72, 57, 92, 82]
                .into_iter()
                .map(|v| ItemRem::new(v, MONKEY_DIVS))
                .collect_vec(),
        ),
        operation: Box::new(|&v| v + 4),
        test: Box::new(|v| if v.rems[1] == 0 { 7 } else { 5 }),
    });
    monkeys.push(Monkey::<ItemRem<N>> {
        inspected: 0,
        items: VecDeque::from(
            [90, 89, 96]
                .into_iter()
                .map(|v| ItemRem::new(v, MONKEY_DIVS))
                .collect_vec(),
        ),
        operation: Box::new(|&v| v * 5),
        test: Box::new(|v| if v.rems[2] == 0 { 5 } else { 1 }),
    });
    monkeys.push(Monkey::<ItemRem<N>> {
        inspected: 0,
        items: VecDeque::from(
            [72, 63, 72, 99]
                .into_iter()
                .map(|v| ItemRem::new(v, MONKEY_DIVS))
                .collect_vec(),
        ),
        operation: Box::new(|&v| v * v),
        test: Box::new(|v| if v.rems[3] == 0 { 0 } else { 4 }),
    });
    monkeys.push(Monkey::<ItemRem<N>> {
        inspected: 0,
        items: VecDeque::from(
            [65].into_iter()
                .map(|v| ItemRem::new(v, MONKEY_DIVS))
                .collect_vec(),
        ),
        operation: Box::new(|&v| v + 1),
        test: Box::new(|v| if v.rems[4] == 0 { 6 } else { 2 }),
    });
    monkeys.push(Monkey::<ItemRem<N>> {
        inspected: 0,
        items: VecDeque::from(
            [97, 71]
                .into_iter()
                .map(|v| ItemRem::new(v, MONKEY_DIVS))
                .collect_vec(),
        ),
        operation: Box::new(|&v| v + 8),
        test: Box::new(|v| if v.rems[5] == 0 { 7 } else { 3 }),
    });
    monkeys.push(Monkey::<ItemRem<N>> {
        inspected: 0,
        items: VecDeque::from(
            [83, 68, 88, 55, 87, 67]
                .into_iter()
                .map(|v| ItemRem::new(v, MONKEY_DIVS))
                .collect_vec(),
        ),
        operation: Box::new(|&v| v + 2),
        test: Box::new(|v| if v.rems[6] == 0 { 2 } else { 1 }),
    });
    monkeys.push(Monkey::<ItemRem<N>> {
        inspected: 0,
        items: VecDeque::from(
            [64, 81, 50, 96, 82, 53, 62, 92]
                .into_iter()
                .map(|v| ItemRem::new(v, MONKEY_DIVS))
                .collect_vec(),
        ),
        operation: Box::new(|&v| v + 5),
        test: Box::new(|v| if v.rems[7] == 0 { 3 } else { 0 }),
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

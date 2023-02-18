const INPUT: &str = include_str!("../data/25.txt");

fn main() {
    let mut sum: i64 = 0;
    for line in INPUT.lines() {
        let mut val = 0;
        for c in line.chars() {
            val *= 5;
            val += match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("invalid SNAFU digit: '{c}'"),
            };
        }
        sum += val;
    }

    let mut snafu = Vec::new();
    let mut val = sum;
    while val != 0 {
        let r = val % 5;
        val = val / 5;
        let c = match r {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                val += 1;
                '='
            }
            4 => {
                val += 1;
                '-'
            }
            _ => unreachable!("bad math"),
        };
        snafu.push(c);
    }
    snafu.reverse();
    let snafu = snafu.into_iter().collect::<String>();
    println!("{snafu}");
}

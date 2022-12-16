use std::collections::HashSet;

use crate::formater::*;
use itertools::Itertools;

pub const FILE: usize = 14;
type Int = u64;
type Data = (HashSet<(i16, i16)>, i16);

pub fn run(filename: Option<String>) -> Result<(), std::io::Error> {
    print_single_parse(FILE, filename, parse_input, part_1, part_2)
}

fn parse_input(input: &str) -> Data {
    let mut out = HashSet::new();
    let mut lines = input.lines().collect_vec();
    let mut max = 0;
    lines.sort();
    lines.iter().dedup().for_each(|s| {
        s.split(" -> ")
            .map(|p| {
                p.split(',')
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect_tuple::<(i16, i16)>()
                    .unwrap()
            })
            .map(|i| {
                max = max.max(i.1);
                i
            })
            .tuple_windows()
            .for_each(|((ax, ay), (bx, by))| {
                if ax == bx {
                    for y in ay.min(by)..=ay.max(by) {
                        out.insert((ax, y));
                    }
                } else if ay == by {
                    for x in ax.min(bx)..=ax.max(bx) {
                        out.insert((x, ay));
                    }
                } else {
                    panic!("unaligned steps")
                }
            })
    });
    (out, max)
}

fn part_1(data: &Data) -> Int {
    let mut map = data.0.clone();
    let max = data.1;

    let mut grain_count = 0;

    'all_grains: loop {
        let mut y = 0;
        let mut x = 500;

        'one_grain: loop {
            if y > max {
                break 'all_grains;
            }
            for nx in [x, x - 1, x + 1] {
                if !map.contains(&(nx, y + 1)) {
                    x = nx;
                    y = y + 1;
                    continue 'one_grain;
                }
            }
            map.insert((x, y));
            break 'one_grain;
        }
        grain_count += 1;
    }

    grain_count
}

fn part_2(data: &Data) -> Int {
    let mut map = data.0.clone();
    let max = data.1 + 1;

    let mut grain_count = 0;

    'all_grains: loop {
        let mut y = 0;
        let mut x = 500;

        'one_grain: loop {
            if y >= max {
                break 'one_grain;
            }
            for nx in [x, x - 1, x + 1] {
                if !map.contains(&(nx, y + 1)) {
                    x = nx;
                    y = y + 1;
                    continue 'one_grain;
                }
            }
            break 'one_grain;
        }
        map.insert((x, y));
        grain_count += 1;
        if x == 500 && y == 0 {
            break 'all_grains;
        }
    }

    grain_count
}

#[cfg(test)]
mod tests {
    use crate::day_14::{parse_input, part_1, part_2, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_example() {
        let exemple = parse_input(EXEMPLE_1);
        assert_eq!(part_1(&exemple), 24);
        assert_eq!(part_2(&exemple), 93);
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}", 858));
        assert_eq!(part_2(&data).to_string(), format!("{}", 26845));
    }
}

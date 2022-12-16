use crate::formater::*;
use itertools::Itertools;
use rangemap::RangeInclusiveSet;

pub const FILE: usize = 15;
type Int = i64;
type Data = (Vec<((Int, Int), (Int, Int))>, Int);

fn distance(a: &(Int, Int), b: &(Int, Int)) -> Int {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn run(filename: Option<String>) -> Result<(), std::io::Error> {
    print_single_parse(FILE, filename, parse_input, part_1, part_2)
}
//012345678901234567890
//Sensor at x=3540455, y=2469135: closest beacon is at x=3866712, y=2438950
fn parse_input(input: &str) -> Data {
    (
        input
            .lines()
            .map(|line| {
                let list: Vec<Int> = line[12..]
                    .split('=')
                    .map(|part| {
                        let i: String = part
                            .chars()
                            .take_while(|c| c.is_ascii_digit() || *c == '-')
                            .collect();
                        i.parse().unwrap()
                    })
                    .collect_vec();
                ((list[0], list[1]), (list[2], list[3]))
            })
            .collect(),
        2000000,
    )
}

fn part_1(data: &Data) -> Int {
    let reference = data.1;
    let sensors = data
        .0
        .iter()
        .map(|(s, b)| (s.clone(), distance(s, b)))
        .collect_vec();

    let ranges: RangeInclusiveSet<Int> = sensors
        .iter()
        .filter_map(|(s, r)| {
            let distance = distance(s, &(s.0, reference));
            let left = r - distance;
            if left >= 0 {
                Some(s.0 - left..=s.0 + left)
            } else {
                None
            }
        })
        .collect();
    let mut to_substract = data
        .0
        .iter()
        .filter_map(|(_, (x, y))| {
            if *y == reference && ranges.contains(x) {
                Some(x)
            } else {
                None
            }
        })
        .collect_vec();
    to_substract.sort();
    to_substract.dedup();
    ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<i64>()
        - to_substract.len() as Int
}

fn part_2(data: &Data) -> Int {
    let reference = data.1;
    let sensors = data
        .0
        .iter()
        .map(|(s, b)| (s.clone(), distance(s, b)))
        .collect_vec();

    /*let search_range: RangeInclusiveSet<Int> = sensors
    .iter()
    .map(|((x, y), r)| 0.max(y - r)..=y + r)
    .collect();*/

    for y in /*search_range.iter().map(|x| x.clone().into_iter()).flatten()*/
        (0..reference).rev().interleave(reference..reference * 2)
    {
        let ranges: RangeInclusiveSet<Int> = sensors
            .iter()
            .filter_map(|(s, r)| {
                let distance = distance(s, &(s.0, y));
                let left = r - distance;
                if left >= 0 {
                    Some(s.0 - left..=s.0 + left)
                } else {
                    None
                }
            })
            .collect();
        if ranges.iter().count() > 1 {
            return (ranges.iter().next().unwrap().end() + 1) * 4000000 + y;
        }
    }
    panic!("no solution found")
}

#[cfg(test)]
mod tests {
    use crate::day_15::{parse_input, part_1, part_2, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_example() {
        let exemple = (parse_input(EXEMPLE_1).0, 10);
        assert_eq!(part_1(&exemple), 26);
        assert_eq!(part_2(&exemple), 56000011);
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}", 4665948));
        assert_eq!(part_2(&data).to_string(), format!("{}", 13543690671045i64));
    }
}

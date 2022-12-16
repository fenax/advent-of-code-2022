use std::collections::HashSet;

use crate::formater::*;
use itertools::Itertools;

pub const FILE: usize = 12;
type Int = u64;
type Data = (Vec<Vec<u8>>, (i8, i8), (i8, i8));

pub fn run(filename: Option<String>) -> Result<(), std::io::Error> {
    print_single_parse(FILE, filename, parse_input, part_1, part_2)
}

fn parse_input(input: &str) -> Data {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.bytes()
                .enumerate()
                .map(|(x, b)| match b {
                    b'a'..=b'z' => b - b'a',
                    b'S' => {
                        start = (x as i8, y as i8);
                        0
                    }
                    b'E' => {
                        end = (x as i8, y as i8);
                        b'z' - b'a'
                    }
                    _ => panic!("invalid input"),
                })
                .collect_vec()
        })
        .collect_vec();
    (map, start, end)
}

fn part_1((map, start, end): &Data) -> Int {
    let height = map.len() as i8;
    let width = map[0].len() as i8;

    //let mut path = vec![vec![Option::<NonZeroU32>::None;width];height];

    let mut full_path = HashSet::<(i8, i8)>::new();
    let mut last_pass_path = vec![*start];
    full_path.insert(*start);

    for i in 1.. {
        let iter = last_pass_path
            .iter()
            .map(|(x, y)| {
                let z = map[*y as usize][*x as usize];
                //println!("{} {} {}", x, y, z);
                [
                    (z, *x - 1, *y),
                    (z, *x + 1, *y),
                    (z, *x, *y - 1),
                    (z, *x, *y + 1),
                ]
            })
            .flatten()
            .filter(|(_, x, y)| *x >= 0 && *y >= 0 && *y < height && *x < width)
            .filter(|(z, x, y)| {
                //println!("{} {}", z, map[*y as usize][*x as usize]);
                z + 1 >= map[*y as usize][*x as usize]
            })
            .map(|(_, x, y)| (x, y));

        let current_pass_path: HashSet<(i8, i8)> = HashSet::from_iter(iter);
        if current_pass_path.contains(end) {
            return i;
        }
        let new = current_pass_path
            .difference(&full_path)
            .cloned()
            .collect_vec();

        full_path.extend(new.iter());
        if new.len() == 0 {
            panic!("broken {}", i)
        }
        last_pass_path = new;
    }

    panic!("should not reach here");
}

fn part_2((map, _, end): &Data) -> Int {
    let height = map.len() as i8;
    let width = map[0].len() as i8;

    //let mut path = vec![vec![Option::<NonZeroU32>::None;width];height];

    let mut full_path = HashSet::<(i8, i8)>::new();
    let mut last_pass_path = vec![*end];
    full_path.insert(*end);

    for i in 0.. {
        let vector = last_pass_path
            .iter()
            .map(|(x, y)| {
                let z = map[*y as usize][*x as usize];
                //                println!("{} {} {}", x, y, z);
                [
                    (z, *x - 1, *y),
                    (z, *x + 1, *y),
                    (z, *x, *y - 1),
                    (z, *x, *y + 1),
                ]
            })
            .flatten()
            .filter(|(_, x, y)| *x >= 0 && *y >= 0 && *y < height && *x < width)
            .filter(|(z, x, y)| {
                //                println!("{} {}", z, map[*y as usize][*x as usize]);
                *z <= map[*y as usize][*x as usize] + 1
            })
            .collect_vec();

        if let Some(_) = vector.iter().find(|(z, _, _)| *z == 0) {
            return i;
        }
        let current_pass_path: HashSet<(i8, i8)> =
            HashSet::from_iter(vector.iter().map(|(_, x, y)| (*x, *y)));

        let new = current_pass_path
            .difference(&full_path)
            .cloned()
            .collect_vec();

        full_path.extend(new.iter());
        if new.len() == 0 {
            panic!("broken {}", i)
        }
        last_pass_path = new;
    }

    panic!("should not reach here");
}

#[cfg(test)]
mod tests {
    use crate::day_12::{parse_input, part_1, part_2, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_example() {
        let exemple = parse_input(EXEMPLE_1);
        assert_eq!(part_1(&exemple), 31);
        assert_eq!(part_2(&exemple), 29);
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}", 383));
        assert_eq!(part_2(&data).to_string(), format!("{}", 377));
    }
}

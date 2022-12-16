use itertools::Itertools;

use crate::formater::*;

pub const FILE: usize = 13;
type Int = u64;
type Data = Vec<(Thing, Thing)>;

#[derive(PartialEq, Eq, Clone, Debug, Ord)]
enum Thing {
    List(Vec<Thing>),
    Int(Int),
}

impl PartialOrd for Thing {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Thing::Int(s), Thing::Int(o)) => s.partial_cmp(o),
            (Thing::List(_), Thing::Int(_)) => self.partial_cmp(&Thing::List(vec![other.clone()])),
            (Thing::Int(_), Thing::List(_)) => Thing::List(vec![self.clone()]).partial_cmp(other),
            (Thing::List(s), Thing::List(o)) => s.partial_cmp(o),
        }
    }
}

pub fn run(filename: Option<String>) -> Result<(), std::io::Error> {
    print_single_parse(FILE, filename, parse_input, part_1, part_2)
}

fn parse_iterator_list<'a, 'b>(input: &'a mut impl Iterator<Item = u8>) -> Thing {
    let mut list: Vec<Thing> = Vec::new();
    let mut intpart: Option<Int> = None;
    loop {
        match input.next() {
            Some(b'[') => {
                list.push(parse_iterator_list(input));
                intpart = None
            }
            Some(b']') => {
                if let Some(int) = intpart {
                    list.push(Thing::Int(int));
                    //intpart = None; breaking
                }
                break;
            }
            Some(b',') => {
                if let Some(int) = intpart {
                    list.push(Thing::Int(int));
                    intpart = None;
                }
            }
            Some(c @ b'0'..=b'9') => {
                intpart =
                    Some(((c - b'0') as Int) + if let Some(int) = intpart { int * 10 } else { 0 })
            }
            Some(_) => panic!("invalid symbol"),
            None => {
                return list.first().unwrap_or(&Thing::List(Vec::new())).clone();
            }
        }
    }
    Thing::List(list)
}

fn parse_input(input: &str) -> Data {
    input
        .split("\n\n")
        .filter_map(|group| {
            group
                .lines()
                .map(|s| parse_iterator_list(&mut s.bytes()))
                .collect_tuple()
        })
        .collect_vec()
}

fn part_1(data: &Data) -> Int {
    data.iter()
        .enumerate()
        .filter_map(|(i, (left, right))| {
            if left < right {
                Some(i as Int + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(data: &Data) -> Int {
    let mut list: Vec<&Thing> = data.iter().map(|(a, b)| [a, b]).flatten().collect();
    let a = Thing::List(vec![Thing::List(vec![Thing::Int(2)])]);
    let b = Thing::List(vec![Thing::List(vec![Thing::Int(6)])]);
    list.push(&a);
    list.push(&b);
    list.sort();
    list.iter()
        .enumerate()
        .filter_map(|(i, thing)| {
            if *thing == &a || *thing == &b {
                Some(i as Int + 1)
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use crate::day_13::{parse_input, part_1, part_2, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test_example() {
        let exemple = parse_input(EXEMPLE_1);
        assert_eq!(part_1(&exemple), 13);
        assert_eq!(part_2(&exemple), 140);
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}", 6240));
        assert_eq!(part_2(&data).to_string(), format!("{}", 23142));
    }
}

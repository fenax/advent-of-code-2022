use crate::formater::*;
use itertools::Itertools;

pub const FILE: usize = 11;
type Int = u64;
type Data = Vec<Monkey>;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Int),
    Mul(Int),
    Square,
}

impl Operation {
    fn apply(&self, val: Int) -> Int {
        match self {
            Operation::Add(x) => *x + val,
            Operation::Mul(x) => *x * val,
            Operation::Square => val * val,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    inspections: Int,
    items: Vec<Int>,
    operation: Operation,
    test: Int,
    if_true: usize,
    if_false: usize,
}

pub fn run() -> Result<(), std::io::Error> {
    print_single_parse(FILE, parse_input, part_1, part_2)
}

//0000000000111111111122222222223333
//0123456789012345678901234567890123
//Monkey 0:
//  Starting items: 74, 64, 74, 63, 53
//  Operation: new = old * 7
//  Test: divisible by 5
//    If true: throw to monkey 1
//    If false: throw to monkey 6
fn parse_input(input: &str) -> Data {
    input
        .split("\n\n")
        .map(|m| {
            if let Some((_no, src_item, src_operation, src_test, src_true, src_false)) =
                m.lines().collect_tuple()
            {
                Monkey {
                    inspections: 0,
                    items: src_item[18..]
                        .split(',')
                        .map(str::trim)
                        .map(str::parse)
                        .map(Result::unwrap)
                        .collect_vec(),
                    operation: match src_operation.as_bytes()[23] {
                        b'+' => Operation::Add(src_operation[25..].parse().unwrap()),
                        b'*' => {
                            let param = &src_operation[25..];
                            if param == "old" {
                                Operation::Square
                            } else {
                                Operation::Mul(param.parse().unwrap())
                            }
                        }
                        _ => panic!("invalid math"),
                    },
                    test: src_test[21..].parse().unwrap(),
                    if_true: src_true[29..].parse().unwrap(),
                    if_false: src_false[30..].parse().unwrap(),
                }
            } else {
                panic!("invalid monkey \n {}", m)
            }
        })
        .collect()
}

fn part_1(data: &Data) -> Int {
    let mut data = data.clone();
    let len = data.len();
    for _ in 0..20 {
        //rounds
        for m in 0..len {
            let transfer = data[m]
                .items
                .iter()
                .map(|item| {
                    let mut worry = data[m].operation.apply(*item);
                    worry /= 3;

                    (
                        if (worry % data[m].test) == 0 {
                            data[m].if_true
                        } else {
                            data[m].if_false
                        },
                        worry,
                    )
                })
                .collect_vec();
            data[m].items.clear();
            data[m].inspections += transfer.len() as Int;
            for (target, worry) in transfer {
                data[target].items.push(worry);
            }
        }
    }
    let mut scores = data.iter().map(|monkey| monkey.inspections).collect_vec();
    scores.sort();
    scores.pop().unwrap() * scores.pop().unwrap()
}

fn part_2(data: &Data) -> Int {
    let mut data = data.clone();
    let common = data.iter().fold(1, |acc, monkey| acc * monkey.test);

    //println!("common is {}", common);

    let len = data.len();
    for _x in 0..10000 {
        //rounds
        for m in 0..len {
            let transfer = data[m]
                .items
                .iter()
                .map(|item| {
                    let worry = data[m].operation.apply(*item);
                    //worry /= 3;

                    (
                        if (worry % data[m].test) == 0 {
                            data[m].if_true
                        } else {
                            data[m].if_false
                        },
                        worry,
                    )
                })
                .collect_vec();
            data[m].items.clear();
            data[m].inspections += transfer.len() as Int;
            for (target, worry) in transfer {
                data[target].items.push(worry % common);
            }
        }
    }
    let mut scores = data.iter().map(|monkey| monkey.inspections).collect_vec();
    scores.sort();
    scores.pop().unwrap() * scores.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_11::{parse_input, part_1, part_2, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_example() {
        let exemple = parse_input(EXEMPLE_1);

        //assert_eq!(parse_input(EXEMPLE_1), exemple);
        assert_eq!(part_1(&exemple), 10605);
        assert_eq!(part_2(&exemple), 2713310158);
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}", 54054));
        assert_eq!(part_2(&data).to_string(), format!("{}", 14314925001i128));
    }
}

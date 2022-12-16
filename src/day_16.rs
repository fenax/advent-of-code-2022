use std::collections::HashMap;

use crate::formater::*;

pub const FILE: usize = 16;
type Int = u64;
type Data = HashMap<[u8; 2], (Int, Vec<[u8; 2]>)>;

fn build_value_list(data: &Data) -> Vec<(Int, [u8; 2])> {
    Has
}

pub fn run(filename: Option<String>) -> Result<(), std::io::Error> {
    print_single_parse(FILE, filename, parse_input, part_1, part_2)
}

//0123456789012345678901234567890
//Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
//                        1234567890123456789012345
fn parse_input(input: &str) -> Data {
    input
        .lines()
        .map(|line| {
            let key: [u8; 2] = line[6..8].clone();
            let mut rest = line[23..].chars();
            let rate: String = rest.take_while(char::is_ascii_digit).collect();
            let targets: String = rest.skip(24).collect();
            (
                key,
                (
                    rate.parse().unwrap(),
                    targets.split(", ").map(|x| x[..2].clone()).collect(),
                ),
            )
        })
        .collect()
}

fn part_1(data: &Data) -> Int {
    0
}

fn part_2(data: &Data) -> Int {
    0
}

#[cfg(test)]
mod tests {
    use crate::day_16::{parse_input, part_1, part_2, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_example() {
        let exemple = ();

        assert_eq!(parse_input(EXEMPLE_1), exemple);
        assert_eq!(part_1(&exemple), 0);
        assert_eq!(part_2(&exemple), 0);
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}", 0));
        assert_eq!(part_2(&data).to_string(), format!("{}", 0));
    }
}

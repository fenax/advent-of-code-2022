use crate::formater::*;

pub const FILE: usize = 3;
type Int = u64;
type Data = Vec<(u64, u64)>;

pub fn run() -> Result<(), std::io::Error> {
    print_single_parse(FILE, parse_input, part_1, part_2)
}

fn parse_input(input: &str) -> Data {
    input
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(str::as_bytes)
        .map(|x| {
            let len = x.len();
            (slice_to_mask(&x[..len / 2]), slice_to_mask(&x[len / 2..]))
        })
        .collect()
}

fn part_1(data: &Data) -> Int {
    data.iter()
        .map(|(a, b)| priority_from_mask(a & b))
        .sum::<u32>() as Int
}

fn part_2(data: &Data) -> Int {
    data.windows(3)
        .step_by(3)
        .map(|group| group.iter().fold(!0u64, |acc, (a, b)| acc & (*a | *b)))
        .map(priority_from_mask)
        .sum::<u32>() as Int
}

fn slice_to_mask(data: &[u8]) -> u64 {
    data.iter()
        .map(to_priority_mask)
        .fold(0u64, |acc, x| (acc | x))
}

fn to_priority(c: &u8) -> u32 {
    (match *c {
        b'a'..=b'z' => *c - b'a' + 1,
        b'A'..=b'Z' => *c - b'A' + 27,
        _ => panic!("invalid item type {} {}", c, *c as char),
    }) as u32
}

fn to_priority_mask(c: &u8) -> u64 {
    1 << to_priority(c)
}

fn priority_from_mask(m: u64) -> u32 {
    m.trailing_zeros()
}

#[cfg(test)]
mod tests {
    use crate::day_03::{parse_input, part_1, part_2, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_example() {
        let input = parse_input(EXEMPLE_1);
        assert_eq!(part_1(&input), 157);
        assert_eq!(part_2(&input), 70);
    }
    #[test]
    fn test() {
        let input = read_file(FILE);
        let data = parse_input(&input);
        assert_eq!(part_1(&data).to_string(), format!("{}", 7967));
        assert_eq!(part_2(&data).to_string(), format!("{}", 2716));
    }
}

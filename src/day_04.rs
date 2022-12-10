use crate::formater::*;

pub const FILE: usize = 4;
type Int = u64;
type Data = Vec<((u8, u8), (u8, u8))>;

pub fn run() -> Result<(), std::io::Error> {
    print_single_parse(FILE, parse_input, part_1, part_2)
}

fn parse_input(input: &str) -> Data {
    input
        .lines()
        .map(|x| {
            let mut x = x
                .split(|x| x == '-' || x == ',')
                .map(str::parse)
                .map(Result::unwrap);
            (
                (x.next().unwrap(), x.next().unwrap()),
                (x.next().unwrap(), x.next().unwrap()),
            )
        })
        .collect()
}

fn part_1(data: &Data) -> Int {
    data.iter()
        .filter(|((a, b), (x, y))| (*a >= *x && *y >= *b) || (*x >= *a && *b >= *y))
        .count() as Int
}

fn part_2(data: &Data) -> Int {
    data.iter()
        .filter(|((a, b), (x, y))| {
            (*a >= *x && *y >= *b)
                || (*x >= *a && *b >= *y)
                || (*a >= *x && *y >= *a)
                || (*b >= *x && *y >= *b)
        })
        .count() as Int
}

#[cfg(test)]
mod tests {
    use crate::day_04::{parse_input, part_1, part_2, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_example() {
        let exemple = vec![
            ((2, 4), (6, 8)),
            ((2, 3), (4, 5)),
            ((5, 7), (7, 9)),
            ((2, 8), (3, 7)),
            ((6, 6), (4, 6)),
            ((2, 6), (4, 8)),
        ];

        assert_eq!(parse_input(EXEMPLE_1), exemple);
        assert_eq!(part_1(&exemple), 2);
        assert_eq!(part_2(&exemple), 4);
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}", 538));
        assert_eq!(part_2(&data).to_string(), format!("{}", 792));
    }
}

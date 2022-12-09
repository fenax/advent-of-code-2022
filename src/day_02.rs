use crate::formater::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Play {
    fn parse(byte: u8) -> Play {
        match byte {
            b'A' | b'X' => Play::Rock,
            b'B' | b'Y' => Play::Paper,
            b'C' | b'Z' => Play::Scissors,
            _ => panic!("Invalid game {} {}", byte as char, byte),
        }
    }
    fn win_against(self) -> Play {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
    fn lose_against(self) -> Play {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
}

fn round_score(other: Play, me: Play) -> Int {
    let ret = (match (me, other) {
        (Play::Rock, Play::Paper)
        | (Play::Paper, Play::Scissors)
        | (Play::Scissors, Play::Rock) => 0,
        (Play::Rock, Play::Rock)
        | (Play::Paper, Play::Paper)
        | (Play::Scissors, Play::Scissors) => 3,
        (Play::Paper, Play::Rock)
        | (Play::Scissors, Play::Paper)
        | (Play::Rock, Play::Scissors) => 6,
    }) + (me as Int);
    //    println!("{:?} {:?} = {}",me,other,ret);
    ret
}

pub const FILE: usize = 2;
type Int = u64;
type Data = Vec<(u8, u8)>;

pub fn run() -> Result<(), std::io::Error> {
    print_single_parse(FILE, parse_input, calculate_score, calculate_score_2)
}

fn parse_input(input: &str) -> Data {
    input
        .lines()
        .map(|round| (round.as_bytes()[0], round.as_bytes()[2]))
        .collect()
}

fn calculate_score(data: &Data) -> Int {
    data.iter()
        .map(|(x, y)| round_score(Play::parse(*x), Play::parse(*y)))
        .sum()
}

fn calculate_score_2(data: &Data) -> Int {
    data.iter()
        .map(|(other, result)| {
            let other = Play::parse(*other);
            (
                other,
                match result {
                    b'X' => other.lose_against(),
                    b'Y' => other,
                    b'Z' => other.win_against(),
                    _ => panic!("invalid result"),
                },
            )
        })
        .map(|(x, y)| round_score(x, y))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_02::{calculate_score, calculate_score_2, parse_input, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "A Y
B X
C Z";

    #[test]
    fn test_example() {
        let data = parse_input(EXEMPLE_1);
        assert_eq!(calculate_score(&data), 15);
        assert_eq!(calculate_score_2(&data), 12);
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(calculate_score(&data).to_string(), format!("{}", 11666));
        assert_eq!(calculate_score_2(&data).to_string(), format!("{}", 12767));
    }
}

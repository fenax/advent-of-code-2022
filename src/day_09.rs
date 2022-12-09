use std::{collections::HashSet, str::FromStr};

use crate::formater::*;

pub const FILE: usize = 9;
type Int = usize;
type Data = Vec<Moves>;

enum Moves {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Knot {
    x: isize,
    y: isize,
}

impl Knot {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    fn mov(&mut self, x: isize, y: isize) {
        self.x += x;
        self.y += y;
    }
    fn follows(&mut self, target: &Knot) {
        let dx = target.x - self.x;
        let dy = target.y - self.y;
        if dx.abs().max(dy.abs()) > 1 {
            self.mov(dx.signum(), dy.signum());
        }
    }
}

impl FromStr for Moves {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dist = s[2..]
            .parse::<isize>()
            .map_err(|e| format!("ERROR:{:?}", e))?;
        match s.bytes().nth(0) {
            Some(b'U') => Ok(Self::Up(dist)),
            Some(b'D') => Ok(Self::Down(dist)),
            Some(b'L') => Ok(Self::Left(dist)),
            Some(b'R') => Ok(Self::Right(dist)),
            _ => Err(format!("invalid input \"{}\"", s)),
        }
    }
}

impl Moves {}

pub fn run() -> Result<(), std::io::Error> {
    print_single_parse(FILE, parse_input, part_1, part_2)
}

fn parse_input(input: &str) -> Data {
    input.lines().map(str::parse).map(Result::unwrap).collect()
}

fn part_1(data: &Data) -> Int {
    let mut head = Knot::new();
    let mut tail = Knot::new();

    let mut visited: HashSet<Knot> = HashSet::new();

    for step in data {
        let (x_mov, y_mov, dist) = match step {
            Moves::Up(dist) => (0, 1, dist),
            Moves::Down(dist) => (0, -1, dist),
            Moves::Left(dist) => (-1, 0, dist),
            Moves::Right(dist) => (1, 0, dist),
        };
        for _ in 0..*dist {
            head.mov(x_mov, y_mov);
            tail.follows(&head);

            visited.insert(tail);
        }
    }
    visited.iter().count()
}

fn part_2(data: &Data) -> Int {
    let mut head = Knot::new();
    let mut rope = vec![Knot::new(); 9];

    let mut visited: HashSet<Knot> = HashSet::new();

    for step in data {
        let (x_mov, y_mov, dist) = match step {
            Moves::Up(dist) => (0, 1, dist),
            Moves::Down(dist) => (0, -1, dist),
            Moves::Left(dist) => (-1, 0, dist),
            Moves::Right(dist) => (1, 0, dist),
        };
        for _ in 0..*dist {
            head.mov(x_mov, y_mov);
            let mut last = head;
            for tail in rope.iter_mut() {
                tail.follows(&last);
                last = *tail;
            }
            visited.insert(last);
        }
    }
    visited.iter().count()
}

#[cfg(test)]
mod tests {
    use crate::day_09::{parse_input, part_1, part_2, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
    static EXEMPLE_2: &'static str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    #[test]
    fn test_example() {
        let exemple = parse_input(EXEMPLE_1);
        assert_eq!(part_1(&exemple), 13);
        let exemple = parse_input(EXEMPLE_2);
        assert_eq!(part_2(&exemple), 36);
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}", 6745));
        assert_eq!(part_2(&data).to_string(), format!("{}", 2793));
    }
}

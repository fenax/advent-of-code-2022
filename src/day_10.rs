use std::str::FromStr;

use itertools::Itertools;

use crate::formater::*;

pub const FILE: usize = 10;
type Int = i32;
type Data = Vec<Instruction>;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(Int),
}

struct InstructionIter<'a, It>
where
    It: Iterator<Item = &'a Instruction>,
{
    instr: &'a mut It,
    wait: Int,
    x: Int,
    next_x: Int,
}

impl<'a, It> Iterator for InstructionIter<'a, It>
where
    It: Iterator<Item = &'a Instruction>,
{
    type Item = Int;

    fn next(&mut self) -> Option<Int> {
        if self.wait <= 0 {
            self.x = self.next_x;
            match self.instr.next() {
                Some(Instruction::Noop) => Some(self.x),
                Some(Instruction::Addx(val)) => {
                    self.next_x += val;
                    self.wait = 1;
                    Some(self.x)
                }
                None => None,
            }
        } else {
            self.wait -= 1;
            Some(self.x)
        }
    }
}

fn iter_cycles<'a, It>(iter: &'a mut It) -> InstructionIter<'a, It>
where
    It: Iterator<Item = &'a Instruction>,
{
    InstructionIter {
        instr: iter,
        wait: 0,
        x: 1,
        next_x: 1,
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line: Vec<_> = s.split_whitespace().collect();
        match line.as_slice() {
            ["addx", param] => Ok(Instruction::Addx(
                param
                    .parse()
                    .map_err(|_| format!("invalid parameter in \"{}\"", s))?,
            )),
            ["noop"] => Ok(Instruction::Noop),
            _ => Err(format!("unknown instruction \"{}\"", s)),
        }
    }
}

pub fn run() -> Result<(), std::io::Error> {
    print_single_parse(FILE, parse_input, part_1, part_2)
}

fn parse_input(input: &str) -> Data {
    input
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect_vec()
}

fn part_1(data: &Data) -> Int {
    let mut input = data.iter();
    let iter = iter_cycles(&mut input).enumerate();
    iter.skip(19) //skip the 19 first
        .step_by(40)
        .map(|(i, x)| (i as Int + 1) * x)
        .sum()
}

fn part_2(data: &Data) -> String {
    let mut input = data.iter();
    let iter = iter_cycles(&mut input).enumerate();
    iter.fold(String::with_capacity(41 * 6), |mut acc, (i, x)| {
        let pixel = (i % 40) as Int;
        if pixel <= (x + 1) && pixel >= (x - 1) {
            acc.push('#');
        } else {
            acc.push(' ');
        }
        if pixel == 39 {
            acc.push('\n');
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::day_10::{parse_input, part_1, part_2, Instruction, FILE};
    use crate::formater::read_file;

    static EXEMPLE_0: &'static str = "noop
addx 3
addx -5
";
    static EXEMPLE_1: &'static str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_example() {
        let exemple = vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ];

        assert_eq!(parse_input(EXEMPLE_0), exemple);
        let exemple = parse_input(EXEMPLE_1);
        assert_eq!(part_1(&exemple), 13140);
        assert_eq!(
            part_2(&exemple),
            "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  \n###   ###   ###   ###   ###   ###   ### \n####    ####    ####    ####    ####    \n#####     #####     #####     #####     \n######      ######      ######      ####\n#######       #######       #######     \n"
        );
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}", 13760));
        assert_eq!(part_2(&data).to_string(), "###  #### #  # ####  ##  ###  #### #### \n#  # #    # #     # #  # #  # #    #    \n#  # ###  ##     #  #    #  # ###  ###  \n###  #    # #   #   #    ###  #    #    \n# #  #    # #  #    #  # #    #    #    \n#  # #    #  # ####  ##  #    #### #    \n");
    }
}

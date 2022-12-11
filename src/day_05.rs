use crate::formater::*;

pub const FILE: usize = 5;
type Int = String;
type Data = (Vec<Vec<char>>, Vec<(u8, u8, u8)>);

pub fn run(filename: Option<String>) -> Result<(), std::io::Error> {
    print_single_parse(FILE, filename, parse_input, part_1, part_2)
}

fn parse_input(input: &str) -> Data {
    let v: Vec<&str> = input.split("\n\n").collect();
    let mut stacks: Vec<&str> = v[0].lines().collect();
    let columns: Vec<_> = stacks
        .pop()
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c.is_whitespace() { None } else { Some(i) })
        .collect();
    let mut out = vec![Vec::new(); columns.len()];

    stacks.reverse();
    for l in stacks {
        for (i, s) in columns.iter().enumerate() {
            if let Some(item) = l.chars().nth(*s) {
                if item.is_alphanumeric() {
                    out[i].push(item);
                }
            }
        }
    }
    (
        out,
        v[1].lines()
            .map(|l| {
                let words: Vec<&str> = l.split_ascii_whitespace().collect();
                (
                    words[1].parse().unwrap(),
                    words[3].parse().unwrap(),
                    words[5].parse().unwrap(),
                )
            })
            .collect(),
    )
}

fn part_1(data: &Data) -> Int {
    let mut stacks = data.0.clone();
    //move 2 from 8 to 2
    for (count, from, to) in data.1.iter() {
        for _ in 0..*count {
            let item = stacks[*from as usize - 1].pop().unwrap();
            stacks[*to as usize - 1].push(item);
        }
    }
    stacks
        .iter_mut()
        .map(Vec::pop)
        .map(|x| x.unwrap_or(' '))
        .collect()
}

fn part_2(data: &Data) -> Int {
    let mut stacks = data.0.clone();
    //move 2 from 8 to 2
    for (count, from, to) in data.1.iter() {
        let from = *from as usize - 1;
        let to = *to as usize - 1;
        let len = stacks[from].len();
        let mut temp = stacks[from].split_off(len - (*count as usize));
        stacks[to].append(&mut temp);
    }
    stacks
        .iter_mut()
        .map(Vec::pop)
        .map(|x| x.unwrap_or(' '))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day_05::{parse_input, part_1, part_2, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_example() {
        let exemple = (
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)],
        );

        assert_eq!(parse_input(EXEMPLE_1), exemple);
        assert_eq!(part_1(&exemple), "CMZ");
        assert_eq!(part_2(&exemple), "MCD");
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), "SPFMVDTZT");
        assert_eq!(part_2(&data).to_string(), "ZFSJBPRFP");
    }
}

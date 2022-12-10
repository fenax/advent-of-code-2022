use crate::formater::*;
use colored::Colorize;
use gen_iter::gen_iter;
use itertools::Itertools;

pub const FILE: usize = 8;
type Int = usize;
type Data = Map;

struct Map {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

fn indexer(start: isize, step: isize, limit: isize) -> impl Iterator<Item = usize> {
    gen_iter!(move {
        let mut current = start;
        loop {
            if current >= limit || current < 0 {
                return;
            }
            yield current as usize;
            current += step;
        }
    })
}

impl Map {
    #[allow(dead_code)]
    fn print(&self, visible: &Vec<bool>) {
        for line in 0..self.height {
            for i in line * self.width..(line + 1) * (self.width) {
                let c = format!("{}", self.data[i] as char);
                print!("{}", if visible[i] { c.green() } else { c.red() });
            }
            println!("");
        }
    }
}

pub fn run() -> Result<(), std::io::Error> {
    print_single_parse(FILE, parse_input, part_1, part_2)
}

fn parse_input(input: &str) -> Data {
    let width = input.lines().next().unwrap().len();

    let buffer = input.lines().map(|x| x.bytes()).flatten().collect_vec();
    let height = buffer.len() / width;
    Map {
        width,
        height,
        data: buffer,
    }
}

fn part_1(data: &Data) -> Int {
    fn do_line(map: &Data, iter: impl Iterator<Item = usize>, visible: &mut Vec<bool>) {
        let mut last = b'0' - 1;
        for i in iter {
            let cur = map.data[i];
            if cur > last {
                visible[i] = true;
                last = cur;
            }
        }
    }

    let mut visible_map = vec![false; data.width * data.height];
    let line_offset = data.width as isize;
    let limit = (data.width * data.height) as isize;

    for line in 0..(data.height as isize) {
        do_line(
            data,
            indexer(line_offset * line, 1, limit).take(data.width),
            &mut visible_map,
        );
        do_line(
            data,
            indexer(line_offset * line + data.width as isize - 1, -1, limit).take(data.width),
            &mut visible_map,
        );
    }
    for column in 0..(data.width as isize) {
        do_line(
            data,
            indexer(column, line_offset, limit).take(data.width),
            &mut visible_map,
        );
        do_line(
            data,
            indexer(
                column + (line_offset * (data.height as isize - 1)) - 1,
                -line_offset,
                limit,
            ),
            &mut visible_map,
        );
    }

    visible_map.iter().filter(|x| **x).count()
}

fn part_2(data: &Data) -> Int {
    fn do_line(init: u8, map: &Data, mut iter: impl Iterator<Item = usize>) -> usize {
        let mut count = 0;
        iter.next(); // skip first element
        for i in iter {
            let cur = map.data[i];
            count += 1;
            if cur >= init {
                break;
            }
        }
        count
    }

    let line_offset = data.width;

    let mut max = 0;
    for line in 0..data.height {
        for column in 0..data.width {
            let cells_left = column + 1;
            let cells_right = data.width - column;

            let cells_up = line + 1;
            let cells_down = data.height - line;

            let limit = (data.height * data.width) as isize;

            let init = data.data[line * data.width + column];

            let cell_index = (line_offset * line + column) as isize;

            let right = do_line(init, data, indexer(cell_index, 1, limit).take(cells_right));
            let left = do_line(init, data, indexer(cell_index, -1, limit).take(cells_left));

            let down = do_line(
                init,
                data,
                indexer(cell_index, line_offset as isize, limit).take(cells_down),
            );
            let up = do_line(
                init,
                data,
                indexer(cell_index, -(line_offset as isize), limit).take(cells_up),
            );
            max = max.max(right * left * down * up);
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use crate::day_08::{parse_input, part_1, part_2, FILE};
    use crate::formater::read_file;

    static EXEMPLE_1: &'static str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test_example() {
        let exemple = parse_input(EXEMPLE_1);
        assert_eq!(part_1(&exemple), 21);
        assert_eq!(part_2(&exemple), 8);
    }
    #[test]
    fn test() {
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}", 1782));
        assert_eq!(part_2(&data).to_string(), format!("{}", 474606));
    }
}

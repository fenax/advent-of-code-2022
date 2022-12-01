use itertools::{izip, Itertools};
use crate::formater::*;
use crate::parser::*;

type Data = Vec<Vec<i64>>;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(1,
        parse_input,
        sum_per_elf,
        top_three_elves)
}

fn parse_input(input:&str) -> Data{
    separated_by_blank(input).map(|elf|{
        one_int_per_line(elf)
    }).collect()
}

fn sum_per_elf(data: &Vec<Vec<i64>>)->i64{
    data.iter().map(|elf|{elf.iter().sum()}).max().unwrap_or(0)
}

fn top_three_elves(data: &Vec<Vec<i64>>)->i64{
    let mut elves : Vec<i64> = data.iter().map(|elf|{elf.iter().sum()}).collect_vec();
    elves.sort();
    
    let end = elves.len();
    elves[end-1] + elves[end-2] + elves[end-3]
}



fn count_increases(data:&Vec<i64>)->i64{
    let mut iter1 = data.iter();
    iter1.next();
    iter1.zip(data.iter()).filter(|(a,b)| {a>b}).count() as i64
}

fn rolling_sum(data:&Vec<i64>)->Vec<i64>{
    let iter0 = data.iter();
    let mut iter1 = data.iter();
    let mut iter2 = data.iter();
    iter1.next();
    iter2.next();
    iter2.next();

    izip!(iter0,iter1,iter2).map(|(a,b,c)|{a+b+c}).collect_vec()
}

fn part2(data:&Vec<Vec<i64>>)->i64{
    0
}

#[cfg(test)]
mod tests {
    use crate::parser::one_int_per_line;
    use crate::formater::read_file;
    use crate::day_01::{sum_per_elf,parse_input, top_three_elves};


    static EXEMPLE_1:&'static str = 
"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_example() {
        let EXEMPLE = vec![vec![1000,2000,3000],vec![4000],vec![5000,6000],vec![7000,8000,9000],vec![10000]];

        assert_eq!(parse_input(EXEMPLE_1),EXEMPLE);
        assert_eq!(sum_per_elf(&EXEMPLE), 24000);
        assert_eq!(top_three_elves(&EXEMPLE),45000)
    }
    #[test]
    fn test(){
        let data = parse_input(&read_file(1));
        assert_eq!(sum_per_elf(&data).to_string(), format!("{}",70613));
        assert_eq!(top_three_elves(&data).to_string(), format!("{}",205805));
    }
}

use itertools::{Itertools};
use crate::formater::*;
use crate::parser::*;
use rayon::prelude::*;


type Int = u64;
type Data = Vec<Vec<Int>>;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(1,
        parse_input,
        sum_per_elf,
        top_three_elves)
}
/* 
pub fn versions() -> Result<(), std::io::Error>{
    print_single_parse(1,
        parse_input_rayon,
        sum_per_elf_rayon,
        top_three_elves_rayon)
}
*/



fn parse_input(input:&str) -> Data{
    input.split("\n\n").map(
        |elf|{
            elf.trim().split("\n").map(str::parse).filter_map(Result::ok).collect()
        }
    ).collect()
}

fn sum_per_elf(data: &Data)->Int{
    data.iter().map(|elf|{elf.iter().sum()}).max().unwrap_or(0)
}

fn top_three_elves(data: &Data)->Int{
    let mut elves : Vec<Int> = data.iter().map(|elf|{elf.iter().sum()}).collect();
    elves.sort();
    
    let end = elves.len();
    elves[end-1] + elves[end-2] + elves[end-3]
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
        let EXEMPLE = vec![
            vec![1000,2000,3000],
            vec![4000],
            vec![5000,6000],
            vec![7000,8000,9000],
            vec![10000]];

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

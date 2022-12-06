use std::ops::BitOr;

use itertools::Itertools;

use crate::formater::*;

pub const FILE:usize = 6;
type Int = usize;
type Data = Vec<u8>;

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(FILE,
        parse_input,
        part_1,
        part_2)
}

fn parse_input(input:&str) -> Data{
    input.bytes().filter(|&v|v>=b'a' && v<=b'z').collect()
}

fn part_1(data: &Data)->Int{
    data.iter().map(to_mask).collect_vec().windows(4).map(|v|{
        v.iter().fold(0,BitOr::bitor)
    }).take_while(|v|v.count_ones() < 4).count() +4
}

fn part_2(data: &Data)->Int{
    data.iter().map(to_mask).collect_vec().windows(14).map(|v|{
        v.iter().fold(0,BitOr::bitor)
    }).take_while(|v|v.count_ones() < 14).count() +14
}

fn to_mask(data:&u8)->u32{
    1<<(data-b'a')
}

#[cfg(test)]
mod tests {
    use crate::formater::read_file;
    use crate::day_06::{FILE,parse_input, part_1, part_2};


    #[test]
    fn test_example() {
        let exemple = 
        vec![("mjqjpqmgbljsphdztnvjfqwrcgsmlb",7,19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz",5,23),
        ("nppdvjthqldpwncqszvftbrmjlhg",6,23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",10,29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",11,26)];
        
        for (input,result1,result2) in exemple{
            let ex = input.bytes().collect();
            //assert_eq!(parse_input(EXEMPLE_1),exemple);
            assert_eq!(part_1(&ex), result1);
            assert_eq!(part_2(&ex), result2);

        }
    }
    #[test]
    fn test(){
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}",1848));
        assert_eq!(part_2(&data).to_string(), format!("{}",2308));
    }
}

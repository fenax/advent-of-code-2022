use crate::formater::*;

pub const FILE:usize = 1;
type Int = u64;
type Data = ();

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(FILE,
        parse_input,
        part_1,
        part_2)
}

fn parse_input(input:&str) -> Data{
    ()
}

fn part_1(data: &Data)->Int{
    0
}

fn part_2(data: &Data)->Int{
    0
}

#[cfg(test)]
mod tests {
    use crate::formater::read_file;
    use crate::day_template::{FILE,parse_input, part_1, part_2};


    static EXEMPLE_1:&'static str = 
"data";

    #[test]
    fn test_example() {
        let exemple = ();

        assert_eq!(parse_input(EXEMPLE_1),exemple);
        assert_eq!(part_1(&exemple), 0);
        assert_eq!(part_2(&exemple), 0);
    }
    #[test]
    fn test(){
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}",0));
        assert_eq!(part_2(&data).to_string(), format!("{}",0));
    }
}

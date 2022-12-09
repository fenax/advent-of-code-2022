use std::{str::FromStr, collections::HashSet};

use crate::formater::*;

pub const FILE:usize = 9;
type Int = usize;
type Data = Vec<Moves>;

enum Moves{
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

#[derive(Debug,Hash,PartialEq, Eq,Clone, Copy)]
struct Knot{
    x:isize,
    y:isize,
}

impl Knot{
    fn new()->Self{
        Self { x: 0, y: 0 }
    }
    fn mov(&mut self, x:isize, y:isize){
        self.x += x;
        self.y += y;
    }
    fn follows(&mut self,target:&Knot){
        if target.x==self.x{
            let diff = target.y-self.y;
            if diff.abs() >1{
                self.y += diff.signum();
            }
        }else if target.y==self.y{
            let diff = target.x-self.x;
            if diff.abs() >1{
                self.x += diff.signum();
            }
        }else if (target.x-self.x).abs().max((target.y-self.y).abs()) > 1{
            self.x += (target.x-self.x).signum();
            self.y += (target.y-self.y).signum();
        }
    }
}

impl FromStr for Moves {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let dist = s[2..].parse::<isize>().map_err(|e|format!("ERROR:{:?}",e))?;
        match s.bytes().nth(0) {
            Some(b'U') => Ok(Self::Up(dist)),
            Some(b'D') => Ok(Self::Down(dist)),
            Some(b'L') => Ok(Self::Left(dist)),
            Some(b'R') => Ok(Self::Right(dist)),
            _ => Err(format!("invalid input \"{}\"",s))
        }
    }
}

impl Moves{
    
}

pub fn run()-> Result<(), std::io::Error>{
    print_single_parse(FILE,
        parse_input,
        part_1,
        part_2)
}

fn parse_input(input:&str) -> Data{
    input.lines().map(str::parse).map(Result::unwrap).collect()
}

fn part_1(data: &Data)->Int{
    /*let mut x = 0isize;
    let mut y = 0isize;
    let mut max_x = 0isize;
    let mut min_x = 0isize;
    let mut max_y = 0isize;
    let mut min_y = 0isize;
    for step in data{
        match step {
            Moves::Up(dist) => y += dist,
            Moves::Down(dist) => y -= dist,
            Moves::Left(dist) => x -= dist,
            Moves::Right(dist) => x+=dist,
        }
        max_x = max_x.max(x);
        min_x = min_x.min(x);
        max_y = max_y.max(y);
        min_y = min_y.min(y);
    }*/
    let mut head = Knot::new();
    let mut tail = Knot::new();
    /* 
    let mut x = 0isize;
    let mut y = 0isize;
    let mut tail_x = 0isize;
    let mut tail_y = 0isize;
    */

    //let mut visited:HashSet<(isize,isize)> = HashSet::new();
    let mut visited:HashSet<Knot> = HashSet::new();

    for step in data{
        let (x_mov, y_mov, dist) = 
            match step{
                Moves::Up(dist) => (0, 1,dist),
                Moves::Down(dist) => (0, -1, dist),
                Moves::Left(dist) => (-1,0,dist),
                Moves::Right(dist) => (1,0,dist),
            };
        for _ in 0..*dist{
            head.mov(x_mov, y_mov);
            tail.follows(&head);
            //x+=x_mov;
            //y+=y_mov;
            visited.insert(tail);
        }
    }
    //println!("X : {}-{} Y : {}-{}",min_x,max_x,min_y,max_y);
    visited.iter().count()
}

fn part_2(data: &Data)->Int{
    let mut head = Knot::new();
    //let mut tail = Knot::new();
    /* 
    let mut x = 0isize;
    let mut y = 0isize;
    let mut tail_x = 0isize;
    let mut tail_y = 0isize;
    */

    //let mut visited:HashSet<(isize,isize)> = HashSet::new();
    let mut rope = vec![Knot::new();9];
    let mut visited:HashSet<Knot> = HashSet::new();

    for step in data{
        let (x_mov, y_mov, dist) = 
            match step{
                Moves::Up(dist) => (0, 1,dist),
                Moves::Down(dist) => (0, -1, dist),
                Moves::Left(dist) => (-1,0,dist),
                Moves::Right(dist) => (1,0,dist),
            };
        for _ in 0..*dist{
            head.mov(x_mov, y_mov);
            let mut last = head;
            for tail in rope.iter_mut(){
                tail.follows(&last);
                last = *tail;

            }
            //x+=x_mov;
            //y+=y_mov;
            visited.insert(last);
        }
    }
    //println!("X : {}-{} Y : {}-{}",min_x,max_x,min_y,max_y);
    visited.iter().count()

}

#[cfg(test)]
mod tests {
    use crate::formater::read_file;
    use crate::day_09::{FILE,parse_input, part_1, part_2};


    static EXEMPLE_1:&'static str = 
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
static EXEMPLE_2:&'static str = 
"R 5
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
    fn test(){
        let data = parse_input(&read_file(FILE));
        assert_eq!(part_1(&data).to_string(), format!("{}",6745));
        assert_eq!(part_2(&data).to_string(), format!("{}",2793));
    }
}

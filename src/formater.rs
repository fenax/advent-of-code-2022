use colored::*;
use std::fs::read_to_string;
use std::time::Instant;

pub fn timed_run<F, T>(mut f: F) -> T
where
    F: FnMut() -> T,
{
    let start = Instant::now();
    let x = f();
    let duration = start.elapsed();
    println!("{}", format!("  Timing : {:?}", duration).dimmed());
    x
}

#[allow(dead_code)]
pub fn read_file(num: usize) -> String {
    read_to_string(format!("day_{:02}.input", num)).unwrap()
}

pub fn print_single_parse<F, G, H, T, U, V>(
    num: usize,
    filename: Option<String>,
    mut parser: F,
    mut part1: G,
    mut part2: H,
) -> Result<(), std::io::Error>
where
    F: FnMut(&str) -> T,
    G: FnMut(&T) -> U,
    H: FnMut(&T) -> V,
    U: std::fmt::Display,
    V: std::fmt::Display,
{
    print_header(num);
    let file_data = read_to_string(if let Some(file) = filename {
        file
    } else {
        format!("day_{:02}.input", num)
    })?;
    print_parse();
    let data = timed_run(|| parser(&file_data));
    print_part_1();
    let result1 = timed_run(|| part1(&data));
    print_result(&result1);
    print_part_2();
    let result2 = timed_run(|| part2(&data));
    print_result(&result2);
    Ok(())
}

fn print_header(num: usize) {
    println!("{}", format!("Day {:2}", num).bold());
}

fn print_parse() {
    print!("   {}", "parsing".green().dimmed());
}

fn print_part_1() {
    print!("    {}", "part 1".green());
}

fn print_part_2() {
    print!("    {}", "part 2".green());
}

fn print_result<T: std::fmt::Display>(result: &T) {
    let str = result.to_string();
    str.lines().for_each(|l| {
        println!("     {}", l);
    });
}

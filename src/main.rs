#![feature(generators)]

use colored::Colorize;
use std::time::Instant;

#[allow(dead_code)]
mod formater;
#[allow(dead_code)]
mod parser;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;

fn main() -> Result<(), std::io::Error> {
    let start = Instant::now();

    day_01::run()?;
    day_02::run()?;
    day_03::run()?;
    day_04::run()?;
    day_05::run()?;
    day_06::run()?;
    day_07::run()?;
    day_08::run()?;
    day_09::run()?;
    day_10::run()?;

    let duration = start.elapsed();
    println!("{}", format!("  Timing : {:?}", duration).dimmed());

    Ok(())
}

#![feature(generators)]
use clap::Parser;
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
mod day_11;
mod day_12;

/// Program to run Advent Of Code 2022
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run a specific day
    #[arg(short, long)]
    day: Option<u8>,

    /// Specify a file name, require a specified day
    #[arg(short, long)]
    file: Option<String>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let start = Instant::now();

    match args.day {
        Some(01) => day_01::run(args.file)?,
        Some(02) => day_02::run(args.file)?,
        Some(03) => day_03::run(args.file)?,
        Some(04) => day_04::run(args.file)?,
        Some(05) => day_05::run(args.file)?,
        Some(06) => day_06::run(args.file)?,
        Some(07) => day_07::run(args.file)?,
        Some(08) => day_08::run(args.file)?,
        Some(09) => day_09::run(args.file)?,
        Some(10) => day_10::run(args.file)?,
        Some(11) => day_11::run(args.file)?,
        Some(12) => day_12::run(args.file)?,

        Some(_) => {
            println!("Invalid day");
        }

        None => {
            day_01::run(None)?;
            day_02::run(None)?;
            day_03::run(None)?;
            day_04::run(None)?;
            day_05::run(None)?;
            day_06::run(None)?;
            day_07::run(None)?;
            day_08::run(None)?;
            day_09::run(None)?;
            day_10::run(None)?;
            day_11::run(None)?;
            day_12::run(None)?;
        }
    }

    let duration = start.elapsed();
    println!("{}", format!("  Timing : {:?}", duration).dimmed());

    Ok(())
}

#[allow(dead_code)]
mod formater;
#[allow(dead_code)]
mod parser;

mod day_01;
mod day_02;

fn main() -> Result<(), std::io::Error>  {
    day_01::run()?;
    day_02::run()?;
    Ok(())
}

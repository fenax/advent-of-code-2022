#[allow(dead_code)]
mod formater;
#[allow(dead_code)]
mod parser;

mod day_01;


fn main() -> Result<(), std::io::Error>  {
    day_01::run()?;
    Ok(())
}

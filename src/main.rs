mod formater;
mod parser;

mod day_01;


fn main() -> Result<(), std::io::Error>  {
    day_01::run()?;
    Ok(())
}

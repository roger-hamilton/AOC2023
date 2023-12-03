pub mod part1;
pub mod part2;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub type Input<'a> = Vec<&'a str>;

pub fn parse_input(input: &str) -> Result<Input> {
    Ok(input.lines().collect())
}

pub mod part1;
pub mod part2;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Input {}

pub fn parse_input(input: &str) -> Result<Input> {
    todo!()
}

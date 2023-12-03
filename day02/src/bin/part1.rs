use day02::{part1::process, Result};

fn main() -> Result<()> {
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("Part 1: {}", result);
    Ok(())
}

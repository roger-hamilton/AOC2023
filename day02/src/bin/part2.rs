use day02::{part2::process, Result};

fn main() -> Result<()> {
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("Part 2: {}", result);
    Ok(())
}

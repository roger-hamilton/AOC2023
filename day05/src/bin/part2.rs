use std::time::Instant;

use day05::{part2::process, Result};

fn main() -> Result<()> {
    let file = include_str!("../../input.txt");
    println!("Starting Part 2");
    let now = Instant::now();
    {
        let result = process(file)?;
        println!("Part 2: {}", result);
    }
    println!("  Time: {:?}", now.elapsed());
    Ok(())
}

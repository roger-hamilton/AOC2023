use std::time::Instant;

use day04::{part2::process, Result};

fn main() -> Result<()> {
    let file = include_str!("../../input.txt");
    let now = Instant::now();
    {
        let result = process(file)?;
        println!("Part 2: {}", result);
    }
    println!("  Time: {:?}", now.elapsed());
    Ok(())
}

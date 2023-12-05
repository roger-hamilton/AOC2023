use std::time::Instant;

use {{crate_name}}::{part1::process, Result};

fn main() -> Result<()> {
    let file = include_str!("../../input.txt");
    let now = Instant::now();
    {
        let result = process(file)?;
        println!("Part 1: {}", result);
    }
    println!("  Time: {:?}", now.elapsed());
    Ok(())
}
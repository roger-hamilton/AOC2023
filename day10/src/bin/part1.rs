use std::time::Instant;

use day10::{part1::process, Result};


#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let file = include_str!("../../input.txt");
    let now = Instant::now();
    {
        let result = process(file)?;
        println!("Part 1: {}", result);
    }
    println!("  Time: {:?}", now.elapsed());
    Ok(())
}

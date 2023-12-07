use day05::{part2::process, Result};
use std::time::Instant;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

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

use std::time::Instant;

use {{crate_name}}::{part2::process, Result};

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
        println!("Part 2: {}", result);
    }
    println!("  Time: {:?}", now.elapsed());
    Ok(())
}

use glam::IVec2;
use itertools::Itertools;

use crate::{parse_input, Result};

static OFFSETS: [IVec2; 8] = [
    IVec2::new(-1, -1),
    IVec2::new(0, -1),
    IVec2::new(1, -1),
    IVec2::new(-1, 0),
    IVec2::new(1, 0),
    IVec2::new(-1, 1),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
];

pub fn process(input: &str) -> Result<u32> {
    let input = parse_input(input)?;

    Ok(input
        .symbols
        .iter()
        .flat_map(|(pos, _)| OFFSETS.iter().map(|offset| *offset + *pos))
        .filter_map(|pos| input.numbers.iter().find(move |n| n.pos == pos))
        .unique_by(|n| n.id)
        .map(|n| n.num)
        .sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "#};
        assert_eq!(4361, process(input.trim())?);
        Ok(())
    }

    #[test]
    fn test_process2() -> Result<()> {
        let input = indoc! {r#"
        467..114..
        ...*......
        ..35..633.
        ......#123
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "#};
        assert_eq!(4361 + 123, process(input.trim())?);
        Ok(())
    }
}

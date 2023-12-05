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
        // only `*`s are gears
        .filter_map(|(pos, c)| if *c == '*' { Some(*pos) } else { None })
        .map(|pos| {
            OFFSETS
                .iter()
                .filter_map(|offset| input.numbers.iter().find(move |n| n.pos == *offset + pos))
                .unique_by(|n| n.id)
                .map(|n| n.num)
                .collect_vec()
        })
        // gears only have 2 parts
        .filter(|v| v.len() == 2)
        .map(|nums| nums.iter().copied().product::<u32>())
        .sum())
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
        assert_eq!(467835, process(input.trim())?);
        Ok(())
    }
}

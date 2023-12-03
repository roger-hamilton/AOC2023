use itertools::Itertools;

use crate::{parse_input, Result};

pub fn process(input: &str) -> Result<u32> {
    let input = parse_input(input)?;

    let result = input
        .iter()
        .map(|game| {
            let all_cubes = game.cubes.iter().flatten();
            let max_by_color = all_cubes
                .into_grouping_map_by(|c| c.color)
                .fold(0, |acc, _, val| acc.max(val.count));
            max_by_color.values().product::<u32>()
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#};
        assert_eq!(2286, process(input.trim())?);
        Ok(())
    }
}

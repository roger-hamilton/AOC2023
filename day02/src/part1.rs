use crate::{parse_input, Result};

fn max_by_color(color: &str) -> u32 {
    match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => 0,
    }
}

pub fn process(input: &str) -> Result<u32> {
    let input = parse_input(input)?;

    let result = input
        .iter()
        .filter_map(|game| {
            let mut all_cubes = game.cubes.iter().flatten();
            if all_cubes.any(|c| c.count > max_by_color(c.color)) {
                return None;
            }
            Some(game.id)
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
        assert_eq!(8, process(input.trim())?);
        Ok(())
    }
}

use nom::{
    bytes::complete::is_not,
    character::{self, complete::space1},
    multi::separated_list0,
    IResult, Parser,
};
use nom_supreme::ParserExt;

use crate::Result;

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    is_not("0123456789")
        .precedes(separated_list0(space1, character::complete::u32))
        .parse(input)
}

fn parse_input(input: &str) -> Result<Vec<(u32, u32)>> {
    let (input, times) = parse_line(input).map_err(|_| "Failed to Parse Line")?;
    let (_, distances) = parse_line(input).map_err(|_| "Failed to Parse Line")?;

    Ok(times.into_iter().zip(distances).collect())
}

pub fn process(input: &str) -> Result<usize> {
    let input = parse_input(input)?;

    Ok(input
        .iter()
        .map(|(t, d)| {
            (0..*t)
                .filter_map(move |held| {
                    if held * (t - held) > *d {
                        Some(())
                    } else {
                        None
                    }
                })
                .count()
        })
        .product::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        Time:      7  15   30
        Distance:  9  40  200
        "#};
        assert_eq!(288, process(input.trim())?);
        Ok(())
    }
}

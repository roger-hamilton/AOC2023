use crate::Result;
use itertools::Itertools;
use nom::{
    character::{
        self,
        complete::{line_ending, space1},
    },
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
pub struct Input {
    histories: Vec<Vec<i64>>,
}

fn parse_history(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, history) = separated_list1(
        space1::<&str, nom::error::Error<&str>>,
        character::complete::i64,
    )(input)
    .unwrap();
    Ok((input, history))
}

pub fn parse_input(input: &str) -> Result<Input> {
    let (_, histories) = separated_list1(line_ending, parse_history)(input).unwrap();

    Ok(Input { histories })
}

fn find_next(history: &[i64]) -> i64 {
    assert!(history.len() > 0);
    if history.len() == 1 {
        return history[0];
    }

    let paired = history.iter().tuple_windows();

    if paired.clone().all(|(a, b)| a == b) {
        return history[0];
    }

    find_next(&paired.map(|(a, b)| b - a).collect_vec()) + history.iter().last().unwrap()
}

pub fn process(input: &str) -> Result<i64> {
    let input = parse_input(input)?;

    Ok(input.histories.iter().map(|h| find_next(h)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
        "#};
        assert_eq!(114, process(input.trim())?);
        Ok(())
    }

    #[rstest]
    #[case(vec![0, 3, 6, 9, 12, 15], 18)]
    #[case(vec![1, 3, 6, 10, 15, 21], 28)]
    #[case(vec![10, 13,16,21,30,45], 68)]
    fn test_find_next(#[case] history: Vec<i64>, #[case] expected: i64) {
        assert_eq!(expected, find_next(&history));
    }
}

use crate::Result;
use itertools::Itertools;

#[derive(Debug)]
pub struct Input {
    histories: Vec<Vec<i64>>,
}

pub fn parse_input(input: &str) -> Result<Input> {
    let histories = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    Ok(Input { histories })
}

fn find_prev(history: &[i64]) -> i64 {
    if history.len() == 1 {
        return history[0];
    }

    let diffs = history
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    if diffs.iter().all(|a| a == &0) {
        history[0]
    } else {
        history[0] - find_prev(&diffs)
    }
}

pub fn process(input: &str) -> Result<i64> {
    let input = parse_input(input)?;

    Ok(input.histories.iter().map(|h| find_prev(h)).sum())
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
        assert_eq!(2, process(input.trim())?);
        Ok(())
    }

    #[rstest]
    #[case(vec![ 0, 3,  6,  9,  12, 15], -3)]
    #[case(vec![ 1, 3,  6,  10, 15, 21],  0)]
    #[case(vec![10, 13, 16, 21, 30, 45],  5)]
    fn test_find_next(#[case] history: Vec<i64>, #[case] expected: i64) {
        assert_eq!(expected, find_prev(&history));
    }
}

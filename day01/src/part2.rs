use crate::{parse_input, Result};
use std::iter::successors;

fn process_line(line: &str) -> u32 {
    let mut it = successors(Some(line), |s| s.get(1..)).filter_map(|str| {
        let next_as_digit = str.chars().next().and_then(|c| c.to_digit(10));
        if let val @ Some(..) = next_as_digit {
            val
        } else if str.starts_with("one") {
            Some(1)
        } else if str.starts_with("two") {
            Some(2)
        } else if str.starts_with("three") {
            Some(3)
        } else if str.starts_with("four") {
            Some(4)
        } else if str.starts_with("five") {
            Some(5)
        } else if str.starts_with("six") {
            Some(6)
        } else if str.starts_with("seven") {
            Some(7)
        } else if str.starts_with("eight") {
            Some(8)
        } else if str.starts_with("nine") {
            Some(9)
        } else {
            None
        }
    });
    let first = it.next().expect("should be a number");
    let last = it.last().unwrap_or(first);
    first * 10 + last
}

pub fn process(input: &str) -> Result<u32> {
    let input = parse_input(input)?;
    let sum_lines = input.iter().map(|l| process_line(l)).sum();
    Ok(sum_lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#};
        assert_eq!(281, process(input)?);
        Ok(())
    }
}

use crate::{parse_input, Result};

fn process_line(line: &str) -> u32 {
    let mut it = line.chars().filter_map(|c| c.to_digit(10));
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
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "#};
        assert_eq!(142, process(input.trim())?);
        Ok(())
    }
}

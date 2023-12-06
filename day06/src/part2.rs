use crate::Result;

fn parse_line(input: &str) -> usize {
    input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0usize, |acc, d| acc * 10 + (d as usize))
}

fn parse_input(input: &str) -> Result<(usize, usize)> {
    let mut lines = input.lines();

    let time = parse_line(lines.next().ok_or("EOF")?);
    let distance = parse_line(lines.next().ok_or("EOF")?);

    Ok((time, distance))
}

pub fn process(input: &str) -> Result<usize> {
    let (time, distance) = parse_input(input)?;

    let count = (0..time)
        .filter_map(|held| {
            if held * (time - held) > distance {
                Some(())
            } else {
                None
            }
        })
        .count();
    Ok(count)
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
        assert_eq!(71503, process(input.trim())?);
        Ok(())
    }
}

use crate::Result;

#[derive(Debug)]
pub struct Input {}

pub fn parse_input(input: &str) -> Result<Input> {
    todo!()
}

pub fn process(input: &str) -> Result<u32> {
    let input = parse_input(input)?;

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        
        "#};
        assert_eq!(13, process(input.trim())?);
        Ok(())
    }
}

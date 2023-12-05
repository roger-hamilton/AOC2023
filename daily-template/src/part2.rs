use crate::{parse_input, Result};

pub fn process(input: &str) -> Result<u32> {
    let _input = parse_input(input)?;

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
        assert_eq!(30, process(input.trim())?);
        Ok(())
    }
}

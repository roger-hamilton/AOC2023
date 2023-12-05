use crate::{parse_input, Input, Result};

fn get_location(input: &Input, seed: usize) -> usize {
    let mut source = "seed".to_string();

    let mut target = seed;

    while source != "location" {
        for map in &input.maps {
            if map.from == source {
                target = map.get_destination(target).unwrap();
                source = map.to.clone();
                break;
            }
        }
    }
    target
}

pub fn process(input: &str) -> Result<usize> {
    let input = parse_input(input)?;

    let res = input
        .seeds
        .iter()
        .map(|s| get_location(&input, *s))
        .min()
        .unwrap();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
        "#};

        let inp = parse_input(input.trim())?;

        assert_eq!(82, get_location(&inp, 79));
        assert_eq!(43, get_location(&inp, 14));
        assert_eq!(86, get_location(&inp, 55));
        assert_eq!(35, get_location(&inp, 13));

        assert_eq!(35, process(input.trim())?);
        Ok(())
    }
}

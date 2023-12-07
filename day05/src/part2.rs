use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::*;
pub struct PairedIter<T, I: Iterator<Item = T>> {
    iter: I,
}

impl<T, I: Iterator<Item = T>> Iterator for PairedIter<T, I> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.iter.next()?;
        let second = self.iter.next()?;
        Some((first, second))
    }
}

pub trait Pairable: Iterator {
    fn pairs(self) -> PairedIter<Self::Item, Self>
    where
        Self: Sized,
    {
        PairedIter { iter: self }
    }
}

impl<T> Pairable for std::slice::Iter<'_, T> {}

pub fn process(input: &str) -> Result<usize> {
    let input = parse_input(input)?;

    let result = input
        .seeds
        .iter()
        .pairs()
        .par_bridge()
        .map(|(start, len)| *start..(*start + len))
        .flat_map(|range| range.clone())
        .map(|s| {
            input
                .maps
                .iter()
                .fold(s, |acc, map| map.get_destination(acc).expect("Invalid map"))
        })
        .min();

    result.ok_or("No result found".into())
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

        assert_eq!(46, process(input.trim())?);
        Ok(())
    }
}

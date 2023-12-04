use std::collections::BTreeMap;

use crate::{parse_input, Card, Result};

fn calc_matches(card: &Card) -> u32 {
    card.nums
        .iter()
        .filter(|n| card.winners.contains(n))
        .count() as u32
}

pub fn process(input: &str) -> Result<u32> {
    let input = parse_input(input)?;

    let mut card_counts = input
        .cards
        .iter()
        .map(|c| (c.id, 1))
        .collect::<BTreeMap<_, _>>();

    for card in input.cards {
        let matches = calc_matches(&card);
        if let Some(count) = card_counts.get(&card.id).copied() {
            for i in 1..=matches {
                card_counts.entry(card.id + i).and_modify(|c| *c += count);
            }
        }
    }
    Ok(card_counts.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#};
        assert_eq!(30, process(input.trim())?);
        Ok(())
    }
}

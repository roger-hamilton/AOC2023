use itertools::Itertools;

use crate::{parse_input, Hand, Result};

fn compare(a: &&Hand, b: &&Hand) -> std::cmp::Ordering {
    let ar = a.rank();
    let br = b.rank();

    match ar.cmp(&br) {
        std::cmp::Ordering::Equal => {
            for i in 0..a.cards.len() {
                if a.cards[i] != b.cards[i] {
                    return a.cards[i].cmp(&b.cards[i]);
                }
            }
            unreachable!("Hands cannot be equal")
        }
        o => o,
    }
}

pub fn process(input: &str) -> Result<u64> {
    let input = parse_input(input)?;

    Ok(input
        .hands
        .iter()
        .sorted_by(compare)
        .map(|h| h.bid)
        .enumerate()
        .map(|(i, r)| (i + 1) as u64 * r)
        .sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        "#};
        assert_eq!(6440, process(input.trim())?);
        Ok(())
    }
}

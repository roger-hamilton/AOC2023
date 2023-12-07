use itertools::Itertools;

use crate::{parse_input, Card, Hand, Rank, Result};

fn joker_rank(hand: &Hand) -> Rank {
    let starting_rank = hand.rank();
    let joker_count = hand.cards.iter().filter(|&&c| c == Card(0)).count();

    match joker_count {
        0 => starting_rank,
        1 => match starting_rank {
            // JABCD -> AABCD
            Rank::HighCard => Rank::Pair,
            // JAABC -> AAABC
            Rank::Pair => Rank::ThreeOfAKind,
            // JAABB -> AAABB
            Rank::TwoPair => Rank::FullHouse,
            // JAAAB -> AAAAB
            Rank::ThreeOfAKind => Rank::FourOfAKind,
            // JAAAA -> AAAAA
            Rank::FourOfAKind => Rank::FiveOfAKind,
            _ => unreachable!(),
        },
        2 => match starting_rank {
            // JJABC -> AAABC
            Rank::Pair => Rank::ThreeOfAKind,
            // JJAAB -> AAAAB
            Rank::TwoPair => Rank::FourOfAKind,
            // JJAAA -> AAAAA
            Rank::FullHouse => Rank::FiveOfAKind,
            _ => unreachable!(),
        },
        3 => match starting_rank {
            // JJJAB -> AAAAB
            Rank::ThreeOfAKind => Rank::FourOfAKind,
            // JJJAA -> AAAAA
            Rank::FullHouse => Rank::FiveOfAKind,
            _ => unreachable!(),
        },
        // JJJJA -> AAAAA
        4 => Rank::FiveOfAKind,
        // JJJJJ -> AAAAA
        5 => Rank::FiveOfAKind,
        _ => unreachable!("Invalid number of Jokers"),
    }
}

fn compare(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    let ar = joker_rank(a);
    let br = joker_rank(b);

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

fn replace_with_joker(hand: &Hand, joker: Card) -> Hand {
    Hand {
        cards: hand
            .cards
            .iter()
            .map(|c| if *c == joker { Card(0) } else { *c })
            .collect_vec(),
        bid: hand.bid,
    }
}

pub fn process(input: &str) -> Result<u64> {
    let input = parse_input(input)?;

    Ok(input
        .hands
        .iter()
        // replace J with Jokers
        .map(|h| replace_with_joker(h, Card(11)))
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
        assert_eq!(5905, process(input.trim())?);
        Ok(())
    }
}

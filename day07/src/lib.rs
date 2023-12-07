use itertools::Itertools;

pub mod part1;
pub mod part2;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Rank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Card(u8);

impl Card {
    fn from_char(c: char) -> Result<Self> {
        Ok(Card(card_value(c)?))
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
}

impl Hand {
    fn rank(&self) -> Rank {
        use Rank::*;
        let counts = self.cards.iter().counts();
        let counts = counts.values().sorted().rev().collect_vec();

        match counts[0] {
            5 => FiveOfAKind,
            4 => FourOfAKind,
            3 => match counts[1] {
                2 => FullHouse,
                _ => ThreeOfAKind,
            },
            2 => match counts[1] {
                2 => TwoPair,
                _ => Pair,
            },
            1 => HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Input {
    hands: Vec<Hand>,
}

fn card_value(c: char) -> Result<u8> {
    match c {
        'A' => Ok(14),
        'T' => Ok(10),
        'J' => Ok(11),
        'Q' => Ok(12),
        'K' => Ok(13),
        c if c.is_ascii_digit() => c
            .to_digit(10)
            .map(|i| i as u8)
            .ok_or("Not a digit!?".into()),
        _ => Err("Invalid card value".into()),
    }
}

pub fn parse_input(input: &str) -> Result<Input> {
    let mut hands = Vec::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let cards = split
            .next()
            .ok_or("EOF")?
            .chars()
            .map(Card::from_char)
            .collect::<Result<Vec<_>>>()?;
        let bid = split.next().ok_or("EOF")?.parse::<u64>()?;

        hands.push(Hand { cards, bid });
    }

    Ok(Input { hands })
}

use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{self, space1},
    },
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

pub mod part1;
pub mod part2;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct Card {
    winners: Vec<u32>,
    nums: Vec<u32>,
}

impl Card {
    fn winner_count(&self) -> usize {
        self.nums
            .iter()
            .filter(|n| self.winners.contains(n))
            .count()
    }
}

#[derive(Debug)]
pub struct Input {
    cards: Vec<Card>,
}

type Res<'a, T> = IResult<&'a str, T>;

fn card(input: &str) -> Res<Card> {
    // parse "Card" + any number of spaces + id
    let (input, _id) = preceded(terminated(tag("Card"), space1), complete::u32)(input)?;
    // parse ":" + any number of spaces
    let (input, _) = terminated(tag(":"), space1)(input)?;

    // parse an (any-number-of-space)-separated list of numbers
    let num_list = || separated_list1(space1, complete::u32);

    // parse 2 lists of numbers, separated by (| surrounded by any number of spaces)
    let (input, (winners, nums)) =
        separated_pair(num_list(), delimited(space1, tag("|"), space1), num_list())(input)?;

    Ok((input, Card { winners, nums }))
}

pub fn parse_input(input: &str) -> Result<Input> {
    // parse a list of cards, separated by newlines
    let (_, cards) = separated_list1(character::complete::newline, card)(input)
        .map_err(|e| format!("Failed to parse input: {}", e))?;

    // sanity check to make sure we got the right number of cards
    let line_count = input.lines().count();
    if cards.len() != line_count {
        return Err(format!("Expected {} cards, got {}", line_count, cards.len()).into());
    }
    Ok(Input { cards })
}

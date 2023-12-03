use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

pub mod part1;
pub mod part2;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub type Input<'a> = Vec<Game<'a>>;

type Res<'a, T> = IResult<&'a str, T>;

#[derive(Debug)]
pub struct Cube<'a> {
    pub color: &'a str,
    pub count: u32,
}

#[derive(Debug)]
pub struct Game<'a> {
    pub id: u32,
    pub cubes: Vec<Vec<Cube<'a>>>,
}

fn cube(input: &str) -> Res<Cube> {
    map(
        separated_pair(complete::u32, space1, alpha1),
        |(count, color)| Cube { color, count },
    )(input)
}

fn round(input: &str) -> Res<Vec<Cube>> {
    separated_list1(tag(", "), cube)(input)
}

fn rounds(input: &str) -> Res<Vec<Vec<Cube>>> {
    separated_list1(tag("; "), round)(input)
}

fn id(input: &str) -> Res<u32> {
    preceded(tag("Game "), terminated(complete::u32, tag(": ")))(input)
}

fn game(input: &str) -> Res<Game> {
    map(tuple((id, rounds)), |(id, cubes)| Game { id, cubes })(input)
}

fn games(input: &str) -> Res<Input> {
    separated_list1(complete::line_ending, game)(input)
}

pub fn parse_input(input: &str) -> Result<Input> {
    if let Ok((_, games)) = games(input) {
        Ok(games)
    } else {
        Err(From::from("Failed to parse input"))
    }
}

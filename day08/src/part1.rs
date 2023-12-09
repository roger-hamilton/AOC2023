use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

use crate::Result;

#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Maze<'a> {
    directions: Vec<Dir>,
    rooms: HashMap<&'a str, (&'a str, &'a str)>,
}

type Room<'a> = (&'a str, (&'a str, &'a str));

fn parse_room(input: &str) -> IResult<&str, Room> {
    let (input, label) = alphanumeric1(input)?;
    let (input, _) = tag(" = (")(input)?;

    let (input, left) = alphanumeric1(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = alphanumeric1(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (label, (left, right))))
}

fn parse_dirs(input: &str) -> IResult<&str, Vec<Dir>> {
    many1(map(one_of("LR"), |c| match c {
        'L' => Dir::Left,
        'R' => Dir::Right,
        _ => panic!("invalid direction"),
    }))(input)
}

pub fn parse_maze(input: &str) -> IResult<&str, Maze> {
    let (input, directions) = parse_dirs(input)?;

    let (input, _) = tag("\n\n")(input)?;

    let (input, rooms) = separated_list1(line_ending, parse_room)(input)?;
    let rooms = rooms.into_iter().collect();
    Ok((input, Maze { directions, rooms }))
}

pub fn parse_input(input: &str) -> Result<Maze> {
    let (_, maze) = parse_maze(input).expect("failed to parse input");
    Ok(maze)
}

pub fn process(input: &str) -> Result<u32> {
    let input = parse_input(input)?;

    let mut current = "AAA";
    let mut count = 0;
    for direction in input.directions.iter().cycle() {
        count += 1;
        let (left, right) = input.rooms.get(current).unwrap();
        current = match direction {
            Dir::Left => left,
            Dir::Right => right,
        };

        if current == "ZZZ" {
            break;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
        "#};
        assert_eq!(2, process(input.trim())?);
        Ok(())
    }

    #[test]
    fn test_process2() -> Result<()> {
        let input = indoc! {r#"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        "#};
        assert_eq!(6, process(input.trim())?);
        Ok(())
    }
}

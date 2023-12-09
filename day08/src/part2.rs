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

struct MazeIter<'a> {
    input: &'a Maze<'a>,
    current: &'a str,
    count: usize,
}

impl Maze<'_> {
    fn iter<'a>(&'a self, start: &'a str) -> MazeIter<'a> {
        MazeIter {
            input: self,
            current: start,
            count: 0,
        }
    }
}

impl<'a> Iterator for MazeIter<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let (left, right) = self.input.rooms.get(self.current)?;

        let dir_idx = self.count % self.input.directions.len();

        self.current = match self.input.directions[dir_idx] {
            Dir::Left => *left,
            Dir::Right => *right,
        };

        self.count += 1;

        Some((self.count, self.current))
    }
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

#[inline]
fn gcd(mut a: usize, mut b: usize) -> usize {
    if b > a {
        return gcd(b, a);
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[inline]
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

// First find the cycle length of starting at each "A" node and going to a "Z" node.
// Then find the least common multiple of all of the cycle lengths to find the first time they all line up.
//
// Some things that allow this solution to work:
// - all cycles are a constant length (even from the beginning)
// - the "state" of when a maze reaches a "Z" node doesn't matter (ie the spot in the list of directions)
// - there is just one "Z" node in a starting node's cycle
pub fn process(input: &str) -> Result<usize> {
    let input = parse_input(input)?;

    Ok(input
        .rooms
        .keys()
        .filter(|k| k.ends_with('A'))
        .filter_map(|k| input.iter(k).find(|(_, s)| s.ends_with('Z')))
        .map(|x| x.0)
        .fold(1, lcm))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
        "#};
        assert_eq!(6, process(input.trim())?);
        Ok(())
    }
}

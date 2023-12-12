use itertools::Itertools;
use std::{collections::HashMap, isize};

use crate::Result;

pub enum Cell {
    Empty,
    Start,
    Vertical,
    Horizontal,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

type Input = Vec<Vec<Cell>>;

type Loc = (isize, isize);

pub fn parse_input(input: &str) -> Result<Input> {
    Ok(input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| match c {
                    '.' => Some(Cell::Empty),
                    '|' => Some(Cell::Vertical),
                    '-' => Some(Cell::Horizontal),
                    'F' => Some(Cell::NorthWest),
                    '7' => Some(Cell::NorthEast),
                    'J' => Some(Cell::SouthEast),
                    'L' => Some(Cell::SouthWest),
                    'S' => Some(Cell::Start),
                    _ => None,
                })
                .collect_vec()
        })
        .collect_vec())
}

fn add_connection(
    adj: &mut HashMap<Loc, Vec<Loc>>,
    from: Loc,
    to: Loc,
    width: isize,
    height: isize,
) {
    if from.0 >= width
        || from.1 >= height
        || to.0 >= width
        || to.1 >= height
        || from.0 < 0
        || from.1 < 0
        || to.0 < 0
        || to.1 < 0
    {
        return;
    }
    adj.entry(from)
        .or_insert_with(|| Vec::with_capacity(4))
        .push(to);
    adj.entry(to).or_default().push(from);
}

fn build_adj(input: &Input) -> (Loc, HashMap<Loc, Vec<Loc>>) {
    let height = input.len() as isize;
    let width = input[0].len() as isize;

    let mut adj = HashMap::new();

    let get_at = |x: isize, y: isize| input.get(y as usize).and_then(|row| row.get(x as usize));

    let mut start = (0, 0);

    for (y, line) in input.iter().enumerate() {
        for (x, cell) in line.iter().enumerate() {
            use Cell::*;
            let y = y as isize;
            let x = x as isize;
            match cell {
                Empty => {}
                Vertical => {
                    match get_at(x, y - 1) {
                        Some(Vertical) | Some(NorthEast) | Some(NorthWest) | Some(Start) => {
                            add_connection(&mut adj, (x, y), (x, y - 1), width, height)
                        }
                        _ => {}
                    };
                    match get_at(x, y + 1) {
                        Some(Vertical) | Some(SouthEast) | Some(SouthWest) | Some(Start) => {
                            add_connection(&mut adj, (x, y), (x, y + 1), width, height)
                        }
                        _ => {}
                    };
                }
                Horizontal => {
                    match get_at(x + 1, y) {
                        Some(Horizontal) | Some(SouthEast) | Some(NorthEast) | Some(Start) => {
                            add_connection(&mut adj, (x, y), (x + 1, y), width, height)
                        }
                        _ => {}
                    };
                    match get_at(x - 1, y) {
                        Some(Horizontal) | Some(NorthWest) | Some(SouthWest) | Some(Start) => {
                            add_connection(&mut adj, (x, y), (x - 1, y), width, height)
                        }
                        _ => {}
                    };
                }
                NorthWest => {
                    match get_at(x, y + 1) {
                        Some(Vertical) | Some(SouthEast) | Some(SouthWest) | Some(Start) => {
                            add_connection(&mut adj, (x, y), (x, y + 1), width, height)
                        }
                        _ => {}
                    };
                    match get_at(x + 1, y) {
                        Some(Horizontal) | Some(SouthEast) | Some(NorthEast) | Some(Start) => {
                            add_connection(&mut adj, (x, y), (x + 1, y), width, height)
                        }
                        _ => {}
                    };
                }
                NorthEast => {
                    match get_at(x, y + 1) {
                        Some(Vertical) | Some(SouthEast) | Some(SouthWest) | Some(Start) => {
                            add_connection(&mut adj, (x, y), (x, y + 1), width, height)
                        }
                        _ => {}
                    };
                    match get_at(x - 1, y) {
                        Some(Horizontal) | Some(NorthWest) | Some(SouthWest) | Some(Start) => {
                            add_connection(&mut adj, (x, y), (x - 1, y), width, height)
                        }
                        _ => {}
                    };
                }
                SouthEast => {
                    match get_at(x, y - 1) {
                        Some(Vertical) | Some(NorthEast) | Some(NorthWest) | Some(Start) => {
                            add_connection(&mut adj, (x, y), (x, y - 1), width, height)
                        }
                        _ => {}
                    };
                    match get_at(x - 1, y) {
                        Some(Horizontal) | Some(NorthWest) | Some(SouthWest) | Some(Start) => {
                            add_connection(&mut adj, (x, y), (x - 1, y), width, height)
                        }
                        _ => {}
                    };
                }
                SouthWest => {
                    match get_at(x, y - 1) {
                        Some(Vertical) | Some(NorthEast) | Some(NorthWest) | Some(Start) => {
                            add_connection(&mut adj, (x, y), (x, y - 1), width, height)
                        }
                        _ => {}
                    };
                    match get_at(x + 1, y) {
                        Some(Horizontal) | Some(SouthEast) | Some(NorthEast) | Some(Start) => {
                            add_connection(&mut adj, (x, y), (x + 1, y), width, height)
                        }
                        _ => {}
                    };
                }
                Start => {
                    start = (x, y);
                    match get_at(x - 1, y) {
                        Some(Horizontal) | Some(NorthWest) | Some(SouthWest) => {
                            add_connection(&mut adj, (x, y), (x - 1, y), width, height)
                        }
                        _ => {}
                    };
                    match get_at(x + 1, y) {
                        Some(Horizontal) | Some(SouthEast) | Some(NorthEast) => {
                            add_connection(&mut adj, (x, y), (x + 1, y), width, height)
                        }
                        _ => {}
                    };
                    match get_at(x, y - 1) {
                        Some(Vertical) | Some(NorthEast) | Some(NorthWest) => {
                            add_connection(&mut adj, (x, y), (x, y - 1), width, height)
                        }
                        _ => {}
                    };
                    match get_at(x, y + 1) {
                        Some(Vertical) | Some(SouthEast) | Some(SouthWest) => {
                            add_connection(&mut adj, (x, y), (x, y + 1), width, height)
                        }
                        _ => {}
                    };
                }
            }
        }
    }

    (start, adj)
}

pub fn process(input: &str) -> Result<u32> {
    let input = parse_input(input)?;

    let (start, adj) = build_adj(&input);

    let mut curr = &adj[&start][0];
    let mut last = &start;
    let mut steps = 1;
    while curr != &start {
        let next = adj[curr]
            .iter()
            .find(|&n| match adj.get(n) {
                Some(p) => n != last && p.contains(curr),
                _ => false,
            })
            .unwrap();
        last = curr;
        curr = next;
        steps += 1;
    }

    Ok(steps / 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
        "#};
        assert_eq!(4, process(input.trim())?);
        // assert!(false);
        Ok(())
    }
    #[test]
    fn test_process2() -> Result<()> {
        let input = indoc! {r#"
        |JF7.
        LFJ|.
        SJ.L7
        |F--J
        LJ...
        "#};
        assert_eq!(8, process(input.trim())?);
        // assert!(false);
        Ok(())
    }
}

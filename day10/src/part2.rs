use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    isize,
};

use crate::Result;

pub enum Cell {
    Empty,
    Start,
    Vertical,
    Horizontal,
    // these are the corners
    // the name is the direction of the corner not the path
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

fn color_neighbors(colors: &mut HashMap<Loc, Color>, start: Loc) {
    let start_color = colors[&start];
    let mut queue = VecDeque::new();
    queue.push_back(start);

    // we need to clear the start color so we pass the first check
    colors.remove(&start);

    while let Some(curr) = queue.pop_front() {
        if colors.contains_key(&curr) {
            continue;
        }
        colors.insert(curr, start_color);
        let (cx, cy) = curr;
        queue.extend(
            [(cx - 1, cy), (cx + 1, cy), (cx, cy - 1), (cx, cy + 1)]
                .iter()
                .filter(|&n| colors.get(n).is_none())
                .copied(),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Path,
    Left,
    Right,
}

pub fn process(input: &str) -> Result<u32> {
    let input = parse_input(input)?;

    let (start, adj) = build_adj(&input);

    let mut color = HashMap::new();

    color.insert(start, Color::Path);

    let mut curr = &adj[&start][0];
    let mut last = &start;

    while curr != &start {
        use Cell::*;
        use Color::*;
        color.insert(*curr, Path);

        let (cx, cy) = *curr;
        let (lx, ly) = *last;

        let ccell = &input[cy as usize][cx as usize];

        if cx > lx {
            // right
            color.entry((cx, cy - 1)).or_insert(Left);
            color.entry((cx, cy + 1)).or_insert(Right);
            match ccell {
                NorthEast => {
                    color.entry((cx + 1, cy)).or_insert(Left);
                }
                SouthEast => {
                    color.entry((cx + 1, cy)).or_insert(Right);
                }
                _ => {}
            }
        } else if cx < lx {
            // left
            color.entry((cx, cy - 1)).or_insert(Right);
            color.entry((cx, cy + 1)).or_insert(Left);
            match ccell {
                NorthWest => {
                    color.entry((cx - 1, cy)).or_insert(Right);
                }
                SouthWest => {
                    color.entry((cx - 1, cy)).or_insert(Left);
                }
                _ => {}
            }
        } else if cy > ly {
            // down
            color.entry((cx - 1, cy)).or_insert(Right);
            color.entry((cx + 1, cy)).or_insert(Left);
            match ccell {
                SouthEast => {
                    color.entry((cx, cy + 1)).or_insert(Left);
                }
                SouthWest => {
                    color.entry((cx, cy + 1)).or_insert(Right);
                }
                _ => {}
            }
        } else if cy < ly {
            // up
            color.entry((cx - 1, cy)).or_insert(Left);
            color.entry((cx + 1, cy)).or_insert(Right);

            match ccell {
                NorthEast => {
                    color.entry((cx, cy - 1)).or_insert(Right);
                }
                NorthWest => {
                    color.entry((cx, cy - 1)).or_insert(Left);
                }
                _ => {}
            }
        } else {
            panic!("We didn't move?")
        }

        let next = adj[curr]
            .iter()
            .find(|&n| match adj.get(n) {
                Some(p) => n != last && p.contains(curr),
                _ => false,
            })
            .unwrap();
        last = curr;
        curr = next;
    }
    // spiral inwards until we find cell colored 2 or 3

    let mut curr = (0, 0);
    let mut dir = (1, 0);
    let mut side_len = input.len() as isize;
    // this assumes input is a square
    while !matches!(color.get(&curr), Some(Color::Left) | Some(Color::Right)) {
        let (cx, cy) = curr;
        let (dx, dy) = dir;
        let (nx, ny) = (cx + dx, cy + dy);
        if nx == 0 || nx >= side_len - 1 || ny == 0 || ny >= side_len - 1 {
            if dir == (1, 0) {
                side_len -= 1;
            }
            // turn right
            dir = (-dy, dx);
        }
        curr = (nx, ny);
    }

    let inside_color = if color.get(&curr) == Some(&Color::Right) {
        Color::Left
    } else {
        Color::Right
    };

    let inside_locs = color
        .iter()
        .filter(|(_, &c)| c == inside_color)
        .map(|(&l, _)| l)
        .collect_vec();

    // todo!();
    for loc in inside_locs {
        color_neighbors(&mut color, loc);
    }

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            print!(
                "{}",
                match color.get(&(x as isize, y as isize)) {
                    None => ' ',
                    Some(Color::Path) => '.',
                    Some(Color::Left) => 'X',
                    Some(Color::Right) => 'O',
                }
            );
        }
        println!();
    }

    Ok(color.values().filter(|&&c| c == inside_color).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
        "#};
        assert_eq!(4, process(input.trim())?);
        Ok(())
    }
    #[test]
    fn test_process2() -> Result<()> {
        let input = indoc! {r#"
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
        "#};
        assert_eq!(4, process(input.trim())?);
        Ok(())
    }
}

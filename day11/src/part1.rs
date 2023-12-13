use itertools::Itertools;

use crate::Result;

type Galaxy = (usize, usize);

#[derive(Debug)]
pub struct Input {
    galaxies: Vec<Galaxy>,
}

fn expand_map(galaxies: &mut [Galaxy], height: usize, width: usize) {
    // expand the empty cols
    let mut rows = Vec::new();
    for y in 0..height {
        if galaxies.iter().all(|(_, y2)| *y2 != y) {
            rows.push(y);
        }
    }
    let mut c = 0;
    for y in rows {
        // shift all galaxies "below" this row down by 1
        for (_, y2) in galaxies.iter_mut() {
            if *y2 > y + c {
                *y2 += 1;
            }
        }
        c += 1;
    }

    //expand the empty rows
    let mut cols = Vec::new();
    for x in 0..width {
        if galaxies.iter().all(|(x2, _)| *x2 != x) {
            cols.push(x);
        }
    }
    c = 0;
    for x in cols {
        // shift all galaxies "right" of this col to the right by 1
        for (x2, _) in galaxies.iter_mut() {
            if *x2 > x + c {
                *x2 += 1;
            }
        }
        c += 1;
    }
}

pub fn parse_input(input: &str) -> Result<Input> {
    let mut galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((x, y)),
                '.' => None,
                _ => None,
            })
        })
        .collect_vec();

    expand_map(
        &mut galaxies,
        input.lines().count(),
        input.lines().next().unwrap().len(),
    );
    Ok(Input { galaxies })
}

fn sub_abs(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn manhattan_distance(a: Galaxy, b: Galaxy) -> usize {
    let (x1, y1) = a;
    let (x2, y2) = b;

    sub_abs(x1, x2) + sub_abs(y1, y2)
}

fn get_pairs<T>(galaxies: &[T]) -> Vec<(&T, &T)> {
    let mut vec = Vec::with_capacity(galaxies.len() * (galaxies.len() - 1) / 2);

    for (i, a) in galaxies.iter().enumerate() {
        for b in &galaxies[i + 1..] {
            vec.push((a, b));
        }
    }
    vec
}

pub fn process(input: &str) -> Result<usize> {
    let Input { galaxies } = parse_input(input)?;

    let pairs = get_pairs(&galaxies);

    Ok(pairs
        .iter()
        .copied()
        .map(|(a, b)| manhattan_distance(*a, *b))
        .sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
        "#};
        assert_eq!(374, process(input.trim())?);
        Ok(())
    }

    #[test]
    fn test_manhattan_dist() -> Result<()> {
        let a = (1, 6);
        let b = (5, 11);

        assert_eq!(9, manhattan_distance(a, b));
        Ok(())
    }

    #[test]
    fn test_full() -> Result<()> {
        let input = include_str!("../input.txt");

        let count = input
            .lines()
            .map(|l| l.chars().filter(|c| *c == '#').count())
            .sum::<usize>();
        assert_eq!(442, count);
        let Input { galaxies } = parse_input(input)?;
        let pairs = get_pairs(&galaxies);
        let expected_pairs = galaxies.len() * (galaxies.len() - 1) / 2;
        assert_eq!(expected_pairs, pairs.len());
        assert!(process(input.trim())? > 9165297);
        Ok(())
    }
}

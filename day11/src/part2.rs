use itertools::Itertools;

use crate::Result;

type Galaxy = (usize, usize);

#[derive(Debug)]
pub struct Input {
    galaxies: Vec<Galaxy>,
    expanded_rows: Vec<usize>,
    expanded_cols: Vec<usize>,
}

pub fn parse_input(input: &str) -> Result<Input> {
    let galaxies = input
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

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    // expand the empty cols
    let mut rows = Vec::new();
    for y in 0..height {
        if galaxies.iter().all(|(_, y2)| *y2 != y) {
            rows.push(y);
        }
    }
    //expand the empty rows
    let mut cols = Vec::new();
    for x in 0..width {
        if galaxies.iter().all(|(x2, _)| *x2 != x) {
            cols.push(x);
        }
    }
    Ok(Input {
        galaxies,
        expanded_cols: cols,
        expanded_rows: rows,
    })
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

pub fn process(input: &str, expansion: usize) -> Result<usize> {
    let expansion = expansion - 1;
    let Input {
        galaxies,
        expanded_cols,
        expanded_rows,
    } = parse_input(input)?;

    let pairs = get_pairs(&galaxies);

    Ok(pairs
        .iter()
        .copied()
        .map(|(a, b)| {
            manhattan_distance(*a, *b)
                + expanded_cols
                    .iter()
                    .filter(|x| a.0.min(b.0) < **x && **x < a.0.max(b.0))
                    .count()
                    * expansion
                + expanded_rows
                    .iter()
                    .filter(|y| a.1.min(b.1) < **y && **y < a.1.max(b.1))
                    .count()
                    * expansion
        })
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
        assert_eq!(1030, process(input.trim(), 10)?);
        Ok(())
    }

    #[test]
    fn test_process2() -> Result<()> {
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
        assert_eq!(8410, process(input.trim(), 100)?);
        Ok(())
    }
}

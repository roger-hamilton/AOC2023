use itertools::Itertools;

use crate::{parse_input, Cell, Input, Result};

fn neighbor_indexes(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if x > 0 {
        if y > 0 {
            result.push((x - 1, y - 1));
        }
        result.push((x - 1, y));
        result.push((x - 1, y + 1));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    result.push((x, y + 1));

    if y > 0 {
        result.push((x + 1, y - 1));
    }
    result.push((x + 1, y));
    result.push((x + 1, y + 1));

    result
}

fn num_at(input: &Input, x: usize, y: usize) -> Option<((usize, usize), u32)> {
    if let Some(Cell::Num(n)) = input.get(x, y) {
        let mut n = *n;
        let mut min = x;
        if x > 0 {
            let mut dx = 1;
            while let Some(Cell::Num(m)) = input.get(x - dx, y) {
                n += m * (10u32.pow(dx as u32));
                if dx == x {
                    break;
                }
                dx += 1;
            }
            min = x - dx;
        }

        let mut dx = 1;
        while let Some(Cell::Num(m)) = input.get(x + dx, y) {
            n = n * 10 + m;
            dx += 1;
        }

        Some(((min, y), n))
    } else {
        None
    }
}

pub fn process(input: &str) -> Result<u32> {
    let input = parse_input(input)?;

    let mut sum = 0;

    for (y, row) in input.grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Cell::Sym('*') = cell {
                // check all neighbors
                let neighbors = neighbor_indexes(x, y);
                let n_nums = neighbors
                    .iter()
                    .filter_map(|(x, y)| num_at(&input, *x, *y))
                    .unique()
                    .collect_vec();
                if n_nums.len() == 2 {
                    sum += n_nums[0].1 * n_nums[1].1;
                }
            }
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_process() -> Result<()> {
        let input = indoc! {r#"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        "#};
        assert_eq!(467835, process(input.trim())?);
        Ok(())
    }
}

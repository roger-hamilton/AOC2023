pub mod part1;
pub mod part2;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Empty,
    Num(u32),
    Sym(char),
}

pub struct Input {
    grid: Vec<Vec<Cell>>,
}

impl Input {
    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.grid.get(y).and_then(|row| row.get(x))
    }
}

pub fn parse_input(input: &str) -> Result<Input> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            if let Some(d) = c.to_digit(10) {
                row.push(Cell::Num(d));
            } else if c == '.' {
                row.push(Cell::Empty);
            } else {
                row.push(Cell::Sym(c));
            }
        }
        grid.push(row);
    }

    Ok(Input { grid })
}

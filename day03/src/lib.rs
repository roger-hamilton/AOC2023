use glam::IVec2;

pub mod part1;
pub mod part2;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Entry {
    id: u32,
    pos: IVec2,
    num: u32,
}

pub struct Input {
    numbers: Vec<Entry>,
    symbols: Vec<(IVec2, char)>,
}

pub fn parse_input(input: &str) -> Result<Input> {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    let mut num_id = 0;

    for (y, line) in input.lines().enumerate() {
        let mut num = 0;
        let mut len = 0;
        let mut last_x = 0;
        for (x, c) in line.chars().enumerate() {
            if let Some(digit) = c.to_digit(10) {
                num = num * 10 + digit;
                len += 1;
            } else {
                if len > 0 {
                    while len > 0 {
                        numbers.push(Entry {
                            id: num_id,
                            pos: IVec2::new(x as i32 - len, y as i32),
                            num,
                        });
                        len -= 1;
                    }
                    num = 0;
                    len = 0;
                    num_id += 1;
                }
                if c != '.' {
                    symbols.push((IVec2::new(x as i32, y as i32), c));
                }
            }
            last_x = x;
        }
        if len > 0 {
            while len > 0 {
                numbers.push(Entry {
                    id: num_id,
                    pos: IVec2::new(last_x as i32 - len, y as i32),
                    num,
                });
                len -= 1;
            }
            num_id += 1;
        }
    }

    Ok(Input { numbers, symbols })
}

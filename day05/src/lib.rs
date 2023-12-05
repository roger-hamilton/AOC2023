use itertools::Itertools;

pub mod part1;
pub mod part2;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MapRange {
    source_start: usize,
    dest_start: usize,
    length: usize,
}

#[macro_export]
macro_rules! map_range {
    ($source_start:expr => $dest_start:expr, $length:expr) => {
        MapRange {
            source_start: $source_start,
            dest_start: $dest_start,
            length: $length,
        }
    };
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    map: Vec<MapRange>,
}

impl Map {
    pub fn get_destination(&self, source: usize) -> Option<usize> {
        self.map
            .iter()
            .find_map(|r| {
                if source < r.source_start || source >= r.source_start + r.length {
                    return None;
                }
                Some(r.dest_start + (source - r.source_start))
            })
            .or(Some(source))
    }
}

#[derive(Debug)]
pub struct Input {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

fn parse_map(input: &str) -> Result<Map> {
    let mut lines = input.lines();
    let name_line = lines.next().expect("Group was empty");
    let name = name_line[0..name_line.len() - 5].to_string();
    let (from, to) = name
        .split("-to-")
        .map(|s| s.to_string())
        .collect_tuple()
        .expect("Expected two words");

    let map = lines
        .map(|l| {
            let (dest_start, source_start, length) = l
                .split_ascii_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple()
                .expect("Expected three numbers");
            MapRange {
                dest_start,
                source_start,
                length,
            }
        })
        .collect_vec();

    Ok(Map { from, to, map })
}

pub fn parse_input(input: &str) -> Result<Input> {
    let groups = input.split("\n\n").collect::<Vec<_>>();

    // first group is seeds
    let seeds = groups[0][7..]
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect_vec();

    // remaining groups are maps
    let maps = groups[1..]
        .iter()
        .filter_map(|g| parse_map(g).ok())
        .collect::<Vec<_>>();

    Ok(Input { seeds, maps })
}

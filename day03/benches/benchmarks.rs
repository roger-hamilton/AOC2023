use day03::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../input.txt"))).unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!("../input.txt"))).unwrap();
}

#[divan::bench]
fn parse_input() {
    day03::parse_input(divan::black_box(include_str!("../input.txt"))).unwrap();
}

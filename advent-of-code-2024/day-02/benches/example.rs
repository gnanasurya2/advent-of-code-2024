use day_01;
use divan::black_box;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() -> i32 {
    let input = include_str!("../src/input1.txt");
    day_01::part1::process(black_box(input))
}

#[divan::bench]
fn part2() -> i32 {
    let input = include_str!("../src/input1.txt");
    day_01::part2::process(black_box(input))
}

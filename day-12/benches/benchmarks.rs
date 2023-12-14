use day_12::{process_part1, process_part2};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    process_part1(divan::black_box(include_str!("../test-data.txt",)));
}

#[divan::bench]
fn part2() {
    process_part2(divan::black_box(include_str!("../test-data.txt",)));
}

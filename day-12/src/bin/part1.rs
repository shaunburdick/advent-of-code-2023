use day_12::process_part1;

fn main() {
    let file = include_str!("../../test-data.txt");
    let result = process_part1(file);
    println!("Part 1 Result: {}", result);
}

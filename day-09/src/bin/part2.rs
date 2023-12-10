use day_09::process_part2;

fn main() {
    let file = include_str!("../../test-data.txt");
    let result = process_part2(file);
    println!("Part 1 Result: {}", result);
}

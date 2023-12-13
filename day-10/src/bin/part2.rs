use day_10::process_part2;

fn main() {
    let file = include_str!("../../test-data.txt");
    let result = process_part2(file);
    println!("Part 2 Result: {}", result);
}

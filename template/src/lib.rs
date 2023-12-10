pub fn process_part1(file: &str) -> usize {
    1
}

pub fn process_part2(file: &str) -> usize {
    2
}

#[cfg(test)]
mod tests_{{crate_name}} {
    use rstest::rstest;

    use super::{process_part1, process_part2};

    #[rstest]
    fn test_process_part1() {
        let input = "";
        assert_eq!(process_part1(input), 1);
    }

    #[rstest]
    fn test_process_part2() {
        let input = "";
        assert_eq!(process_part2(input), 2);
    }
}

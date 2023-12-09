use clap::Parser;
use std::{fs::read_to_string, path::PathBuf, process};

#[derive(clap::Parser)]
struct Cli {
    input_file: PathBuf,
}

fn main() {
    // Get command line arguments
    let args = Cli::parse();

    // Read file from CLI arg
    if let Ok(file) = read_to_string(&args.input_file) {
        let numbers = file
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .flat_map(|digit| digit.parse::<isize>())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let part_1_answer = numbers
            .iter()
            .map(|seq| sequence_next_number(seq.to_owned()))
            .sum::<isize>();

        let part_2_answer = 1;

        println!("Part 1: {}\nPart 2: {}", part_1_answer, part_2_answer);
    } else {
        eprintln!("Could not read file: {}", args.input_file.display());
        process::exit(1);
    }
}

/// Generate the next number in a sequence from a list of numbers
///
/// Arguments:
/// - numbers: the list of numbers
fn sequence_next_number(numbers: Vec<isize>) -> isize {
    let mut next_number = *numbers.last().expect("there to always be a number");

    let mut current_sequence = numbers;

    loop {
        current_sequence = current_sequence
            .iter()
            .skip(1)
            .enumerate()
            .map(|(index, current_number)| current_number - current_sequence.get(index).unwrap())
            .collect::<Vec<_>>();

        next_number += current_sequence
            .last()
            .expect("there to always be a number");

        if current_sequence.iter().all(|num| num == &0) {
            break;
        }
    }

    next_number
}

#[cfg(test)]
mod tests_day_9 {
    use super::sequence_next_number;
    use rstest::rstest;

    #[rstest]
    #[case(vec![0, 3, 6, 9, 12, 15], 18)]
    #[case(vec![1, 3, 6, 10, 15, 21], 28)]
    #[case(vec![10, 13, 16, 21, 30, 45], 68)]
    #[case(vec![8, 6, 4, 2, 0, -2, -4], -6)]
    #[case(vec![3, -2, -5, -7, -14, -36, -70, -54, 221], 1254)]
    fn test_sequence_next_number(#[case] input: Vec<isize>, #[case] expected: isize) {
        assert_eq!(sequence_next_number(input), expected);
    }
}

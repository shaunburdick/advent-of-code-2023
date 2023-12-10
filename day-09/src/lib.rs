pub fn process_part1(file: &str) -> isize {
    file.lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .flat_map(|digit| digit.parse::<isize>())
                .collect::<Vec<_>>()
        })
        .map(|seq| sequence_next_number(seq.to_owned()))
        .sum::<isize>()
}

pub fn process_part2(file: &str) -> isize {
    file.lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .flat_map(|digit| digit.parse::<isize>())
                .collect::<Vec<_>>()
        })
        .map(|seq| sequence_previous_number(seq.to_owned()))
        .sum::<isize>()
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
            // skip first number as we are subtracting backwards
            .skip(1)
            .enumerate()
            // subtract the current number by the previous number
            // (which happens to be at the same index of the previous sequence)
            .map(|(index, current_number)| current_number - current_sequence.get(index).unwrap())
            .collect::<Vec<_>>();

        // if the new sequence is empty, we're done so just add zero
        next_number += current_sequence.last().unwrap_or(&0);

        // check if the new sequence is all zeros and break out
        if current_sequence.iter().all(|num| num == &0) {
            break;
        }
    }

    next_number
}

/// Generate the previous number in a sequence from a list of numbers
///
/// Arguments:
/// - numbers: the list of numbers
fn sequence_previous_number(numbers: Vec<isize>) -> isize {
    let mut prev_numbers = vec![*numbers.first().expect("there to always be a number")];

    let mut current_sequence = numbers;

    loop {
        current_sequence = current_sequence
            .iter()
            // skip first number as we are subtracting backwards
            .skip(1)
            .enumerate()
            // subtract the current number by the previous number
            // (which happens to be at the same index of the previous sequence)
            .map(|(index, current_number)| current_number - current_sequence.get(index).unwrap())
            .collect::<Vec<_>>();

        // add first number of sequence to list, or 0 if we're empty
        prev_numbers.push(*current_sequence.first().unwrap_or(&0));

        // check if the new sequence is all zeros and break out
        if current_sequence.iter().all(|num| num == &0) {
            break;
        }
    }

    prev_numbers
        .iter()
        // reverse the order so we start from the "bottom"
        .rev()
        // subtract the current number by the previous number
        .fold(0, |acc, num| num - acc)
}

#[cfg(test)]
mod tests_day_09 {
    use super::{sequence_next_number, sequence_previous_number};
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

    #[rstest]
    #[case(vec![10, 13, 16, 21, 30, 45], 5)]
    #[case(vec![8, 6, 4, 2, 0, -2, -4], 10)]
    #[case(vec![0, -5, -10, -15, -20, -25], 5)]
    #[case(vec![3, 5, 4, -3, -29, -102], -4)]
    fn test_sequence_previous_number(#[case] input: Vec<isize>, #[case] expected: isize) {
        assert_eq!(sequence_previous_number(input), expected);
    }
}

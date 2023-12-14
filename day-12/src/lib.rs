use std::str::FromStr;

use itertools::{repeat_n, Itertools};
use rayon::prelude::*;

use nom::{
    bytes::complete::tag,
    character::complete::{self, one_of, space1},
    error::Error,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    Finish, IResult,
};

pub fn process_part1(file: &str) -> usize {
    file.par_lines()
        .map(|line| {
            let row = SpringRow::from_str(line).expect("line to parse");
            row.get_possible_arrangements().len()
        })
        .sum()
}

pub fn process_part2(file: &str) -> usize {
    2
}

#[derive(Debug, PartialEq, Eq)]
enum SpringCondition {
    Working,
    Unknown,
    None,
}

#[derive(Debug)]
struct SpringRow {
    checksum: Vec<u32>,
    springs: Vec<SpringCondition>,
}

impl SpringRow {
    /// Get a list of possible arrangements
    fn get_possible_arrangements(&self) -> Vec<Vec<&SpringCondition>> {
        let permutations = repeat_n(
            [SpringCondition::Working, SpringCondition::None].iter(),
            self.springs
                .iter()
                .filter(|s| s == &&SpringCondition::Unknown)
                .count(),
        )
        .multi_cartesian_product()
        .filter_map(|permutation| {
            let mut permutation_iter = permutation.iter();
            let new_springs = self
                .springs
                .iter()
                .map(|spring| match spring {
                    SpringCondition::Unknown => {
                        permutation_iter.next().expect("there to be a permutation")
                    }
                    sc => sc,
                })
                .collect::<Vec<_>>();

            let checksum = new_springs
                .iter()
                .group_by(|sc| sc == &&&SpringCondition::Working)
                .into_iter()
                .filter_map(|(is_hashes, group)| {
                    is_hashes.then_some(group.into_iter().count() as u32)
                })
                .collect::<Vec<u32>>();

            if checksum == self.checksum {
                Some(new_springs)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

        permutations
    }
}

fn parse_spring_row(s: &str) -> IResult<&str, (Vec<char>, Vec<u32>)> {
    separated_pair(
        many1(one_of("?#.")),
        space1,
        separated_list1(tag(","), complete::u32),
    )(s)
}

impl FromStr for SpringRow {
    type Err = Error<String>;

    /// Parse a SpringRow from a string
    ///
    /// Example: "???.### 1,1,3" -> SpringRow { checksum: [3, 2, 1], springs: [...] }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_spring_row(s).finish() {
            Ok((_remaining, (spring_chars, checksum))) => Ok(Self {
                checksum,
                springs: spring_chars
                    .iter()
                    .map(|spring_char| match spring_char {
                        '#' => SpringCondition::Working,
                        '?' => SpringCondition::Unknown,
                        _ => SpringCondition::None,
                    })
                    .collect(),
            }),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[cfg(test)]
mod tests_day_12 {
    use rstest::rstest;

    use super::{process_part1, process_part2};

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_process_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process_part1(input), expected);
    }

    #[rstest]
    fn test_process_part2() {
        let input = "";
        assert_eq!(process_part2(input), 2);
    }
}

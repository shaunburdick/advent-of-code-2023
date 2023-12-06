use clap::Parser;
use nom_supreme::ParserExt;
use std::{fs::read_to_string, path::PathBuf, process};

use nom::{
    bytes::complete::is_not,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser as _,
};

#[derive(clap::Parser)]
struct Cli {
    input_file: PathBuf,
}

fn main() {
    // Get command line arguments
    let args = Cli::parse();

    // Read file from CLI arg
    if let Ok(file) = read_to_string(&args.input_file) {
        let (_, races) =
            BoatRace::parse_races(file.as_str()).expect("input file to parse as races");

        let part_1_answer = races
            .iter()
            .map(|race| race.winning_holds().len())
            .product::<usize>();

        let part_2_answer = BoatRace {
            time: 58_819_676,
            distance: 434_104_122_191_218,
        }
        .winning_holds()
        .len();

        println!("Part 1: {}\nPart 2: {}", part_1_answer, part_2_answer);
    } else {
        eprintln!("Could not read file: {}", args.input_file.display());
        process::exit(1);
    }
}

struct BoatRace {
    time: u64,
    distance: u64,
}

impl BoatRace {
    /// Calculate the winning holds for the race
    fn winning_holds(&self) -> Vec<u64> {
        let mut winning_holds = vec![];
        let mut duration = 0;

        loop {
            duration += 1;
            let distance = duration * (self.time - duration);
            if distance > self.distance {
                winning_holds.push(duration);
            } else if !winning_holds.is_empty() {
                break;
            }
        }

        winning_holds
    }

    fn parse_races(s: &str) -> IResult<&str, Vec<Self>> {
        /// Parse numbers from a string
        fn nums(input: &str) -> IResult<&str, Vec<u64>> {
            is_not("0123456789")
                .precedes(separated_list1(space1, complete::u64))
                .parse(input)
        }

        let (left_overs, (times, distances)) = separated_pair(nums, line_ending, nums).parse(s)?;

        Ok((
            left_overs,
            times
                .iter()
                .zip(distances)
                .map(|(time, distance)| BoatRace {
                    time: *time,
                    distance,
                })
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests_day_06 {
    use super::BoatRace;

    #[test]
    fn test_boat_race_winning_holds() {
        assert_eq!(
            BoatRace {
                time: 7,
                distance: 9,
            }
            .winning_holds(),
            (2..=5).collect::<Vec<_>>()
        );

        assert_eq!(
            BoatRace {
                time: 15,
                distance: 40,
            }
            .winning_holds(),
            (4..=11).collect::<Vec<_>>()
        );

        assert_eq!(
            BoatRace {
                time: 30,
                distance: 200,
            }
            .winning_holds(),
            (11..=19).collect::<Vec<_>>()
        );
    }
}

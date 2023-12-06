use clap::Parser;
use std::{
    collections::HashMap, fmt::Debug, fs::read_to_string, path::PathBuf, process, str::FromStr,
};

#[derive(Parser)]
struct Cli {
    input_file: PathBuf,
}

fn main() {
    // Get command line arguments
    let args = Cli::parse();

    // Read file from CLI arg
    if let Ok(file) = read_to_string(&args.input_file) {
        let races = vec![
            BoatRace {
                time: 58,
                distance: 434,
            },
            BoatRace {
                time: 81,
                distance: 1041,
            },
            BoatRace {
                time: 96,
                distance: 2219,
            },
            BoatRace {
                time: 76,
                distance: 1218,
            },
        ];

        let part_1_answer = races
            .iter()
            .map(|race| race.winning_holds().len())
            .product::<usize>();

        let part_2_answer = 1;

        println!("Part 1: {}\nPart 2: {}", part_1_answer, part_2_answer);
    } else {
        eprintln!("Could not read file: {}", args.input_file.display());
        process::exit(1);
    }
}

struct BoatRace {
    time: usize,
    distance: usize,
}

impl BoatRace {
    /// Calculate the winning holds for the race
    fn winning_holds(&self) -> Vec<usize> {
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

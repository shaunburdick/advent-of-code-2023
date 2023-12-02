use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::{Path, PathBuf},
    process,
    str::FromStr,
};

use thiserror::Error;

#[derive(Parser)]
struct Cli {
    input_file: PathBuf,
}

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() {
    // Get command line arguments
    let args = Cli::parse();

    // Read file from CLI arg
    if let Ok(lines) = read_lines(&args.input_file) {
        let (id_sum, max_power) = lines
            // convert to game
            .flat_map(|line| Game::from_str(line.unwrap().as_str()))
            // fold into two values: (sum of ids, sum of game power)
            .fold((0, 0), |mut acc, game| {
                // check if game is above limit
                if !game.above_limit((MAX_RED, MAX_GREEN, MAX_BLUE)) {
                    acc.0 += game.id;
                }

                let max_cubes = game.max_cubes();
                let mut game_power = 1;
                if max_cubes.0 > 0 {
                    game_power *= max_cubes.0;
                }

                if max_cubes.1 > 0 {
                    game_power *= max_cubes.1;
                }

                if max_cubes.2 > 0 {
                    game_power *= max_cubes.2;
                }

                acc.1 += game_power;

                acc
            });

        println!(
            "Part 1 Game Sum: {}\nPart 2 Max Power: {}",
            id_sum, max_power
        );
    } else {
        eprintln!("Could not read file: {}", args.input_file.display());
        process::exit(1);
    }
}

#[derive(Error, Debug)]
enum GameParseError {
    #[error("Unknown game format: {0}")]
    UnknownGameFormat(String),
    #[error("Unable to find Game Id: {0}")]
    MissingGameId(String),
    #[error("Unable to parse Game Id: {0}")]
    ParseGameId(String),
    #[error("Unable to find any rounds in Game Id: {0}")]
    MissingRounds(String),
    #[error("Unknown color amount format: {0}")]
    UnknownColorAmount(String),
    #[error("Unknown color: {0}")]
    UnknownColor(String),
    #[error("Unknown round format: {0}")]
    UnknownRoundFormat(String),
}

#[derive(Debug, Default, PartialEq, Eq)]
struct GameRound {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl FromStr for GameRound {
    type Err = GameParseError;

    /// Parses a round from a comma separated list of colors drawn
    /// If string is empty, it will return a default value of all zeros
    ///
    /// Example:
    /// "3 blue, 4 red, 2 green" -> GameRound { red: 4, green: 2, blue: 3 }
    /// "" -> GameRound { red: 0, green: 0, blue: 0 }
    ///
    /// Arguments:
    /// - s: A string representing a comma separated list of colors drawn
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round = GameRound::default();

        // if s is empty, return the default value
        if s.is_empty() {
            return Ok(round);
        }

        // split on comma to get each color
        for color_group in s.split(',') {
            // color_group should not me "X color"
            let mut values = color_group.trim().split(' ');
            if values.clone().count() != 2 {
                return Err(Self::Err::UnknownRoundFormat(String::from(color_group)));
            }
            // first value should be a number
            // we know it's there from the count() check above
            let num_str = values.next().unwrap();
            // last value should be the color
            // we know it's there from the count() check above
            let color = values.next().unwrap();

            if let Ok(num) = num_str.parse::<usize>() {
                match color {
                    "red" => round.red += num,
                    "green" => round.green += num,
                    "blue" => round.blue += num,
                    _ => return Err(Self::Err::UnknownColor(String::from(color))),
                };
            } else {
                return Err(Self::Err::UnknownColorAmount(String::from(num_str)));
            }
        }

        Ok(round)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Game {
    pub id: usize,
    pub red_total: usize,
    pub green_total: usize,
    pub blue_total: usize,
    pub rounds: Vec<GameRound>,
}

impl Game {
    /// Add a round to the game, incrementing totals
    ///
    /// Arguments:
    /// - round: A game round to add to the game
    pub fn add_round(&mut self, round: GameRound) -> &mut Self {
        self.red_total += round.red;
        self.green_total += round.green;
        self.blue_total += round.blue;
        self.rounds.push(round);

        self
    }

    /// Determine the max number of cubes needed for each color
    ///
    /// Returns a tuple representing (red, green, blue)
    pub fn max_cubes(&self) -> (usize, usize, usize) {
        let mut max_cubes = (0, 0, 0);

        for round in &self.rounds {
            if round.red > max_cubes.0 {
                max_cubes.0 = round.red;
            }

            if round.green > max_cubes.1 {
                max_cubes.1 = round.green;
            }

            if round.blue > max_cubes.2 {
                max_cubes.2 = round.blue;
            }
        }

        max_cubes
    }

    /// Check is the game requires more cubes than a limit
    ///
    /// Arguments:
    /// - limit: A tuple of (red, green, blue) values that represent the max for that color
    pub fn above_limit(&self, limit: (usize, usize, usize)) -> bool {
        // Check each round, making sure not to go over the max allowed for each color
        for round in &self.rounds {
            if round.red > limit.0 || round.green > limit.1 || round.blue > limit.2 {
                return true;
            }
        }

        false
    }
}

impl FromStr for Game {
    type Err = GameParseError;

    /// Parses a game from a string
    ///
    /// Example:
    /// "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" ->
    ///  Game {
    ///     id: 1
    ///     red_total: 5,
    ///     green_total: 4,
    ///     blue_total: 9,
    ///     rounds: [
    ///         GameRound { red: 4, green: 0, blue: 3 },
    ///         GameRound { red: 1, green: 2, blue: 6 },
    ///         GameRound { red: 0, green: 2, blue: 0 },
    ///     ]
    ///  }
    ///
    /// Arguments:
    /// - s: A string representing a game
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game::default();

        // split on : to get the game info and the rounds
        let mut colon_split = s.split(':');
        let game_info = colon_split.next();
        let round_info = colon_split.next();

        if game_info.is_none() || round_info.is_none() {
            return Err(Self::Err::UnknownGameFormat(String::from(s)));
        }

        // parse game info
        let mut game_split = game_info.unwrap().trim().split(' ');
        // skip the 0th element as it should just be "Game"
        let game_id = game_split.nth(1);
        if game_id.is_none() {
            return Err(Self::Err::MissingGameId(String::from(game_info.unwrap())));
        }

        if let Ok(game_id_num) = game_id.unwrap().parse::<usize>() {
            game.id = game_id_num;
        } else {
            return Err(Self::Err::ParseGameId(String::from(game_id.unwrap())));
        }

        if round_info.unwrap().is_empty() {
            return Err(Self::Err::MissingRounds(game.id.to_string()));
        }

        // split rounds on ; to get each individual round
        for round in round_info.unwrap().trim().split(';') {
            // parse rounds and add colors
            game.add_round(GameRound::from_str(round)?);
        }

        Ok(game)
    }
}

// Yoinked from the "efficient" example on rust by example
// @see https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests_day_02 {
    use std::str::FromStr;

    use super::{Game, GameRound};

    #[test]
    fn parse_game_round_from_str() {
        assert_eq!(
            GameRound::from_str("3 blue, 4 red").unwrap(),
            GameRound {
                red: 4,
                green: 0,
                blue: 3
            }
        );

        assert_eq!(
            GameRound::from_str("1 red, 2 green, 6 blue").unwrap(),
            GameRound {
                red: 1,
                green: 2,
                blue: 6
            }
        );

        assert_eq!(
            GameRound::from_str("2 green").unwrap(),
            GameRound {
                red: 0,
                green: 2,
                blue: 0
            }
        );

        assert_eq!(
            GameRound::from_str("").unwrap(),
            GameRound {
                red: 0,
                green: 0,
                blue: 0
            }
        );
    }

    #[test]
    fn parse_game_from_str() {
        assert_eq!(
            Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
            Game {
                id: 1,
                red_total: 5,
                green_total: 4,
                blue_total: 9,
                rounds: vec![
                    GameRound {
                        red: 4,
                        green: 0,
                        blue: 3
                    },
                    GameRound {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    GameRound {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ]
            }
        )
    }

    #[test]
    fn game_max_cubes() {
        let game_1 =
            Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
        assert_eq!(game_1.max_cubes(), (4, 2, 6));

        let game_2 =
            Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue")
                .unwrap();
        assert_eq!(game_2.max_cubes(), (1, 3, 4));

        let game_3 = Game::from_str(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        )
        .unwrap();
        assert_eq!(game_3.max_cubes(), (20, 13, 6));

        let game_4 = Game::from_str(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        )
        .unwrap();
        assert_eq!(game_4.max_cubes(), (14, 3, 15));

        let game_5 =
            Game::from_str("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").unwrap();
        assert_eq!(game_5.max_cubes(), (6, 3, 2));
    }
}

use clap::Parser;
use std::{fmt::Debug, fs::read_to_string, path::PathBuf, process, str::FromStr};

use thiserror::Error;

// use thiserror::Error;

#[derive(Parser)]
struct Cli {
    input_file: PathBuf,
}

fn main() {
    // Get command line arguments
    let args = Cli::parse();

    // Read file from CLI arg
    if let Ok(file) = read_to_string(&args.input_file) {
        let cards = file
            .lines()
            .flat_map(ScratchOffCard::from_str)
            .collect::<Vec<ScratchOffCard>>();

        let part_1_answer = cards.iter().fold(0, |acc, card| acc + card.points());
        let part_2_answer = 0;

        println!("Part 1: {}\nPart 2: {}", part_1_answer, part_2_answer);
    } else {
        eprintln!("Could not read file: {}", args.input_file.display());
        process::exit(1);
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct ScratchOffCard {
    pub id: usize,
    pub winning_numbers: Vec<usize>,
    pub card_numbers: Vec<usize>,
}

impl ScratchOffCard {
    /// Calculates the point value of the card
    ///
    /// To calculate the point value, you need to check for matching numbers between
    /// the winning numbers and the card numbers.
    /// The first match is worth 1 point, each additional match doubles the points
    pub fn points(&self) -> usize {
        let matches = self
            .card_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .collect::<Vec<&usize>>();

        if !matches.is_empty() {
            2_usize.pow(matches.len() as u32 - 1)
        } else {
            0
        }
    }
}

#[derive(Debug, Error)]
enum ScratchOffCardParseError {
    #[error("Unknown card format: {0}")]
    UnknownCardFormat(String),
    #[error("Unable to find Card Id: {0}")]
    MissingCardId(String),
    #[error("Unable to parse Card Id: {0}")]
    ParseCardId(String),
    #[error("Unable to find number info for Card Id: {0}")]
    MissingNumberInfo(String),
    #[error("Unknown number format for Card Id {0}: {1}")]
    UnknownNumberFormat(String, String),
}

impl FromStr for ScratchOffCard {
    type Err = ScratchOffCardParseError;

    /// Parses a ScratchOffCard from a string
    ///
    /// Example:
    /// "Card 1: 41 48 83 86 17 | 83 86 6 31 17 9 48 53" ->
    /// ScratchOffCard {
    ///     id: 1,
    ///     winning_numbers: [ 41, 48, 83, 86, 17],
    ///     card_numbers: [ 83, 86, 6, 31, 17, 9, 48, 53]
    /// }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut card = Self::default();

        // split on : to get the card info and the numbers
        let mut colon_split = s.split(':');
        let card_info = colon_split.next();
        let number_info = colon_split.next();

        if card_info.is_none() || number_info.is_none() {
            return Err(Self::Err::UnknownCardFormat(String::from(s)));
        }

        // parse card info
        let card_split = card_info.unwrap().trim().split(' ');
        // skip the 0th element as it should just be "Card"
        let card_id = card_split.skip(1).collect::<String>();
        if card_id.is_empty() {
            return Err(Self::Err::MissingCardId(String::from(card_info.unwrap())));
        }

        if let Ok(card_id_num) = card_id.trim().parse::<usize>() {
            card.id = card_id_num;
        } else {
            return Err(Self::Err::ParseCardId(String::from(card_info.unwrap())));
        }

        if number_info.unwrap().is_empty() {
            return Err(Self::Err::MissingNumberInfo(card.id.to_string()));
        }

        // split on | to get the winning and card numbers
        let mut pipe_split = number_info.unwrap().split('|');
        let winning_numbers_str = pipe_split.next();
        let card_numbers_str = pipe_split.next();

        if winning_numbers_str.is_none() || card_numbers_str.is_none() {
            return Err(Self::Err::UnknownNumberFormat(
                card.id.to_string(),
                String::from(s),
            ));
        }

        card.winning_numbers = winning_numbers_str
            .unwrap()
            .split(' ')
            .flat_map(|num| num.parse::<usize>())
            .collect::<Vec<usize>>();

        card.card_numbers = card_numbers_str
            .unwrap()
            .split(' ')
            .flat_map(|num| num.parse::<usize>())
            .collect::<Vec<usize>>();

        Ok(card)
    }
}

#[cfg(test)]
mod tests_day_03 {
    use std::str::FromStr;

    use super::ScratchOffCard;

    #[test]
    fn test_from_str() {
        assert_eq!(
            ScratchOffCard::from_str("Card 1: 41 48 83 86 17 | 83 86 6 31 17 9 48 53").unwrap(),
            ScratchOffCard {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                card_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }
        );
    }

    #[test]
    fn test_from_str_with_multiple_spaces() {
        assert_eq!(
            ScratchOffCard::from_str("Card   1: 41  3 83 86 17 | 83 86 6 31 17  9 48 53").unwrap(),
            ScratchOffCard {
                id: 1,
                winning_numbers: vec![41, 3, 83, 86, 17],
                card_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }
        );
    }

    #[test]
    fn test_points() {
        assert_eq!(
            ScratchOffCard::from_str("Card 1: 41 48 83 86 17 | 83 86 6 31 17 9 48 53")
                .unwrap()
                .points(),
            8
        );

        assert_eq!(
            ScratchOffCard::from_str("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")
                .unwrap()
                .points(),
            2
        );

        assert_eq!(
            ScratchOffCard::from_str("Card 3: 1 21 53 59 44 | 69 82 63 72 16 21 14 1")
                .unwrap()
                .points(),
            2
        );

        assert_eq!(
            ScratchOffCard::from_str("Card 4: 41 92 73 84 69 | 59 84 76 51 58 5 54 83")
                .unwrap()
                .points(),
            1
        );

        assert_eq!(
            ScratchOffCard::from_str("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")
                .unwrap()
                .points(),
            0
        );

        assert_eq!(
            ScratchOffCard::from_str("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")
                .unwrap()
                .points(),
            0
        );
    }
}

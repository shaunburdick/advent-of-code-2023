use clap::Parser;
use std::{collections::BTreeMap, fs::read_to_string, path::PathBuf, process, str::FromStr};

use thiserror::Error;

#[derive(clap::Parser)]
struct Cli {
    input_file: PathBuf,
}

fn main() {
    // Get command line arguments
    let args = Cli::parse();

    // Read file from CLI arg
    if let Ok(file) = read_to_string(&args.input_file) {
        let mut cards = file
            .lines()
            .flat_map(CamelCardHand::from_str)
            .collect::<Vec<CamelCardHand>>();

        cards.sort();

        let part_1_answer = cards
            .iter()
            .enumerate()
            .map(|(index, hand)| hand.bid * (index as u32 + 1))
            .sum::<u32>();

        let part_2_answer = 1;

        println!("Part 1: {}\nPart 2: {}", part_1_answer, part_2_answer);
    } else {
        eprintln!("Could not read file: {}", args.input_file.display());
        process::exit(1);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CamelCardHandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    High,
}

#[derive(Debug, PartialEq, Eq)]
struct CamelCardHand {
    cards: String,
    bid: u32,
    hand_type: CamelCardHandType,
}

impl Ord for CamelCardHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.type_rank().cmp(&other.type_rank()) {
            std::cmp::Ordering::Equal => {
                for (s_char, o_char) in self.cards.chars().zip(other.cards.chars()) {
                    let order =
                        CamelCardHand::card_rank(s_char).cmp(&CamelCardHand::card_rank(o_char));
                    if order != std::cmp::Ordering::Equal {
                        return order;
                    }
                }

                std::cmp::Ordering::Equal
            }
            g_or_e => g_or_e,
        }
    }
}

impl PartialOrd for CamelCardHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl CamelCardHand {
    fn new(cards: String, bid: u32) -> Self {
        let hand_type = Self::hand_type(cards.as_str());
        Self {
            cards,
            bid,
            hand_type,
        }
    }

    /// Get the hand type of a set of cards
    fn hand_type(cards: &str) -> CamelCardHandType {
        let card_groups = cards.chars().fold(BTreeMap::new(), |mut map, card| {
            *(map.entry(card).or_insert(0)) += 1;

            map
        });

        match card_groups.len() {
            1 => CamelCardHandType::FiveOfAKind,
            2 => {
                let counts = card_groups.values().collect::<Vec<&i32>>();
                if counts.contains(&&4) {
                    CamelCardHandType::FourOfAKind
                } else {
                    CamelCardHandType::FullHouse
                }
            }
            3 => {
                let counts = card_groups.values().collect::<Vec<&i32>>();
                if counts.contains(&&3) {
                    CamelCardHandType::ThreeOfAKind
                } else {
                    CamelCardHandType::TwoPair
                }
            }
            4 => CamelCardHandType::OnePair,
            _ => CamelCardHandType::High,
        }
    }

    fn type_rank(&self) -> i8 {
        match self.hand_type {
            CamelCardHandType::FiveOfAKind => 6,
            CamelCardHandType::FourOfAKind => 5,
            CamelCardHandType::FullHouse => 4,
            CamelCardHandType::ThreeOfAKind => 3,
            CamelCardHandType::TwoPair => 2,
            CamelCardHandType::OnePair => 1,
            CamelCardHandType::High => 0,
        }
    }

    fn card_rank(card: char) -> i8 {
        match card {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => 9,
            'T' => 8,
            '9' => 7,
            '8' => 6,
            '7' => 5,
            '6' => 4,
            '5' => 3,
            '4' => 2,
            '3' => 1,
            '2' => 0,
            _ => -1,
        }
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Unknown hand format: {0}")]
    Hand(String),
    #[error("Unknown number format: {0}")]
    Number(String),
}

impl FromStr for CamelCardHand {
    type Err = ParseError;

    /// Parse a hand from a hand/bid pairing
    ///
    /// Example: "222JJ 123" -> CamelCardHand { hand: "222JJ", bid: 123 }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((cards, bid)) = s.split_once(' ') {
            if let Ok(bid) = bid.parse::<u32>() {
                Ok(CamelCardHand::new(String::from(cards), bid))
            } else {
                Err(Self::Err::Number(String::from(bid)))
            }
        } else {
            Err(Self::Err::Hand(String::from(s)))
        }
    }
}

#[cfg(test)]
mod tests_day_07 {
    use std::{cmp::Ordering, str::FromStr};

    use super::{CamelCardHand, CamelCardHandType};

    #[test]
    fn test_total_winnings() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let mut cards = input
            .lines()
            .flat_map(CamelCardHand::from_str)
            .collect::<Vec<CamelCardHand>>();

        cards.sort();

        let winnings = cards
            .iter()
            .enumerate()
            .map(|(index, hand)| hand.bid * (index as u32 + 1))
            .sum::<u32>();

        assert_eq!(winnings, 6440);
    }

    #[test]
    fn test_camel_card_hand_from_str() {
        assert_eq!(
            CamelCardHand::from_str("32T3K 765").unwrap(),
            CamelCardHand::new(String::from("32T3K"), 765)
        );
    }

    #[test]
    fn test_camel_card_hand_type() {
        assert_eq!(
            CamelCardHand::hand_type("AAAAA"),
            CamelCardHandType::FiveOfAKind
        );

        assert_eq!(
            CamelCardHand::hand_type("AAAAJ"),
            CamelCardHandType::FourOfAKind
        );

        assert_eq!(
            CamelCardHand::hand_type("AAAJJ"),
            CamelCardHandType::FullHouse
        );

        assert_eq!(
            CamelCardHand::hand_type("AAAJK"),
            CamelCardHandType::ThreeOfAKind
        );

        assert_eq!(
            CamelCardHand::hand_type("AAJJK"),
            CamelCardHandType::TwoPair
        );

        assert_eq!(
            CamelCardHand::hand_type("AAJQK"),
            CamelCardHandType::OnePair
        );

        assert_eq!(CamelCardHand::hand_type("AKQJ9"), CamelCardHandType::High);
    }

    #[test]
    fn test_camel_card_hand_cmp() {
        assert_eq!(
            CamelCardHand::new(String::from("AAAAA"), 123)
                .cmp(&CamelCardHand::new(String::from("AAAAA"), 123)),
            Ordering::Equal
        );

        assert_eq!(
            CamelCardHand::new(String::from("AAAAA"), 123)
                .cmp(&CamelCardHand::new(String::from("AAAAJ"), 123)),
            Ordering::Greater
        );

        assert_eq!(
            CamelCardHand::new(String::from("23456"), 123)
                .cmp(&CamelCardHand::new(String::from("AAAAJ"), 123)),
            Ordering::Less
        );

        assert_eq!(
            CamelCardHand::new(String::from("AAAAQ"), 123)
                .cmp(&CamelCardHand::new(String::from("AAAAJ"), 123)),
            Ordering::Greater
        );
    }
}

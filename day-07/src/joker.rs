use std::{collections::BTreeMap, str::FromStr};

use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum CamelCardHandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    High,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CamelCardHand {
    pub cards: String,
    pub bid: u32,
    pub hand_type: CamelCardHandType,
}

impl Ord for CamelCardHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.type_rank().cmp(&other.type_rank()) {
            std::cmp::Ordering::Equal => {
                for (s_char, o_char) in self.cards.chars().zip(other.cards.chars()) {
                    let order = Self::card_rank(s_char).cmp(&Self::card_rank(o_char));
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
        let mut card_groups = cards.chars().fold(BTreeMap::new(), |mut map, card| {
            *(map.entry(card).or_insert(0)) += 1;

            map
        });

        // if not all jokers, remove the joker count and add it to the largest
        // group of cards
        if card_groups.len() > 1 {
            if let Some(joker_count) = card_groups.remove(&'J') {
                let (biggest_key, biggest_value) = card_groups
                    .iter()
                    .max_by(|(_, a), (_, b)| a.cmp(b))
                    .unwrap();

                card_groups.insert(*biggest_key, biggest_value + joker_count);
            }
        }

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
            'J' => 0,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            _ => -1,
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
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
                Ok(Self::new(String::from(cards), bid))
            } else {
                Err(Self::Err::Number(String::from(bid)))
            }
        } else {
            Err(Self::Err::Hand(String::from(s)))
        }
    }
}

#[cfg(test)]
mod tests_day_07_joker {
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

        assert_eq!(winnings, 5905);
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
            CamelCardHand::hand_type("AAJAA"),
            CamelCardHandType::FiveOfAKind
        );

        assert_eq!(
            CamelCardHand::hand_type("JJJJJ"),
            CamelCardHandType::FiveOfAKind
        );

        assert_eq!(
            CamelCardHand::hand_type("AAJAK"),
            CamelCardHandType::FourOfAKind
        );

        assert_eq!(
            CamelCardHand::hand_type("AAKAJ"),
            CamelCardHandType::FourOfAKind
        );

        assert_eq!(
            CamelCardHand::hand_type("AJAQQ"),
            CamelCardHandType::FullHouse
        );

        assert_eq!(
            CamelCardHand::hand_type("AATJK"),
            CamelCardHandType::ThreeOfAKind
        );

        assert_eq!(
            CamelCardHand::hand_type("AATTK"),
            CamelCardHandType::TwoPair
        );

        assert_eq!(
            CamelCardHand::hand_type("A4JQK"),
            CamelCardHandType::OnePair
        );

        assert_eq!(CamelCardHand::hand_type("AKQ89"), CamelCardHandType::High);
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
                .cmp(&CamelCardHand::new(String::from("AAAAK"), 123)),
            Ordering::Greater
        );

        assert_eq!(
            CamelCardHand::new(String::from("23456"), 123)
                .cmp(&CamelCardHand::new(String::from("AAAAK"), 123)),
            Ordering::Less
        );

        assert_eq!(
            CamelCardHand::new(String::from("AAAAQ"), 123)
                .cmp(&CamelCardHand::new(String::from("AAAAT"), 123)),
            Ordering::Greater
        );

        assert_eq!(
            CamelCardHand::new(String::from("AAAAQ"), 123)
                .cmp(&CamelCardHand::new(String::from("JAAAK"), 123)),
            Ordering::Greater
        );
    }
}
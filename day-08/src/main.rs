use clap::Parser;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char},
    error::Error,
    sequence::{preceded, separated_pair, terminated},
    Finish, IResult,
};
use std::{
    collections::BTreeMap, fmt::Debug, fs::read_to_string, path::PathBuf, process, str::FromStr,
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
        let haunted_wasteland =
            HauntedWasteland::from_str(file.as_str()).expect("Haunted Wasteland to be formated");

        let part_1_answer = haunted_wasteland.turns("AAA", "ZZZ").len();
        let part_2_answer = 1;

        println!("Part 1: {}\nPart 2: {}", part_1_answer, part_2_answer);
    } else {
        eprintln!("Could not read file: {}", args.input_file.display());
        process::exit(1);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HauntedNode {
    id: String,
    left: String,
    right: String,
}

fn parse_haunted_node(s: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alpha1,
        tag(" = "),
        separated_pair(
            preceded(char('('), alpha1),
            tag(", "),
            terminated(alpha1, char(')')),
        ),
    )(s)
}

impl FromStr for HauntedNode {
    type Err = Error<String>;

    /// Parse a Haunted Node from a string
    ///
    /// Example: "AAA = (BBB, CCC)" -> HauntedNode { id: "AAA", left: "BBB", right: "CCC" }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_haunted_node(s).finish() {
            Ok((_remaining, (id, (left, right)))) => Ok(Self {
                id: String::from(id),
                left: String::from(left),
                right: String::from(right),
            }),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]

struct HauntedMap {
    nodes: BTreeMap<String, HauntedNode>,
}

#[derive(Debug, PartialEq, Eq)]
struct HauntedDirections(String);

impl HauntedDirections {
    fn to_iter(&self) -> HauntedDirectionsIterator {
        HauntedDirectionsIterator {
            directions: self.0.as_str(),
            index: 0,
        }
    }
}

struct HauntedDirectionsIterator<'a> {
    directions: &'a str,
    index: usize,
}

impl<'a> Iterator for HauntedDirectionsIterator<'a> {
    type Item = HauntedDirection;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.directions.len() {
            self.index = 0;
        }

        if let Some(char) = self.directions.chars().nth(self.index) {
            self.index += 1;
            Some(HauntedDirection::from_char(char).expect("char to be direction"))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum HauntedDirection {
    Left,
    Right,
}

impl HauntedDirection {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            c => Err(format!("Unknown direction: {}", c)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HauntedWasteland {
    directions: HauntedDirections,
    map: HauntedMap,
}

impl HauntedWasteland {
    fn turns(&self, from: &str, target: &str) -> Vec<char> {
        let mut turns = Vec::new();

        let mut current = self.map.nodes.get(from);
        for turn in self.directions.to_iter() {
            match turn {
                HauntedDirection::Left => {
                    turns.push('L');
                    if let Some(c) = current {
                        if c.left == target {
                            return turns;
                        } else {
                            current = self.map.nodes.get(&c.left);
                        }
                    }
                }
                HauntedDirection::Right => {
                    turns.push('R');
                    if let Some(c) = current {
                        if c.right == target {
                            return turns;
                        } else {
                            current = self.map.nodes.get(&c.right);
                        }
                    }
                }
            }
        }

        turns
    }
}

impl FromStr for HauntedWasteland {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        // first line directions
        let directions = HauntedDirections(String::from(lines.next().unwrap()));

        // skip blank line
        lines.next();

        let map = HauntedMap {
            nodes: lines.fold(BTreeMap::new(), |mut map, line| {
                let node = HauntedNode::from_str(line).expect("line to be node");
                map.insert(node.id.clone(), node);

                map
            }),
        };

        Ok(Self { map, directions })
    }
}

#[cfg(test)]
mod tests_day_08 {
    use std::{collections::BTreeMap, str::FromStr};

    use super::{HauntedDirections, HauntedMap, HauntedNode, HauntedWasteland};

    #[test]
    fn test_haunted_node_from_str() {
        assert_eq!(
            HauntedNode::from_str("AAA = (BBB, CCC)"),
            Ok(HauntedNode {
                id: String::from("AAA"),
                left: String::from("BBB"),
                right: String::from("CCC")
            })
        );

        assert_eq!(
            HauntedNode::from_str("BBB = (DDD, EEE)"),
            Ok(HauntedNode {
                id: String::from("BBB"),
                left: String::from("DDD"),
                right: String::from("EEE")
            })
        );
    }

    #[test]
    fn test_haunted_wasteland_from_str() {
        assert_eq!(
            HauntedWasteland::from_str(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            Ok(HauntedWasteland {
                directions: HauntedDirections(String::from("RL")),
                map: HauntedMap {
                    nodes: BTreeMap::from([
                        (
                            String::from("AAA"),
                            HauntedNode::from_str("AAA = (BBB, CCC)").unwrap()
                        ),
                        (
                            String::from("BBB"),
                            HauntedNode::from_str("BBB = (DDD, EEE)").unwrap()
                        ),
                        (
                            String::from("CCC"),
                            HauntedNode::from_str("CCC = (ZZZ, GGG)").unwrap()
                        ),
                        (
                            String::from("DDD"),
                            HauntedNode::from_str("DDD = (DDD, DDD)").unwrap()
                        ),
                        (
                            String::from("EEE"),
                            HauntedNode::from_str("EEE = (EEE, EEE)").unwrap()
                        ),
                        (
                            String::from("GGG"),
                            HauntedNode::from_str("GGG = (GGG, GGG)").unwrap()
                        ),
                        (
                            String::from("ZZZ"),
                            HauntedNode::from_str("ZZZ = (ZZZ, ZZZ)").unwrap()
                        ),
                    ])
                }
            })
        );
    }

    #[test]
    fn test_haunted_wasteland_turns() {
        let haunted_wasteland = HauntedWasteland::from_str(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        )
        .unwrap();

        assert_eq!(haunted_wasteland.turns("AAA", "ZZZ"), vec!['R', 'L']);

        let haunted_wasteland = HauntedWasteland::from_str(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        )
        .unwrap();

        assert_eq!(
            haunted_wasteland.turns("AAA", "ZZZ"),
            vec!['L', 'L', 'R', 'L', 'L', 'R',]
        );
    }
}

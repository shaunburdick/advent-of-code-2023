use clap::Parser;
use std::{collections::BTreeMap, fs::read_to_string, path::PathBuf, process, str::FromStr};

use thiserror::Error;

#[derive(Parser)]
struct Cli {
    input_file: PathBuf,
}

fn main() {
    // Get command line arguments
    let args = Cli::parse();

    // Read file from CLI arg
    if let Ok(file) = read_to_string(&args.input_file) {
        let almanac = Almanac::from_str(&file).expect("almanac to parse");

        let locations = almanac
            .seeds
            .iter()
            .flat_map(|seed| almanac.get_type_value(*seed, "location"))
            .collect::<Vec<_>>();

        let part_1_answer = locations.iter().min().unwrap();
        let part_2_answer = 1;

        println!("Part 1: {}\nPart 2: {}", part_1_answer, part_2_answer);
    } else {
        eprintln!("Could not read file: {}", args.input_file.display());
        process::exit(1);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SeedRelationRange {
    source_start: usize,
    destination_start: usize,
    range: usize,
}

impl SeedRelationRange {
    /// Get a range map value if it exists in the range
    ///
    /// Arguments:
    /// - id: The id to look for
    fn get_destination(&self, id: &usize) -> Option<usize> {
        if id < &self.source_start || id > &(self.source_start + self.range) {
            return None;
        }

        Some(self.destination_start + (id - self.source_start))
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Unknown range format: {0}")]
    Range(String),
    #[error("Unknown number format: {0}")]
    Number(String),
    #[error("Unknown map title format: {0}")]
    MapTitle(String),
    #[error("Unknown seed id format: {0}")]
    SeedId(String),
}

impl FromStr for SeedRelationRange {
    type Err = ParseError;

    /// Parse a range from a string
    ///
    /// Example:
    /// 50 98 2 ->
    /// SeedRelationRange {
    ///     source: 98,
    ///     destination: 50,
    ///     range: 2
    /// }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let str_destination = split.next();
        let str_source = split.next();
        let str_range = split.next();

        if let (Some(destination), Some(source), Some(range)) =
            (str_destination, str_source, str_range)
        {
            // parse numbers
            if let (Ok(destination_start), Ok(source_start), Ok(range)) = (
                destination.parse::<usize>(),
                source.parse::<usize>(),
                range.parse::<usize>(),
            ) {
                Ok(Self {
                    destination_start,
                    source_start,
                    range,
                })
            } else {
                Err(Self::Err::Number(String::from(s)))
            }
        } else {
            Err(Self::Err::Range(String::from(s)))
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct SeedRelationTable {
    from: String,
    to: String,
    ranges: Vec<SeedRelationRange>,
}

impl SeedRelationTable {
    /// Get the relational value of the id
    /// If its in the range, do the translation
    /// otherwise the id stays the same
    fn get_relation(&self, id: &usize) -> usize {
        for range in &self.ranges {
            if let Some(dest) = range.get_destination(id) {
                return dest;
            }
        }

        *id
    }
}

impl FromStr for SeedRelationTable {
    type Err = ParseError;

    /// Parse from table title
    ///
    /// Example:
    /// "foo-to-bar map:\n1 2 3" ->
    /// SeedRelationTable {
    ///     from: "foo",
    ///     to: "bar",
    ///     ranges: [
    ///         SeedRelationRange {
    ///             source_start: 2,
    ///             destination_start: 1,
    ///             range: 3
    ///         }
    ///     ]
    /// }
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let seed_line = lines.next();
        if seed_line.is_none() {
            return Err(Self::Err::MapTitle(String::new()));
        }

        let mut table = Self::default();

        if let Some(title) = seed_line.unwrap().split_ascii_whitespace().next() {
            let mut to_split = title.split('-');
            let from = to_split.next();
            let to = to_split.last();

            if let (Some(from), Some(to)) = (from, to) {
                table.from = String::from(from);
                table.to = String::from(to);
            } else {
                return Err(Self::Err::MapTitle(String::from(title)));
            }
        } else {
            return Err(Self::Err::MapTitle(String::from(seed_line.unwrap())));
        }

        for line in lines {
            table.ranges.push(SeedRelationRange::from_str(line)?);
        }

        Ok(table)
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<usize>,
    tables: BTreeMap<String, SeedRelationTable>,
}

impl Almanac {
    /// Parse seeds from string
    ///
    /// Example:
    /// "seeds: 79 14 55 13" -> [79, 14, 55, 13]
    fn parse_seeds(&mut self, s: &str) -> Result<(), ParseError> {
        for item in s.split_ascii_whitespace() {
            match item.trim() {
                "seeds:" => {} // do nothing
                num if num.parse::<usize>().is_ok() => {
                    self.seeds.push(num.parse::<usize>().unwrap());
                }
                _ => return Err(ParseError::SeedId(String::from(item))),
            }
        }

        Ok(())
    }

    /// Get type value for a seed
    /// Follows the mapping, until it find a the type value for a seed id
    ///
    /// Arguments:
    /// - seed_id: The id of the seed
    /// - type_name: The name of the type to look for
    fn get_type_value(&self, seed_id: usize, type_name: &str) -> Option<usize> {
        let mut current_type = Some("seed");
        let mut current_id = seed_id;
        while current_type.is_some() {
            if let Some(table) = self.tables.get(current_type.unwrap()) {
                current_id = table.get_relation(&current_id);
                current_type.replace(&table.to);
                if current_type == Some(type_name) {
                    return Some(current_id);
                }
            } else {
                current_type = None;
            }
        }

        None
    }
}

impl FromStr for Almanac {
    type Err = ParseError;

    /// Parse an Almanac from a textual representation of an almanac
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut almanac = Almanac::default();

        let mut groups = s.split("\n\n");
        almanac
            .parse_seeds(groups.next().unwrap())
            .expect("seeds to parse");

        for group in groups {
            let table = SeedRelationTable::from_str(group)?;
            almanac.tables.insert(table.from.clone(), table);
        }

        Ok(almanac)
    }
}

#[cfg(test)]
mod tests_day_05 {
    use std::str::FromStr;

    use super::{Almanac, SeedRelationRange, SeedRelationTable};

    #[test]
    fn test_seed_relation_range_from_str() {
        assert_eq!(
            SeedRelationRange::from_str("50 98 2").unwrap(),
            SeedRelationRange {
                source_start: 98,
                destination_start: 50,
                range: 2
            }
        );
    }

    #[test]
    fn test_seed_relation_range_destination() {
        let range = SeedRelationRange::from_str("50 98 2").unwrap();

        // Not in range
        assert_eq!(range.get_destination(&1), None);

        // In range
        assert_eq!(range.get_destination(&98), Some(50));
        assert_eq!(range.get_destination(&99), Some(51));
    }

    #[test]
    fn test_seed_relation_table_from_str() {
        assert_eq!(
            SeedRelationTable::from_str("seed-to-soil map:").unwrap(),
            SeedRelationTable {
                from: String::from("seed"),
                to: String::from("soil"),
                ranges: Vec::new()
            }
        );

        assert_eq!(
            SeedRelationTable::from_str("seed-to-soil map:\n0 15 37").unwrap(),
            SeedRelationTable {
                from: String::from("seed"),
                to: String::from("soil"),
                ranges: vec![SeedRelationRange::from_str("0 15 37").unwrap()]
            }
        );
    }

    #[test]
    fn test_almanac_parse_seeds() {
        let mut almanac = Almanac::default();

        assert!(almanac.parse_seeds("seeds: 79 14 55 13").is_ok());
        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_almanac_get_type_value() {
        let mut almanac = Almanac::default();
        almanac
            .parse_seeds("seeds: 79 14 55 13")
            .expect("should parse seeds");

        almanac.tables.insert(
            String::from("seed"),
            SeedRelationTable {
                from: String::from("seed"),
                to: String::from("soil"),
                ranges: vec![
                    SeedRelationRange::from_str("50 98 2").unwrap(),
                    SeedRelationRange::from_str("52 50 48").unwrap(),
                ],
            },
        );

        almanac.tables.insert(
            String::from("soil"),
            SeedRelationTable {
                from: String::from("soil"),
                to: String::from("fertilizer"),
                ranges: vec![
                    SeedRelationRange::from_str("0 15 37").unwrap(),
                    SeedRelationRange::from_str("37 52 2").unwrap(),
                    SeedRelationRange::from_str("39 0 15").unwrap(),
                ],
            },
        );

        assert_eq!(almanac.get_type_value(79, "fertilizer"), Some(81));
    }

    #[test]
    fn test_almanac_from_str() {
        let almanac = Almanac::from_str(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        )
        .unwrap();

        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);
        assert_eq!(almanac.tables.len(), 7); // todo: check all the table values?
    }
}

use clap::Parser;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    fs::read_to_string,
    path::PathBuf,
    process,
};

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
        let items = ItemMaps::from_map(file);
        let part_1_answer = items.get_part_numbers().iter().sum::<usize>();
        let part_2_answer = items.get_gear_ratios().iter().sum::<usize>();

        println!("Part 1: {}\nPart 2: {}", part_1_answer, part_2_answer);
    } else {
        eprintln!("Could not read file: {}", args.input_file.display());
        process::exit(1);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ItemPoint {
    pub column: i32,
    pub row: i32,
}

impl ItemPoint {
    /// Get all the surrounding points
    ///
    /// -1 -1, -1 0, -1 1
    ///  0 -1,  0 0,  0 1
    ///  1 -1,  1 0,  1 1
    pub fn get_surrounding_points(&self) -> Vec<ItemPoint> {
        let mut points = Vec::new();

        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for offset in offsets {
            points.push(ItemPoint {
                column: self.column + offset.1,
                row: self.row + offset.0,
            });
        }

        points
    }
}

#[derive(Debug, Default)]
struct ItemMaps {
    pub symbols: BTreeMap<ItemPoint, char>,
    pub numbers: BTreeMap<ItemPoint, usize>,
}

impl ItemMaps {
    /// Generate from a textual representation of the map
    pub fn from_map(map: String) -> ItemMaps {
        let mut items = ItemMaps::default();

        for (row, line) in map.lines().enumerate() {
            let mut current_number = String::new();
            for (column, character) in line.chars().enumerate() {
                // If . wrap up any number and move to next item
                if character == '.' {
                    current_number.clear();
                    continue;
                } else if character.is_ascii_digit() {
                    current_number.push(character);

                    // we've come to the end of a number
                    // Add the parsed number to the map
                    // We know it will parse as we have checked each digit
                    let parsed_num = current_number.parse::<usize>().unwrap();

                    // Add the number to each coordinate in its range
                    for offset in 0..current_number.len() {
                        items.numbers.insert(
                            ItemPoint {
                                column: column as i32 - offset as i32,
                                row: row as i32,
                            },
                            parsed_num,
                        );
                    }
                } else {
                    // it must be a symbol
                    items.symbols.insert(
                        ItemPoint {
                            column: column as i32,
                            row: row as i32,
                        },
                        character,
                    );
                    current_number.clear();
                }
            }
        }

        items
    }

    /// Get part numbers for a map
    /// A part number is valid if its positionally next to a symbol
    pub fn get_part_numbers(&self) -> Vec<usize> {
        let mut part_numbers = Vec::new();

        for point in self.symbols.keys() {
            let surrounding_points = point.get_surrounding_points();
            let mut matching_numbers = BTreeSet::new();
            for surrounding_point in surrounding_points {
                if let Some(part_number) = self.numbers.get(&surrounding_point) {
                    matching_numbers.insert(*part_number);
                }
            }
            // add any numbers to the result
            part_numbers.extend(matching_numbers.iter());
        }

        part_numbers
    }

    /// Get gear ratios for a map
    /// A gear is the '*' symbol, as long as only two numbers are near it
    pub fn get_gear_ratios(&self) -> Vec<usize> {
        let mut gear_ratios = Vec::new();

        for (point, _) in self.symbols.iter().filter(|s| *s.1 == '*') {
            let surrounding_points = point.get_surrounding_points();
            let mut matching_numbers = BTreeSet::new();
            for surrounding_point in surrounding_points {
                if let Some(part_number) = self.numbers.get(&surrounding_point) {
                    matching_numbers.insert(*part_number);
                }
            }
            // add any numbers to the result as long as there are just two
            if matching_numbers.len() == 2 {
                gear_ratios.push(matching_numbers.iter().product());
            }
        }

        gear_ratios
    }
}

#[cfg(test)]
mod tests_day_03 {
    use super::{ItemMaps, ItemPoint};
    use std::collections::BTreeMap;

    #[test]
    fn test_from_map() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let symbols: BTreeMap<ItemPoint, char> = BTreeMap::from([
            (ItemPoint { row: 1, column: 3 }, '*'),
            (ItemPoint { row: 3, column: 6 }, '#'),
            (ItemPoint { row: 4, column: 3 }, '*'),
            (ItemPoint { row: 5, column: 5 }, '+'),
            (ItemPoint { row: 8, column: 3 }, '$'),
            (ItemPoint { row: 8, column: 5 }, '*'),
        ]);

        let numbers: BTreeMap<ItemPoint, usize> = BTreeMap::from([
            (ItemPoint { row: 0, column: 0 }, 467),
            (ItemPoint { row: 0, column: 1 }, 467),
            (ItemPoint { row: 0, column: 2 }, 467),
            (ItemPoint { row: 0, column: 5 }, 114),
            (ItemPoint { row: 0, column: 6 }, 114),
            (ItemPoint { row: 0, column: 7 }, 114),
            (ItemPoint { row: 2, column: 2 }, 35),
            (ItemPoint { row: 2, column: 3 }, 35),
            (ItemPoint { row: 2, column: 6 }, 633),
            (ItemPoint { row: 2, column: 7 }, 633),
            (ItemPoint { row: 2, column: 8 }, 633),
            (ItemPoint { row: 4, column: 0 }, 617),
            (ItemPoint { row: 4, column: 1 }, 617),
            (ItemPoint { row: 4, column: 2 }, 617),
            (ItemPoint { row: 5, column: 7 }, 58),
            (ItemPoint { row: 5, column: 8 }, 58),
            (ItemPoint { row: 6, column: 2 }, 592),
            (ItemPoint { row: 6, column: 3 }, 592),
            (ItemPoint { row: 6, column: 4 }, 592),
            (ItemPoint { row: 7, column: 6 }, 755),
            (ItemPoint { row: 7, column: 7 }, 755),
            (ItemPoint { row: 7, column: 8 }, 755),
            (ItemPoint { row: 9, column: 1 }, 664),
            (ItemPoint { row: 9, column: 2 }, 664),
            (ItemPoint { row: 9, column: 3 }, 664),
            (ItemPoint { row: 9, column: 5 }, 598),
            (ItemPoint { row: 9, column: 6 }, 598),
            (ItemPoint { row: 9, column: 7 }, 598),
        ]);

        let items = ItemMaps::from_map(String::from(input));

        assert_eq!(items.symbols, symbols);
        assert_eq!(items.numbers, numbers);
    }

    #[test]
    fn test_get_surrounding_points() {
        assert_eq!(
            ItemPoint { row: 4, column: 0 }.get_surrounding_points(),
            vec![
                ItemPoint { row: 3, column: -1 }, // -1 -1
                ItemPoint { row: 3, column: 0 },  // -1  0
                ItemPoint { row: 3, column: 1 },  // -1  1
                ItemPoint { row: 4, column: -1 }, //  0 -1
                ItemPoint { row: 4, column: 1 },  //  0  1
                ItemPoint { row: 5, column: -1 }, //  1 -1
                ItemPoint { row: 5, column: 0 },  //  1  0
                ItemPoint { row: 5, column: 1 },  //  1  1
            ]
        )
    }

    #[test]
    fn test_get_part_numbers() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let items = ItemMaps::from_map(String::from(input));

        assert_eq!(
            items.get_part_numbers(), // [35, 467, 617, 664, 592, 598, 755, 633]
            vec![35, 467, 617, 664, 592, 598, 755, 633]
        )
    }

    #[test]
    fn test_get_gear_ratios() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let items = ItemMaps::from_map(String::from(input));

        assert_eq!(
            items.get_gear_ratios(), // [16345, 451490]
            vec![16345, 451490]
        )
    }
}

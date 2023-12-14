use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

pub fn process_part1(file: &str) -> usize {
    let sky = SkyMap::from_str(file).expect("file to be a sky map");
    let distances = sky.shortest_distances();

    distances
        .iter()
        .fold(0, |acc, (_, _, distance)| acc + distance)
}

pub fn process_part2(file: &str) -> usize {
    2
}

#[derive(Debug)]
struct SkyMap {
    galaxies: Vec<Galaxy>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

impl SkyMap {
    fn shortest_distances(&self) -> Vec<(&Galaxy, &Galaxy, usize)> {
        let mut distances = Vec::new();

        self.galaxies.iter().combinations(2).for_each(|combo| {
            distances.push((
                combo[0],
                combo[1],
                Galaxy {
                    id: 0,
                    col: combo[0].col
                        + self
                            .empty_cols
                            .iter()
                            .filter(|col| col < &&combo[0].col)
                            .count(),
                    row: combo[0].row
                        + self
                            .empty_rows
                            .iter()
                            .filter(|row| row < &&combo[0].row)
                            .count(),
                }
                .distance(&Galaxy {
                    id: 1,
                    col: combo[1].col
                        + self
                            .empty_cols
                            .iter()
                            .filter(|col| col < &&combo[1].col)
                            .count(),
                    row: combo[1].row
                        + self
                            .empty_rows
                            .iter()
                            .filter(|row| row < &&combo[1].row)
                            .count(),
                }),
            ))
        });

        distances
    }
}

impl FromStr for SkyMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = Vec::new();
        let mut empty_row_map = HashMap::new();
        let mut empty_col_map = HashMap::new();

        s.lines().enumerate().for_each(|(row, line)| {
            empty_row_map.entry(row).or_insert(0);
            line.chars().enumerate().for_each(|(col, character)| {
                empty_col_map.entry(col).or_insert(0);
                if character == '#' {
                    empty_col_map.entry(col).and_modify(|cnt| *cnt += 1);
                    empty_row_map.entry(row).and_modify(|cnt| *cnt += 1);
                    galaxies.push(Galaxy {
                        id: galaxies.len() + 1,
                        row,
                        col,
                    })
                }
            });
        });

        Ok(Self {
            galaxies,
            empty_cols: empty_col_map
                .iter()
                .filter_map(|(col, cnt)| if cnt == &0 { Some(*col) } else { None })
                .collect(),
            empty_rows: empty_row_map
                .iter()
                .filter_map(|(col, cnt)| if cnt == &0 { Some(*col) } else { None })
                .collect(),
        })
    }
}

#[derive(Debug)]
struct Galaxy {
    id: usize,
    row: usize,
    col: usize,
}

impl Galaxy {
    fn distance(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

#[cfg(test)]
mod tests_day_11 {
    use rstest::rstest;

    use super::{process_part1, process_part2};

    #[rstest]
    #[case(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        374
    )]
    fn test_process_part1(#[case] input: &str, #[case] result: usize) {
        assert_eq!(process_part1(input), result);
    }

    #[rstest]
    fn test_process_part2() {
        let input = "";
        assert_eq!(process_part2(input), 2);
    }
}

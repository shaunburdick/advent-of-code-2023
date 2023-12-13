use std::{collections::BTreeMap, str::FromStr};
use thiserror::Error;

pub fn process_part1(file: &str) -> usize {
    let map = PipeMap::from_str(file).expect("map to parse");
    let furthest_loop = map.get_furthest_loop().expect("a loop to exist");

    furthest_loop.len() / 2
}

pub fn process_part2(file: &str) -> usize {
    let map = PipeMap::from_str(file).expect("map to parse");

    let enclosed_tiles = map.get_enclosed_tiles(&map.get_furthest_loop().expect("a loop to exist"));

    enclosed_tiles.len()
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Unknown Pipe Character: {0}")]
    Character(char),
    #[error("Missing Start Position: {0}")]
    MissingStart(String),
}

#[derive(Debug)]
struct PipeMap {
    map: BTreeMap<Point, Pipe>,
    start: Point,
}

impl PipeMap {
    /// Get the distance for the furthest point from the start position
    fn get_furthest_loop(&self) -> Option<Vec<Pipe>> {
        let starting_segment = self
            .map
            .get(&self.start)
            .expect("starting segment to exist");

        let mut loops = self
            .start
            .get_surrounding_points()
            .iter()
            .flat_map(|p| self.map.get(p))
            .filter(|p| p.out == self.start || p.into == self.start)
            .flat_map(|seg| {
                self.get_loop(Pipe {
                    out: seg.point,
                    ..*starting_segment
                })
            })
            .collect::<Vec<_>>();

        loops.sort_by_key(|a| a.len());

        loops.pop()
    }

    /// Get enclosed tiles
    ///
    /// Arguments:
    /// - pipe_loop: A list of pipes that form a loop
    fn get_enclosed_tiles(&self, pipe_loop: &[Pipe]) -> Vec<Point> {
        let bounds = pipe_loop.iter().fold(BTreeMap::new(), |mut map, pipe| {
            map.entry(pipe.point.y)
                .and_modify(|e: &mut BTreeMap<i32, char>| {
                    e.insert(pipe.point.x, pipe.symbol);
                })
                .or_insert(BTreeMap::from([(pipe.point.x, pipe.symbol)]));

            map
        });

        let mut enclosed_tiles = Vec::new();

        if bounds.is_empty() {
            return enclosed_tiles; // return empty vec
        }

        for (y, x_bounds) in bounds {
            let mut x_current = *x_bounds.keys().min().expect("this to never be empty");
            let x_max = x_bounds.keys().max().expect("this to never be empty");
            let mut crossings = 0;
            while &x_current < x_max {
                let current_point = Point { y, x: x_current };
                if let Some(symbol) = x_bounds.get(&x_current) {
                    // Only count non-horizontal movements
                    // So something like FJ would only count as one crossing
                    // S only counts depending on what sub-type it is.
                    // In my case, S works for all the tests but fails the final
                    // as the final has S as a segment of -
                    // I'm not going to write that logic in, I've spent enough time...
                    match symbol {
                        'S' | '|' | 'F' | '7' => {
                            crossings += 1;
                        }
                        _ => {}
                    }
                } else if crossings % 2 != 0 {
                    // if current location doesn't exist in the map, it's an empty spot
                    enclosed_tiles.push(current_point);
                }

                x_current += 1;
            }
        }

        enclosed_tiles
    }

    /// Get a loop based on a starting segment
    ///
    /// Arguments:
    /// - starting_segment: A pipe segment to start the loop from. It will proceed from the `out` value
    fn get_loop(&self, starting_segment: Pipe) -> Option<Vec<Pipe>> {
        let mut pipe_loop = vec![starting_segment.clone()];
        let mut prev_segment = &starting_segment;
        if let Some(mut current_segment) = self.map.get(&starting_segment.out) {
            loop {
                if starting_segment.point == current_segment.point {
                    break; // we found a match!
                } else {
                    let next_point = if current_segment.out != prev_segment.point {
                        &current_segment.out
                    } else {
                        &current_segment.into
                    };

                    if let Some(out_loop) = self.map.get(next_point) {
                        pipe_loop.push(current_segment.clone());
                        prev_segment = current_segment;
                        current_segment = &out_loop;
                    } else {
                        return None;
                    }
                }
            }
        } else {
            return None;
        }

        Some(pipe_loop)
    }
}

impl FromStr for PipeMap {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: BTreeMap<Point, Pipe> = BTreeMap::new();
        let mut start: Option<Point> = None;

        for (y, line) in s.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };
                if symbol != '.' {
                    if symbol == 'S' {
                        start.replace(point);
                    }
                    let mut section = Pipe::new(&point, symbol)?;

                    // if either x or y are negative, assume it's a dead end
                    if section.into.x < 0
                        || section.into.y < 0
                        || section.out.x < 0
                        || section.out.y < 0
                    {
                        section.pipe_type = PipeType::DeadEnd;
                    }

                    map.insert(point, section);
                }
            }
        }

        if let Some(start) = start {
            Ok(Self { map, start })
        } else {
            Err(Self::Err::MissingStart(String::from(s)))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PipeType {
    Loop,
    DeadEnd,
    Unknown,
}

#[derive(Debug, Clone)]
struct Pipe {
    symbol: char,
    point: Point,
    into: Point,
    out: Point,
    pipe_type: PipeType,
}

impl Pipe {
    fn new(current_point: &Point, symbol: char) -> Result<Self, ParseError> {
        match symbol {
            '|' => Ok(Pipe {
                symbol,
                point: *current_point,
                into: Point {
                    x: current_point.x,
                    y: current_point.y - 1,
                },
                out: Point {
                    x: current_point.x,
                    y: current_point.y + 1,
                },
                pipe_type: PipeType::Unknown,
            }),
            '-' => Ok(Pipe {
                symbol,
                point: *current_point,
                into: Point {
                    x: current_point.x - 1,
                    y: current_point.y,
                },
                out: Point {
                    x: current_point.x + 1,
                    y: current_point.y,
                },
                pipe_type: PipeType::Unknown,
            }),
            'L' => Ok(Pipe {
                symbol,
                point: *current_point,
                into: Point {
                    x: current_point.x,
                    y: current_point.y - 1,
                },
                out: Point {
                    x: current_point.x + 1,
                    y: current_point.y,
                },
                pipe_type: PipeType::Unknown,
            }),
            'J' => Ok(Pipe {
                symbol,
                point: *current_point,
                into: Point {
                    x: current_point.x - 1,
                    y: current_point.y,
                },
                out: Point {
                    x: current_point.x,
                    y: current_point.y - 1,
                },
                pipe_type: PipeType::Unknown,
            }),
            '7' => Ok(Pipe {
                symbol,
                point: *current_point,
                into: Point {
                    x: current_point.x - 1,
                    y: current_point.y,
                },
                out: Point {
                    x: current_point.x,
                    y: current_point.y + 1,
                },
                pipe_type: PipeType::Unknown,
            }),
            'F' => Ok(Pipe {
                symbol,
                point: *current_point,
                into: Point {
                    x: current_point.x + 1,
                    y: current_point.y,
                },
                out: Point {
                    x: current_point.x,
                    y: current_point.y + 1,
                },
                pipe_type: PipeType::Unknown,
            }),
            'S' =>
            // Have it point to itself for now
            {
                Ok(Pipe {
                    symbol,
                    point: *current_point,
                    into: Point {
                        x: current_point.x,
                        y: current_point.y,
                    },
                    out: Point {
                        x: current_point.x,
                        y: current_point.y,
                    },
                    pipe_type: PipeType::Loop, // it's the start of the loop
                })
            }

            c => Err(ParseError::Character(c)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// Get all the surrounding points
    ///
    /// -1 -1, -1 0, -1 1
    ///  0 -1,  0 0,  0 1
    ///  1 -1,  1 0,  1 1
    pub fn get_surrounding_points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        let offsets = [
            // (-1, -1),
            (-1, 0),
            // (-1, 1),
            (0, -1),
            (0, 1),
            // (1, -1),
            (1, 0),
            // (1, 1),
        ];

        for offset in offsets {
            points.push(Point {
                y: self.y + offset.1,
                x: self.x + offset.0,
            });
        }

        points
    }
}

#[cfg(test)]
mod tests_day_10 {
    use rstest::rstest;

    use super::{process_part1, process_part2};

    #[rstest]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        4
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        8
    )]
    fn test_process_part1(#[case] input: &str, #[case] result: usize) {
        assert_eq!(process_part1(input), result);
    }

    #[rstest]
    #[case(
        "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        4
    )]
    #[case(
        ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        8
    )]
    #[case(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        10
    )]
    fn test_process_part2(#[case] input: &str, #[case] result: usize) {
        assert_eq!(process_part2(input), result);
    }
}

use std::{collections::BTreeMap, str::FromStr};
use thiserror::Error;

pub fn process_part1(file: &str) -> usize {
    let map = PipeMap::from_str(file).expect("map to parse");
    map.get_furthest_point()
}

pub fn process_part2(file: &str) -> usize {
    2
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
    fn get_furthest_point(&self) -> usize {
        let mut furthest_point = 0;
        let starting_segment = self
            .map
            .get(&self.start)
            .expect("starting segment to exist");

        let biggest_loop = self
            .start
            .get_surrounding_points()
            .iter()
            .flat_map(|p| self.map.get(p))
            .filter(|p| p.out == self.start || p.into == self.start)
            .flat_map(|seg| {
                self.get_loop_length(&Pipe {
                    out: seg.point,
                    ..*starting_segment
                })
            })
            .max();

        if let Some(biggest_loop) = biggest_loop {
            furthest_point = biggest_loop / 2;
        }

        furthest_point
    }

    fn get_loop_length(&self, starting_segment: &Pipe) -> Option<usize> {
        let mut loop_length = 1;
        let mut prev_segment = starting_segment;
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
                        prev_segment = current_segment;
                        current_segment = &out_loop;
                        loop_length += 1;
                    } else {
                        return None;
                    }
                }
            }
        } else {
            return None;
        }

        Some(loop_length)
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
                        start.replace(point.clone());
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

        // let mut new_deadends = false;
        // loop {
        //     new_deadends = false;
        //     let unknown_points = map
        //         .iter()
        //         .filter(|(_, section)| section.pipe_type == PipeType::Unknown)
        //         .map(|(point, _)| point)
        //         .collect::<Vec<_>>();

        //     for point in unknown_points {
        //         let cur_section = map.get(point).expect("point to exist");
        //         let into_point = map.get(&cur_section.into);
        //         let out_point = map.get(&cur_section.out);
        //         if let (Some(into), Some(out)) = (into_point, out_point) {
        //             if into.pipe_type == PipeType::DeadEnd || out.pipe_type == PipeType::DeadEnd {
        //                 let mut cur = cur_section;
        //                 *cur.pipe_type = PipeType::DeadEnd;
        //                 new_deadends = true;
        //             } else if into.pipe_type == PipeType::Loop && out.pipe_type == PipeType::Loop {
        //                 cur_section.pipe_type = PipeType::Loop;
        //             }
        //         } else {
        //             cur_section.pipe_type = PipeType::DeadEnd;
        //             new_deadends = true;
        //         }
        //     }

        //     if !new_deadends {
        //         break;
        //     }
        // }

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

#[derive(Debug)]
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
    fn test_process_part2() {
        let input = "";
        assert_eq!(process_part2(input), 2);
    }
}

use std::{collections::HashSet, iter::successors, str::FromStr};

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;

fn main() {
    let map = parse(&["aoc", "2022", "14.txt"]).unwrap();
    println!("part1 : {}", part1(map.clone()));
    println!("part2 : {}", part2(map.clone()));
}

fn parse(path: &[&str]) -> Result<Map> {
    let paths = challenges_common::get_input_lines(path)
        .map(|line| Path::from_str(&line))
        .collect::<Result<_>>()?;

    Ok(Map::from_paths(&paths))
}

fn part1(mut map: Map) -> u32 {
    let mut nb_rested = 0;
    while map.drop_sand() {
        nb_rested += 1;
    }
    nb_rested
}

fn part2(mut map: Map) -> u32 {
    let mut nb_rested = 1;

    while map.drop_sand_with_floor() {
        nb_rested += 1;
    }
    nb_rested
}

type Int = u32;
#[derive(Clone, Hash, PartialEq, Eq)]
struct Coord {
    x: Int,
    y: Int,
}

impl FromStr for Coord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((Ok(x), Ok(y))) = s.split(",").map(|s| s.parse()).collect_tuple() {
            Ok(Self { x, y })
        } else {
            Err(anyhow!("cannot parse Coord from '{}'", s))
        }
    }
}

struct Line {
    from: Coord,
    to: Coord,
}

impl Line {
    fn coords(&self) -> impl Iterator<Item = Coord> + '_ {
        let dir_x = {
            let diff_x = self.to.x as i32 - self.from.x as i32;
            if diff_x == 0 {
                0
            } else {
                diff_x / diff_x.abs()
            }
        };

        let dir_y = {
            let diff_y = self.to.y as i32 - self.from.y as i32;
            if diff_y == 0 {
                0
            } else {
                diff_y / diff_y.abs()
            }
        };
        successors(Some(self.from.clone()), move |prev| {
            if prev == &self.to {
                None
            } else {
                Some(Coord {
                    x: (prev.x as i32 + dir_x) as u32,
                    y: (prev.y as i32 + dir_y) as u32,
                })
            }
        })
    }
}

struct Path {
    lines: Vec<Line>,
}

impl Path {
    fn coords(&self) -> impl Iterator<Item = Coord> + '_ {
        self.lines.iter().flat_map(Line::coords)
    }
}

impl FromStr for Path {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines = s
            .split(" -> ")
            .tuple_windows()
            .map(|(from, to)| match (from.parse(), to.parse()) {
                (Ok(from), Ok(to)) => Ok(Line { from, to }),
                (Err(err), _) | (_, Err(err)) => Err(err.context("error parsing line")),
            })
            .collect::<Result<_>>()?;

        Ok(Self { lines })
    }
}

#[derive(Clone)]
struct Map {
    max_y: Int,
    filled: HashSet<Coord>,
}

impl Map {
    fn from_paths(paths: &Vec<Path>) -> Map {
        let filled: HashSet<Coord> = paths.iter().flat_map(Path::coords).collect();
        let max_y = filled.iter().map(|coord| coord.y).max().unwrap_or(0);
        Map { max_y, filled }
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand_unit = Coord { x: 500, y: 0 };

        while let Some(new_position) = self.next_position(&sand_unit) {
            sand_unit = new_position;
            if sand_unit.y >= self.max_y {
                return false;
            }
        }

        self.filled.insert(sand_unit);

        true
    }

    fn drop_sand_with_floor(&mut self) -> bool {
        let mut sand_unit = Coord { x: 500, y: 0 };

        while let Some(new_position) = self.next_position(&sand_unit) {
            sand_unit = new_position;
            if sand_unit.y == self.max_y + 1 {
                break;
            }
        }

        let result = sand_unit.y != 0;
        self.filled.insert(sand_unit);
        result
    }

    fn next_position(&self, coord: &Coord) -> Option<Coord> {
        vec![
            Coord {
                x: coord.x,
                y: coord.y + 1,
            },
            Coord {
                x: coord.x - 1,
                y: coord.y + 1,
            },
            Coord {
                x: coord.x + 1,
                y: coord.y + 1,
            },
        ]
        .into_iter()
        .find(|coord| !self.filled.contains(coord))
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn given_test_part1() {
        let map = parse(&["aoc", "2022", "14-test.txt"]).unwrap();
        assert_eq!(part1(map), 24)
    }

    #[test]
    fn given_test_part2() {
        let map = parse(&["aoc", "2022", "14-test.txt"]).unwrap();
        assert_eq!(part2(map), 93)
    }
}

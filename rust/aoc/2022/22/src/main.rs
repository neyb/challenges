use std::collections::HashMap;

use crate::part2::Cube;
use anyhow::{anyhow, bail, Result};

mod part1;
mod part2;

fn main() {
    let (map, path) = parse(&["aoc", "2022", "22.txt"]).unwrap();
    println!(
        "part1: {}",
        solve(&map, &path, |coord, direction| {
            use part1::Map;
            map.coord_at(coord, direction)
        })
    );
    println!("part2: {}", {
        let cube = Cube::try_from(&map).unwrap();
        solve(&map, &path, |coord, direction| {
            cube.coord_at(coord, direction)
        })
    });
}

fn parse(path: &[&str]) -> Result<(Map, Path)> {
    use challenges_common::MyIterTools;

    let input = challenges_common::get_input_lines(path)
        .split(|line| line.is_empty())
        .collect::<Vec<_>>();

    let map = input.get(0).ok_or_else(|| anyhow!("no map in input"))?;

    let path = input
        .get(1)
        .ok_or_else(|| anyhow!("no path in input"))?
        .get(0)
        .ok_or_else(|| anyhow!("empty path"))?;

    Ok((map.try_into()?, path.parse()?))
}

fn solve(map: &Map, path: &Path, coord_at: impl Fn(&Coord, &Direction) -> Coord) -> u32 {
    let state = path.steps.iter().fold(
        Position {
            coord: map.first_node(),
            direction: Direction::Right,
        },
        |mut state, step| {
            state.apply(step, map, |coord, direction| coord_at(coord, direction));
            state
        },
    );

    state.password()
}

struct Map {
    nodes: HashMap<Coord, Node>,
    faces_size: CoordUnit,
}

impl Map {
    fn first_node(&self) -> Coord {
        (0..)
            .map(|x| Coord { x, y: 0 })
            .find(|coord| self.nodes.get(coord) == Some(&Node::Open))
            .unwrap()
    }

    fn get(&self, coord: &Coord) -> Option<&Node> {
        self.nodes.get(coord)
    }

    fn move_until_wall_by(
        &self,
        start: &Coord,
        direction: &Direction,
        nb_step: CoordUnit,
        coord_at: impl Fn(&Coord, &Direction) -> Coord,
    ) -> Coord {
        let mut result = start.clone();
        for _ in 0..nb_step {
            let coord = coord_at(&result, direction);
            match self.get(&coord) {
                Some(Node::Open) => result = coord,
                _ => break,
            }
        }

        result
    }
}

type CoordUnit = i32;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Coord {
    x: CoordUnit,
    y: CoordUnit,
}

impl Coord {
    fn new(x: CoordUnit, y: CoordUnit) -> Self {
        Self { x, y }
    }
    fn at(&self, direction: &Direction) -> Self {
        use Direction::*;

        match direction {
            Up => Coord {
                x: self.x,
                y: self.y - 1,
            },
            Left => Coord {
                x: self.x - 1,
                y: self.y,
            },
            Right => Coord {
                x: self.x + 1,
                y: self.y,
            },
            Down => Coord {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

#[derive(PartialEq)]
enum Node {
    Wall,
    Open,
}

#[derive(PartialEq, Debug)]
struct Path {
    steps: Vec<Step>,
}

#[derive(PartialEq, Debug)]
enum Step {
    GoStraight(CoordUnit),
    Turn(Side),
}

#[derive(PartialEq, Debug)]
enum Side {
    Left,
    Right,
}

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn opposite(&self) -> Self {
        use Direction::*;

        match self {
            Up => Down,
            Left => Right,
            Right => Left,
            Down => Up,
        }
    }

    fn turn(&self, side: &Side) -> Self {
        use Direction::*;

        match (self, side) {
            (Right, Side::Left) | (Left, Side::Right) => Up,
            (Right, Side::Right) | (Left, Side::Left) => Down,
            (Down, Side::Left) | (Up, Side::Right) => Right,
            (Up, Side::Left) | (Down, Side::Right) => Left,
        }
    }
}

struct Position {
    coord: Coord,
    direction: Direction,
}

impl Position {
    fn apply(&mut self, step: &Step, map: &Map, coord_at: impl Fn(&Coord, &Direction) -> Coord) {
        match step {
            Step::GoStraight(nb_steps) => {
                self.coord =
                    map.move_until_wall_by(&self.coord, &self.direction, *nb_steps, coord_at)
            }
            Step::Turn(side) => self.direction = self.direction.turn(side),
        }
    }

    fn password(&self) -> u32 {
        (self.coord.y as u32 + 1) * 1000
            + (self.coord.x as u32 + 1) * 4
            + match self.direction {
                Direction::Up => 3,
                Direction::Left => 2,
                Direction::Right => 0,
                Direction::Down => 1,
            }
    }
}

impl TryFrom<&Vec<String>> for Map {
    type Error = anyhow::Error;

    fn try_from(value: &Vec<String>) -> Result<Self> {
        let mut nodes = HashMap::new();
        for (y, line) in value.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let node = match char {
                    ' ' => None,
                    '.' => Some(Node::Open),
                    '#' => Some(Node::Wall),
                    _ => bail!("unexpected char: {}", char),
                };

                if let Some(node) = node {
                    nodes.insert(
                        Coord {
                            x: x as CoordUnit,
                            y: y as CoordUnit,
                        },
                        node,
                    );
                }
            }
        }

        // the following code is wrong but should be able to get the faces_size is given data...
        let faces_size = nodes
            .keys()
            .filter(|coord| coord.y == 0)
            .map(|coord| coord.x)
            .min()
            .ok_or_else(|| anyhow!("could not "))?;

        Ok(Self { nodes, faces_size })
    }
}

impl std::str::FromStr for Path {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let regex = lazy_regex::regex!(r"\d+|R|L");
        let steps = regex
            .captures_iter(s)
            .map(|captures| {
                let step = captures.get(0).unwrap().as_str();
                step.parse()
            })
            .collect::<Result<_>>()?;

        Ok(Self { steps })
    }
}

impl std::str::FromStr for Step {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use Step::*;
        let step = match s {
            "L" => Turn(Side::Left),
            "R" => Turn(Side::Right),
            _ => GoStraight(s.parse::<CoordUnit>()?),
        };
        Ok(step)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn should_parse_path() {
        use crate::Side::*;
        use crate::Step::*;

        let path: Path = "10R5L5R10L4R5L5".parse().unwrap();

        assert_eq!(
            path.steps,
            vec![
                GoStraight(10),
                Turn(Right),
                GoStraight(5),
                Turn(Left),
                GoStraight(5),
                Turn(Right),
                GoStraight(10),
                Turn(Left),
                GoStraight(4),
                Turn(Right),
                GoStraight(5),
                Turn(Left),
                GoStraight(5),
            ]
        );
    }

    #[test]
    fn should_parse_given_input() {
        let (map, path) = parse(&["aoc", "2022", "22-test.txt"]).unwrap();

        assert_eq!(map.nodes.len(), 96);
        assert_eq!(path, "10R5L5R10L4R5L5".parse().unwrap())
    }
}

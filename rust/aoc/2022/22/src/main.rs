use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use part2::Cube;

mod part1;
mod part2;

fn main() {
    let (map, path) = parse(&["aoc", "2022", "22.txt"]).unwrap();
    println!(
        "part1: {}",
        solve(&map, &path, |position| { part1::jump(&map, position) })
    );
    println!("part2: {}", {
        let cube = Cube::try_from(&map).unwrap();
        solve(&map, &path, |position| cube.jump(position))
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

fn solve(map: &Map, path: &Path, jump: impl Fn(&Position) -> Position) -> u32 {
    let position = path.steps.iter().fold(
        Position {
            coord: map.first_node(),
            direction: Direction::Right,
        },
        |mut state, step| {
            state.apply(step, map, |position| jump(position));
            state
        },
    );

    position.password()
}

struct Map {
    nodes: HashMap<Coord, Node>,
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

    fn move_front_until_wall_by(
        &self,
        position: &Position,
        nb_step: CoordUnit,
        jump: impl Fn(&Position) -> Position,
    ) -> Position {
        let mut resulting_position = position.clone();
        for _ in 0..nb_step {
            let position = {
                let new_potential_coord =
                    resulting_position.coord.at(&resulting_position.direction);
                match self.get(&new_potential_coord) {
                    Some(_) => Position {
                        coord: new_potential_coord,
                        ..resulting_position
                    },
                    None => jump(&resulting_position),
                }
            };

            match self.get(&position.coord) {
                Some(Node::Open) => resulting_position = position,
                _ => break,
            }
        }

        resulting_position
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

#[derive(Clone, Eq, PartialEq, Debug)]
struct Position {
    coord: Coord,
    direction: Direction,
}

impl Position {
    fn apply(&mut self, step: &Step, map: &Map, jump: impl Fn(&Position) -> Position) {
        match step {
            Step::GoStraight(nb_steps) => {
                *self = map.move_front_until_wall_by(self, *nb_steps, jump)
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

        Ok(Self { nodes })
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

    #[test]
    fn part1_given_test() {
        let (map, path) = parse(&["aoc", "2022", "22-test.txt"]).unwrap();
        assert_eq!(
            solve(&map, &path, |position| { part1::jump(&map, position) }),
            6032
        );
    }

    #[test]
    fn part1() {
        let (map, path) = parse(&["aoc", "2022", "22.txt"]).unwrap();
        assert_eq!(
            solve(&map, &path, |position| { part1::jump(&map, position) }),
            181128
        );
    }

    #[test]
    fn part2_given_test() {
        let (map, path) = parse(&["aoc", "2022", "22-test.txt"]).unwrap();
        let cube = Cube::try_from(&map).unwrap();
        assert_eq!(solve(&map, &path, |position| cube.jump(position)), 5031);
    }
}

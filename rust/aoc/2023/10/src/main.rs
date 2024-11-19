use anyhow::{Context, Result};
use challenges_common::graph::{grid, Coord};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "10.txt"]);
    println!("part 1: {}", run(&content).unwrap());
    println!("part 2: {}", run_part2(&content).unwrap());
}

type Len = usize;

fn run(content: &str) -> Result<Len> {
    let map: Map = content.parse()?;
    map.find_animal_loop()
        .map(|path| path.element.len() / 2)
        .context("Cannot find longest loop")
}

fn run_part2(content: &str) -> Result<usize> {
    let map: Map = content.parse()?;
    let animal_loop = map.find_animal_loop().context("Cannot find longest loop")?;
    Ok(map.count_inside_loop(&animal_loop))
}

#[derive(Debug)]
struct Map {
    grid: grid::Grid<Node>,
    animal_coord: Coord,
}

impl Map {
    fn find_animal_loop(&self) -> Option<Path> {
        let mut current_coord = self.animal_coord;
        let mut current_direction = Direction::Right;
        let mut path = Path { element: vec![] };
        loop {
            let next_coord = coord_at(&current_coord, &current_direction);
            let next_node = self.grid.get(&next_coord)?;

            if matches!(next_node, Node::Animal) {
                path.element.push(PathElement {
                    pipe: Pipe(current_direction, Direction::Right),
                    coord: next_coord,
                });
                return Some(path);
            }

            let exit_direction = next_node.follow_pipe(&current_direction.opposite())?;
            path.element.push(PathElement {
                pipe: Pipe(current_direction, exit_direction.opposite()),
                coord: next_coord,
            });

            current_direction = exit_direction;
            current_coord = next_coord;
        }
    }

    fn count_inside_loop(&self, animal_loop: &Path) -> usize {
        use ExploreState::*;

        let mut inside_count = 0;

        let pipe_by_coord: HashMap<Coord, Pipe> = animal_loop
            .element
            .iter()
            .map(|elt| (elt.coord, elt.pipe))
            .collect();
        for y in 0..self.grid.height() {
            let mut explore_state = NotOnPipe { inside: false };
            for x in 0..self.grid.width() {
                let coord = Coord { x, y };

                match pipe_by_coord.get(&coord) {
                    None => {
                        if matches!(explore_state, NotOnPipe { inside: true }) {
                            inside_count += 1;
                        }
                    }
                    Some(pipe) if pipe.is_vertical() => {
                        explore_state = match explore_state {
                            NotOnPipe { inside } => NotOnPipe { inside: !inside },
                            es @ OnPipe { .. } => es,
                        };
                    }
                    Some(pipe) if pipe.is_horizontal() => {}
                    // node is an angle, so it has only 1 vert dir
                    Some(pipe) => {
                        use Direction::*;

                        let vert_direction = match pipe {
                            Pipe(Up, _) | Pipe(_, Up) => Up,
                            Pipe(Down, _) | Pipe(_, Down) => Down,
                            _ => panic!("pipe seems horizontal..."),
                        };

                        explore_state = match explore_state {
                            NotOnPipe { inside } => OnPipe {
                                from: vert_direction,
                                prev_inside: inside,
                            },
                            OnPipe { from, prev_inside } => NotOnPipe {
                                inside: if from == vert_direction {
                                    prev_inside
                                } else {
                                    !prev_inside
                                },
                            },
                        }
                    }
                }
            }
        }
        inside_count
    }
}

#[derive(PartialEq)]
enum ExploreState {
    NotOnPipe { inside: bool },
    OnPipe { from: Direction, prev_inside: bool },
}

fn coord_at(coord: &Coord, direction: &Direction) -> Coord {
    match direction {
        Direction::Up => Coord {
            x: coord.x,
            y: coord.y - 1,
        },
        Direction::Down => Coord {
            x: coord.x,
            y: coord.y + 1,
        },
        Direction::Left => Coord {
            x: coord.x - 1,
            y: coord.y,
        },
        Direction::Right => Coord {
            x: coord.x + 1,
            y: coord.y,
        },
    }
}

impl FromStr for Map {
    type Err = CannotParseMap;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: grid::Grid<Node> = s.parse()?;
        let animal_coord = grid
            .coords()
            .find(|coord| matches!(grid.get(coord), Some(Node::Animal)))
            .ok_or(CannotParseMap::NoAnimalFound)?;
        Ok(Self { grid, animal_coord })
    }
}

#[derive(thiserror::Error, Debug)]
enum CannotParseMap {
    #[error("Cannot parse map: {0}")]
    CannotParseGrid(
        #[from]
        #[source]
        grid::CannotParseGrid,
    ),
    #[error("No animal found")]
    NoAnimalFound,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Node {
    Empty,
    Animal,
    Pipe(Pipe),
}

impl Node {
    fn follow_pipe(&self, from: &Direction) -> Option<Direction> {
        match self {
            Node::Pipe(Pipe(d1, d2)) if d1 == from => Some(*d2),
            Node::Pipe(Pipe(d1, d2)) if d2 == from => Some(*d1),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pipe(Direction, Direction);

impl Pipe {
    fn is_vertical(&self) -> bool {
        matches!(
            self,
            Pipe(Direction::Up, Direction::Down) | Pipe(Direction::Down, Direction::Up)
        )
    }

    fn is_horizontal(&self) -> bool {
        matches!(
            self,
            Pipe(Direction::Left, Direction::Right) | Pipe(Direction::Right, Direction::Left)
        )
    }
}

impl TryFrom<char> for Node {
    type Error = grid::CannotParseElementFromChar;

    fn try_from(value: char) -> Result<Self, grid::CannotParseElementFromChar> {
        Ok(match value {
            '.' => Node::Empty,
            'S' => Node::Animal,
            '|' => Node::Pipe(Pipe(Direction::Up, Direction::Down)),
            '-' => Node::Pipe(Pipe(Direction::Left, Direction::Right)),
            'L' => Node::Pipe(Pipe(Direction::Up, Direction::Right)),
            'J' => Node::Pipe(Pipe(Direction::Left, Direction::Up)),
            '7' => Node::Pipe(Pipe(Direction::Left, Direction::Down)),
            'F' => Node::Pipe(Pipe(Direction::Right, Direction::Down)),
            _ => return Err(value.into()),
        })
    }
}

#[derive(Debug)]
struct Path {
    element: Vec<PathElement>,
}

#[derive(Debug)]
struct PathElement {
    pipe: Pipe,
    coord: Coord,
}

#[cfg(test)]
mod tests {

    mod part1 {
        use crate::*;

        #[test]
        fn given_test_1() {
            let content = challenges_common::get_input_content(&["aoc", "2023", "10-test-1.txt"]);
            assert_eq!(run(&content).unwrap(), 4);
        }

        #[test]
        fn given_test_2() {
            let content = challenges_common::get_input_content(&["aoc", "2023", "10-test-2.txt"]);
            assert_eq!(run(&content).unwrap(), 8);
        }
    }

    mod part2 {
        use crate::*;

        #[test]
        fn given_test_1() {
            let content = challenges_common::get_input_content(&["aoc", "2023", "10-test-1.txt"]);
            assert_eq!(run_part2(&content).unwrap(), 1);
        }

        #[test]
        fn given_test_2() {
            let content = challenges_common::get_input_content(&["aoc", "2023", "10-test-2.txt"]);
            assert_eq!(run_part2(&content).unwrap(), 1);
        }

        #[test]
        fn given_test_3() {
            let content = challenges_common::get_input_content(&["aoc", "2023", "10-test-3.txt"]);
            assert_eq!(run_part2(&content).unwrap(), 4);
        }

        #[test]
        fn given_test_4() {
            let content = challenges_common::get_input_content(&["aoc", "2023", "10-test-4.txt"]);
            assert_eq!(run_part2(&content).unwrap(), 8);
        }
    }
}

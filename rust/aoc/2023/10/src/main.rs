use anyhow::{Context, Result};
use challenges_common::graph::grid;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "10.txt"]);
    println!("part 1: {}", run(&content).unwrap())
}

type Len = u32;

fn run(content: &str) -> Result<Len> {
    let map: Map = content.parse()?;
    map.find_longest_loop_from_animal()
        .map(|len| len / 2)
        .context("Cannot find longest loop")
}

struct Map {
    grid: grid::Grid<Node>,
    animal_coord: grid::Coord,
}

impl Map {
    fn find_longest_loop_from_animal(&self) -> Option<Len> {
        self.find_longest_loop_from(Direction::Right)
    }

    fn find_longest_loop_from(&self, direction: Direction) -> Option<Len> {
        let mut current_coord = self.animal_coord;
        let mut current_direction = direction;
        let mut len = 0;
        loop {
            let next_coord = coord_at(&current_coord, &current_direction);
            let next_node = self.grid.at(&next_coord)?;

            len += 1;
            if matches!(next_node, Node::Animal) {
                return Some(len);
            }

            current_direction = next_node.follow_pipe(&current_direction.opposite())?;
            current_coord = next_coord;
        }
    }
}

fn coord_at(coord: &grid::Coord, direction: &Direction) -> grid::Coord {
    match direction {
        Direction::Up => grid::Coord {
            x: coord.x,
            y: coord.y - 1,
        },
        Direction::Down => grid::Coord {
            x: coord.x,
            y: coord.y + 1,
        },
        Direction::Left => grid::Coord {
            x: coord.x - 1,
            y: coord.y,
        },
        Direction::Right => grid::Coord {
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
            .find(|coord| matches!(grid.at(coord), Some(Node::Animal)))
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

#[derive(Eq, PartialEq, Copy, Clone)]
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

enum Node {
    Empty,
    Animal,
    Pipe(Direction, Direction),
}

impl Node {
    fn follow_pipe(&self, from: &Direction) -> Option<Direction> {
        match self {
            Node::Pipe(d1, d2) if d1 == from => Some(*d2),
            Node::Pipe(d1, d2) if d2 == from => Some(*d1),
            _ => None,
        }
    }
}

impl TryFrom<char> for Node {
    type Error = grid::CannotParseElementFromChar;

    fn try_from(value: char) -> Result<Self, grid::CannotParseElementFromChar> {
        Ok(match value {
            '.' => Node::Empty,
            'S' => Node::Animal,
            '|' => Node::Pipe(Direction::Up, Direction::Down),
            '-' => Node::Pipe(Direction::Left, Direction::Right),
            'L' => Node::Pipe(Direction::Up, Direction::Right),
            'J' => Node::Pipe(Direction::Left, Direction::Up),
            '7' => Node::Pipe(Direction::Left, Direction::Down),
            'F' => Node::Pipe(Direction::Right, Direction::Down),
            _ => return Err(value.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

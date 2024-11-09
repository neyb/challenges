use anyhow::{Context, Result};
use challenges_common::graph::{grid, Coord};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "10.txt"]);
    println!("part 1: {}", run(&content).unwrap())
}

type Len = usize;

fn run(content: &str) -> Result<Len> {
    let map: Map = content.parse()?;
    map.find_animal_loop()
        .map(|path| (path.nodes.len() + 1) / 2)
        .context("Cannot find longest loop")
}

struct Map {
    grid: grid::Grid<Node>,
    animal_coord: Coord,
}

impl Map {
    fn find_animal_loop(&self) -> Option<Path> {
        let mut current_coord = self.animal_coord;
        let mut current_direction = Direction::Right;
        let mut path = Path { nodes: vec![] };
        loop {
            let next_coord = coord_at(&current_coord, &current_direction);
            let next_node = self.grid.at(&next_coord)?;

            if matches!(next_node, Node::Animal) {
                return Some(path);
            }

            let exit_direction = next_node.follow_pipe(&current_direction.opposite())?;
            path.nodes.push(PathElement {
                node: *next_node,
                coord: current_coord,
                entry_direction: current_direction,
                exit_direction,
            });

            current_direction = exit_direction;
            current_coord = next_coord;
        }
    }

    fn get_groups(&self, animal_loop: &Path) -> Vec<Group> {
        let mut visited = HashSet::from_iter(animal_loop.nodes.iter().map(|elt| elt.coord));

        self.grid
            .coords()
            .filter_map(|coord| self.get_group_starting_at(&coord, &mut visited))
            .collect()
    }

    fn get_group_starting_at(&self, coord: &Coord, visited: &mut HashSet<Coord>) -> Option<Group> {
        if visited.contains(&coord) {
            return None;
        }

        let mut group = Group::new();
        let mut coords_to_explore = vec![*coord];
        while let Some(current_coord) = coords_to_explore.pop() {
            if !(visited.contains(&current_coord)) {
                coords_to_explore
                    .extend(Direction::all().map(|dir| coord_at(&current_coord, &dir)));
                group.push(current_coord);
                visited.insert(current_coord);
            }
        }
        Some(group)
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

struct Group {
    nodes: HashSet<Coord>,
}

impl Group {
    fn new() -> Self {
        Self {
            nodes: HashSet::new(),
        }
    }

    fn push(&mut self, coord: Coord) {
        self.nodes.insert(coord);
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
    fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Copy, Clone)]
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

struct Path {
    nodes: Vec<PathElement>,
}

struct PathElement {
    node: Node,
    coord: grid::Coord,
    entry_direction: Direction,
    exit_direction: Direction,
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

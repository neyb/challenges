use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

fn main() {
    let moves = challenges_common::get_input_lines(&vec!["aoc", "2022", "9.txt"])
        .map(|line| line.parse::<Move>().unwrap())
        .collect_vec();

    println!("part1: {}", part1(&moves));
}

fn part1(moves: &Vec<Move>) -> usize {
    moves
        .iter()
        .fold(Rope::new_at_origin(), |mut rope, m| {
            rope.apply_move(&m);
            rope
        })
        .tail
        .visited
        .len()
}

struct Rope {
    head: Head,
    tail: Tail,
}

impl Rope {
    fn new_at_origin() -> Self {
        Rope {
            head: Coord::new_orig(),
            tail: Tail::new(Coord::new_orig()),
        }
    }

    fn apply_move(&mut self, m: &Move) {
        for _ in 0..m.nb_steps {
            self.head.move_to(&m.direction);
            self.tail.follow(&self.head);
        }
    }
}

type Head = Coord;
struct Tail {
    coord: Coord,
    visited: HashSet<Coord>,
}

impl Tail {
    fn new(coord: Coord) -> Self {
        let visited = HashSet::from([coord.clone()]);
        Self { coord, visited }
    }

    fn follow(&mut self, head: &Head) {
        use Direction::*;

        match (head.x - self.coord.x, head.y - self.coord.y) {
            (2 | -2, 1) => self.coord.move_to(&Up),
            (2 | -2, -1) => self.coord.move_to(&Down),
            (1, 2 | -2) => self.coord.move_to(&Right),
            (-1, 2 | -2) => self.coord.move_to(&Left),
            _ => (),
        };

        match (head.x - self.coord.x, head.y - self.coord.y) {
            (2, 0) => self.coord.move_to(&Right),
            (-2, 0) => self.coord.move_to(&Left),
            (0, 2) => self.coord.move_to(&Up),
            (0, -2) => self.coord.move_to(&Down),
            _ => (),
        };

        self.visited.insert(self.coord.clone());
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Coord {
    x: i16,
    y: i16,
}

impl Coord {
    fn new_orig() -> Self {
        Coord { x: 0, y: 0 }
    }

    fn move_to(&mut self, direction: &Direction) {
        use Direction::*;
        match direction {
            Up => self.y += 1,
            Down => self.y -= 1,
            Left => self.x -= 1,
            Right => self.x += 1,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Move {
    nb_steps: u8,
    direction: Direction,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split(" ").collect_vec()[..] {
            [direction, nb_step] => Ok(Self {
                nb_steps: nb_step.parse()?,
                direction: direction.parse()?,
            }),
            _ => Err(anyhow!("cannot parse move from {}", s)),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "R" => Self::Right,
            "L" => Self::Left,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => Err(anyhow!("cannot parse direction from {}", s))?,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn parsing_simple_move() {
        let parsed_move: Move = "R 4".parse().unwrap();
        assert_eq!(
            parsed_move,
            Move {
                direction: Direction::Right,
                nb_steps: 4
            }
        );
    }

    #[test]
    fn moving_coordinate() {
        let mut coord = Coord { x: 1, y: 1 };
        coord.move_to(&Direction::Up);
        assert_eq!(coord, Coord { x: 1, y: 2 })
    }

    #[test]
    fn tail_following_head_to_right() {
        let mut tail = Tail::new(Coord { x: 0, y: 0 });
        let head = Coord { x: 2, y: 0 };

        tail.follow(&head);

        assert_eq!(tail.coord, Coord { x: 1, y: 0 });
        assert_eq!(tail.visited.len(), 2)
    }

    #[test]
    fn tail_following_diag() {
        let mut tail = Tail::new(Coord { x: 0, y: 0 });
        let head = Coord { x: -2, y: 1 };

        tail.follow(&head);

        assert_eq!(tail.coord, Coord { x: -1, y: 1 })
    }
}

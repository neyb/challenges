use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

fn main() {
    let moves = challenges_common::get_input_lines(&vec!["aoc", "2022", "9.txt"])
        .map(|line| line.parse::<Move>().unwrap())
        .collect_vec();

    println!("part1: {}", part1(&moves));
    println!("part2: {}", part2(&moves));
}

fn part1(moves: &Vec<Move>) -> usize {
    count_visited(moves, 1)
}

fn part2(moves: &Vec<Move>) -> usize {
    count_visited(moves, 9)
}

fn count_visited(moves: &Vec<Move>, tail_position: usize) -> usize {
    moves
        .iter()
        .fold(Rope::new_at_origin(tail_position + 1), |mut rope, m| {
            rope.apply_move(&m);
            rope
        })
        .tail()
        .count_visited()
}

struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    fn new_at_origin(nb_knots: usize) -> Self {
        Rope {
            knots: vec![Knot::new(Coord::new_orig()); nb_knots],
        }
    }

    fn tail(&self) -> &Knot {
        self.knots.last().unwrap()
    }

    fn apply_move(&mut self, m: &Move) {
        for _ in 0..m.nb_steps {
            let mut prev_coord = None;

            for knot in &mut self.knots {
                match prev_coord {
                    None => knot.move_to(&m.direction),
                    Some(prev_coord) => knot.follow(prev_coord),
                }
                prev_coord = Some(&knot.coord);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Knot {
    coord: Coord,
    visited: HashSet<Coord>,
}

impl Knot {
    fn new(coord: Coord) -> Self {
        let mut coord = Self {
            coord,
            visited: HashSet::new(),
        };
        coord.mark_position();
        coord
    }

    fn move_to(&mut self, direction: &Direction) {
        self.coord.move_to(direction);
        self.mark_position();
    }

    fn follow(&mut self, to_follow: &Coord) {
        use Direction::*;

        if to_follow.x.abs_diff(self.coord.x) > 1 || to_follow.y.abs_diff(self.coord.y) > 1 {
            if to_follow.x > self.coord.x {
                self.coord.move_to(&Right)
            }
            if to_follow.x < self.coord.x {
                self.coord.move_to(&Left)
            }
            if to_follow.y > self.coord.y {
                self.coord.move_to(&Up)
            }
            if to_follow.y < self.coord.y {
                self.coord.move_to(&Down)
            }
        }

        self.mark_position()
    }

    fn mark_position(&mut self) {
        self.visited.insert(self.coord.clone());
    }

    fn count_visited(&self) -> usize {
        self.visited.len()
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
    fn follow_close_should_not_move() {
        let mut tail = Knot::new(Coord { x: 0, y: 0 });
        let head = Coord { x: 1, y: 0 };

        tail.follow(&head);

        assert_eq!(tail.coord, Coord { x: 0, y: 0 });
    }

    #[test]
    fn follow_close_diag_should_not_move() {
        let mut tail = Knot::new(Coord { x: 0, y: 0 });
        let head = Coord { x: 1, y: 1 };

        tail.follow(&head);

        assert_eq!(tail.coord, Coord { x: 0, y: 0 });
    }

    #[test]
    fn tail_following_head_to_right() {
        let mut tail = Knot::new(Coord { x: 0, y: 0 });
        let head = Coord { x: 2, y: 0 };

        tail.follow(&head);

        assert_eq!(tail.coord, Coord { x: 1, y: 0 });
        assert_eq!(tail.visited.len(), 2)
    }

    #[test]
    fn tail_following_diag() {
        let mut tail = Knot::new(Coord { x: 0, y: 0 });
        let head = Coord { x: -2, y: 1 };

        tail.follow(&head);

        assert_eq!(tail.coord, Coord { x: -1, y: 1 })
    }

    #[test]
    fn tail_following_real_diag() {
        let mut tail = Knot::new(Coord { x: 0, y: 0 });
        let head = Coord { x: -2, y: -2 };

        tail.follow(&head);

        assert_eq!(tail.coord, Coord { x: -1, y: -1 })
    }

    #[test]
    fn solution_part1() {
        let moves = challenges_common::get_input_lines(&["aoc", "2022", "9.txt"])
            .map(|line| line.parse::<Move>().unwrap())
            .collect_vec();

        assert_eq!(part1(&moves), 6337);
    }

    #[test]
    fn given_test_part2() {
        let moves = challenges_common::get_input_lines(&["aoc", "2022", "9-test.txt"])
            .map(|line| line.parse::<Move>().unwrap())
            .collect_vec();

        assert_eq!(part2(&moves), 1);
    }

    #[test]
    fn given_larger_test_part2_step_by_step() {
        let mut rope = Rope::new_at_origin(10);

        rope.apply_move(&Move::from_str("R 5").unwrap());
        assert_eq!(rope.tail().visited, HashSet::from([Coord { x: 0, y: 0 }]));

        rope.apply_move(&Move::from_str("U 8").unwrap());
        assert_eq!(rope.tail().visited, HashSet::from([Coord { x: 0, y: 0 }]));

        rope.apply_move(&Move::from_str("L 8").unwrap());
        assert_eq!(
            rope.tail().visited,
            HashSet::from([
                Coord { x: 0, y: 0 },
                Coord { x: 1, y: 1 },
                Coord { x: 2, y: 2 },
                Coord { x: 1, y: 3 }
            ])
        );
    }

    #[test]
    fn _given_larger_test_part2() {
        let moves = challenges_common::get_input_lines(&["aoc", "2022", "9-larger-test.txt"])
            .map(|line| line.parse::<Move>().unwrap())
            .collect_vec();

        assert_eq!(part2(&moves), 36);
    }

    #[test]
    fn _solution_part2() {
        let moves = challenges_common::get_input_lines(&vec!["aoc", "2022", "9.txt"])
            .map(|line| line.parse::<Move>().unwrap())
            .collect_vec();

        assert_eq!(part2(&moves), 2455);
    }
}

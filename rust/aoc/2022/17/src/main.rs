use anyhow::{anyhow, bail, Result};

use challenges_common::get_input_content;

mod brute_force;
mod smarter;

fn main() {
    let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17.txt"]);
    println!(
        "part1: {}",
        brute_force::rock_tower_high(&jet_pattern, 2022).unwrap()
    );
    println!(
        "part2: {}",
        smarter::rock_tower_high(&jet_pattern, 1_000_000_000_000).unwrap()
    );
}

// should be generics... but later...
type YType = u64;
type NbFallsType = u64;

#[derive(Clone)]
struct JetPattern {
    pattern: Vec<Direction>,
}

impl JetPattern {
    fn parse_location(location: &[&str]) -> JetPattern {
        let content = get_input_content(location).unwrap();
        JetPattern::parse(&content).unwrap()
    }

    fn parse(input: &str) -> Result<Self> {
        Ok(Self {
            pattern: input
                .chars()
                .map(|c| match c {
                    '<' => Ok(Direction::Left),
                    '>' => Ok(Direction::Right),
                    _ => bail!("{} is not a jet direction", c),
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Down,
}

struct Rock {
    origin: Coord,
    shape: RockShape,
}

impl Rock {
    fn parts(&self) -> Vec<Coord> {
        return self
            .shape
            .parts()
            .iter()
            .map(|part| part.move_by(&self.origin))
            .collect();
    }

    fn can_move(&self, direction: &Direction, state: &State) -> bool {
        !self
            .parts()
            .into_iter()
            .map(|part| part.move_to(direction))
            .any(|part| match part {
                None => true,
                Some(part) => state.contains(&part),
            })
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum RockShape {
    Minus,
    Plus,
    Corner,
    Pipe,
    Square,
}

impl RockShape {
    fn parts(&self) -> &'static [Coord] {
        use RockShape::*;

        match self {
            Minus => {
                const SHAPE: [Coord; 4] = [c(0, 0), c(1, 0), c(2, 0), c(3, 0)];
                &SHAPE
            }
            Plus => {
                const SHAPE: [Coord; 5] = [c(0, 1), c(1, 0), c(1, 1), c(1, 2), c(2, 1)];
                &SHAPE
            }
            Corner => {
                const SHAPE: [Coord; 5] = [c(0, 0), c(1, 0), c(2, 0), c(2, 1), c(2, 2)];
                &SHAPE
            }
            Pipe => {
                const SHAPE: [Coord; 4] = [c(0, 0), c(0, 1), c(0, 2), c(0, 3)];
                &SHAPE
            }
            Square => {
                const SHAPE: [Coord; 4] = [c(0, 0), c(0, 1), c(1, 0), c(1, 1)];
                &SHAPE
            }
        }
    }

    fn next(&self) -> Self {
        use RockShape::*;
        match self {
            Minus => Plus,
            Plus => Corner,
            Corner => Pipe,
            Pipe => Square,
            Square => Minus,
        }
    }
}

const fn c(x: u8, y: YType) -> Coord {
    Coord { x, y }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Coord {
    x: u8,
    y: YType,
}

impl Coord {
    fn move_to(&self, direction: &Direction) -> Option<Coord> {
        let (x, y) = match direction {
            Direction::Left => (self.x.checked_sub(1)?, self.y),
            Direction::Right => (if self.x + 1 >= 7 { None? } else { self.x + 1 }, self.y),
            Direction::Down => (self.x, self.y.checked_sub(1)?),
        };

        Some(Self { x, y })
    }

    fn move_by(&self, coord: &Coord) -> Coord {
        Coord {
            x: self.x + coord.x,
            y: self.y + coord.y,
        }
    }
}

struct JetState {
    pattern: JetPattern,
    next_index: usize,
}

impl JetState {
    fn from(jet_pattern: JetPattern) -> Self {
        Self {
            next_index: 0,
            pattern: jet_pattern,
        }
    }
}

impl Iterator for JetState {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let jet_direction = self.pattern.pattern.get(self.next_index).unwrap();
        self.next_index = (self.next_index + 1) % self.pattern.pattern.len();
        Some(*jet_direction)
    }
}

struct Landscape {
    lines: Vec<[bool; 7]>,
}

impl Landscape {
    fn new() -> Self {
        Self { lines: Vec::new() }
    }

    fn insert(&mut self, coord: &Coord) {
        let line_index = coord.y as usize;
        for _ in self.lines.len()..(line_index + 1) {
            self.lines.push([false; 7])
        }

        let line = self.lines.get_mut(line_index).unwrap();
        line[coord.x as usize] = true;
    }

    fn contains(&self, coord: &Coord) -> bool {
        let line_index = coord.y as usize;

        match self.lines.get(line_index) {
            None => false,
            Some(line) => line[coord.x as usize],
        }
    }
}

struct State {
    falling_rock: Rock,
    landscape: Landscape,
    jet_state: JetState,
    height: YType,
}

impl State {
    fn from(jet_pattern: JetPattern) -> Self {
        Self {
            landscape: Landscape::new(),
            jet_state: JetState::from(jet_pattern),
            falling_rock: Rock {
                origin: Coord { x: 2, y: 3 },
                shape: RockShape::Minus,
            },
            height: 0,
        }
    }

    fn run_falls(&mut self, nb_falls: NbFallsType) -> Result<()> {
        (0..nb_falls).try_fold((), |_, _| self.run_until_next_rock())
    }

    fn run_until_next_rock(&mut self) -> Result<()> {
        while self.run_turn()? {}
        Ok(())
    }

    fn run_turn(&mut self) -> Result<bool> {
        let jet_direction = self
            .jet_state
            .next()
            .ok_or_else(|| anyhow!("no next jet direction"))?;

        self.move_if_possible(&jet_direction);

        let landscape_touched = !self.move_if_possible(&Direction::Down);
        if landscape_touched {
            for part in self.falling_rock.parts() {
                self.height = self.height.max(part.y + 1);
                self.landscape.insert(&part);
            }
            self.falling_rock = Rock {
                origin: self.next_origin(),
                shape: self.falling_rock.shape.next(),
            };

            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn move_if_possible(&mut self, direction: &Direction) -> bool {
        let can_move = self.falling_rock.can_move(direction, self);
        if can_move {
            self.falling_rock.origin = self.falling_rock.origin.move_to(direction).unwrap()
        }
        can_move
    }

    fn next_origin(&self) -> Coord {
        Coord {
            x: 2,
            y: self.height + 3,
        }
    }

    fn contains(&self, part: &Coord) -> bool {
        self.landscape.contains(part)
    }
}

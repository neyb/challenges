use anyhow::{anyhow, Error};
use challenges_common::graph::{CannotParseElementFromChar, Coord, Direction, Grid, Turn};
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "06.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

#[derive(Clone, Hash, Eq, PartialEq)]
struct GuardState {
    position: Coord,
    direction: Direction,
}

impl GuardState {
    fn next(mut self, map: &Map) -> Option<Self> {
        let face_position = self.position.try_at(self.direction)?;
        let next = match map.grid.get(&face_position) {
            Some(position) if position == &Block::Obstruction => {
                self.direction = self.direction.turn(Turn::Right);
                self
            }
            _ => {
                self.position = face_position;
                self
            }
        };
        Some(next).filter(|guard_state| map.grid.get(&guard_state.position).is_some())
    }
}

#[derive(Clone)]
struct Map {
    grid: Grid<Block>,
    init_guard: GuardState,
}

impl Map {
    fn all_guard_positions(&self) -> HashSet<Coord> {
        std::iter::successors(Some(self.init_guard.clone()), |guard| {
            guard.clone().next(self)
        })
        .map(|guard_state| guard_state.position)
        .collect()
    }
}

#[derive(Eq, PartialEq, Clone)]
enum Block {
    Empty,
    Obstruction,
}

#[derive(Eq, PartialEq)]
enum PositionInit {
    Empty,
    Block,
    GuardInit,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid<PositionInit> = s.parse()?;
        let guard_position = grid
            .find(|n| n == &PositionInit::GuardInit)
            .ok_or_else(|| anyhow!("guard not found"))?;

        Ok(Self {
            grid: grid.map(|init| match init {
                PositionInit::Empty => Block::Empty,
                PositionInit::Block => Block::Obstruction,
                PositionInit::GuardInit => Block::Empty,
            }),
            init_guard: GuardState {
                direction: Direction::Up,
                position: guard_position,
            },
        })
    }
}

impl TryFrom<char> for PositionInit {
    type Error = CannotParseElementFromChar;

    fn try_from(value: char) -> anyhow::Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '#' => Self::Block,
            '^' => Self::GuardInit,
            _ => Err(CannotParseElementFromChar::from(value))?,
        })
    }
}

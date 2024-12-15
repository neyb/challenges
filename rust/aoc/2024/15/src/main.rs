use anyhow::{anyhow, Error};
use challenges_common::graph::Direction::{Down, Left, Right, Up};
use challenges_common::graph::{grid, Direction};
use itertools::Itertools;
use std::fmt::Display;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "15.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

struct Moves {
    moves: Vec<Move>,
}

impl Moves {
    fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves.iter()
    }
}

struct Move(Direction);

impl FromStr for Moves {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        use grid::Direction::*;
        let moves = s
            .chars()
            .filter(|c| c != &'\n')
            .map(|c| match c {
                '^' => anyhow::Ok(Move(Up)),
                'v' => anyhow::Ok(Move(Down)),
                '<' => anyhow::Ok(Move(Left)),
                '>' => anyhow::Ok(Move(Right)),
                _ => Err(anyhow!("Cannot parse move: {c}")),
            })
            .try_collect()?;
        anyhow::Ok(Self { moves })
    }
}

use challenges_common::graph::{CannotParseElementFromChar, CannotParseGrid, Coord, Grid};
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "17.txt"]);
    println!("part1: {}", part1::run(&content).unwrap());
    println!("part2: {}", part2::run(&content).unwrap());
}

mod part1;
mod part2;

struct Map {
    grid: Grid<Block>,
}

impl Map {
    fn end_coord(&self) -> Coord {
        Coord {
            x: self.grid.width() - 1,
            y: self.grid.height() - 1,
        }
    }
}

struct Block {
    heat_loss: u8,
}

impl FromStr for Map {
    type Err = CannotParseGrid;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        Ok(Self { grid })
    }
}

impl TryFrom<char> for Block {
    type Error = CannotParseElementFromChar;

    fn try_from(char: char) -> anyhow::Result<Self, Self::Error> {
        Ok(Self {
            heat_loss: char
                .to_digit(10)
                .ok_or_else(|| CannotParseElementFromChar::from(char))?
                as u8,
        })
    }
}

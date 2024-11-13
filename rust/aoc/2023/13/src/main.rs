use challenges_common::graph::{CannotParseElementFromChar, CannotParseGrid, Grid};
use challenges_common::MyIterTools;
use itertools::Itertools;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "13.txt"]);
    println!("part 1: {}", part1::run(&content).unwrap());
    println!("part 2: {}", part2::run(&content).unwrap());
}

mod part1;
mod part2;

struct Patterns {
    patterns: Vec<Pattern>,
}

impl FromStr for Patterns {
    type Err = CannotParseGrid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            patterns: s
                .lines()
                .split(|line| line.is_empty())
                .map(|s| s.join("\n").parse())
                .try_collect()?,
        })
    }
}

struct Pattern {
    grid: Grid<Place>,
}

impl FromStr for Pattern {
    type Err = CannotParseGrid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { grid: s.parse()? })
    }
}

struct Place(bool);

impl TryFrom<char> for Place {
    type Error = CannotParseElementFromChar;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        Ok(match char {
            '.' => Self(false),
            '#' => Self(true),
            _ => Err(Self::Error::from(char))?,
        })
    }
}

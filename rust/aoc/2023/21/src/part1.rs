use anyhow::*;
use challenges_common::graph::{CannotParseElementFromChar, Coord, Grid};
use std::convert::TryFrom;
use std::str::FromStr;

type Res = usize;
pub(crate) fn run(content: &String) -> Result<Res> {
    let map: Map = content.parse()?;
    map.explore(16)
}

struct Map {
    grid: Grid<Position>,
}

impl Map {
    pub(crate) fn explore(&self, count: u8) -> Result<Res> {
        let mut explored = Vec::new();
        let mut current_positions = Vec::new();
    }

    fn start(&self) -> Option<Coord> {
        self.grid
            .nodes()
            .iter()
            .find(|position| position.position_type == PositionType::Start)
            .map(|(coord, _)| coord)
    }
}

struct Position {
    position_type: PositionType,
    explore_range: Option<u8>,
}

enum PositionType {
    Start,
    Empty,
    Rock,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self { grid: s.parse()? })
    }
}

impl TryFrom<char> for Position {
    type Error = CannotParseElementFromChar;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let position_type = match value {
            '.' => PositionType::Empty,
            '#' => PositionType::Rock,
            'X' => PositionType::Start,
            _ => Err(CannotParseElementFromChar::from(value))?,
        };

        Result::Ok(Self {
            position_type,
            explore_range: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "21-test.txt"]);
        assert_eq!(run(&content).unwrap(), 16);
    }
}

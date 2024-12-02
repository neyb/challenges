use crate::{Position, PositionType};
use anyhow::*;
use challenges_common::graph::{CannotParseGrid, Coord, Grid};
use std::str::FromStr;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let mut map: Map = content.parse()?;
    map.explore(64)?;
    Ok(map.count_explored(64))
}

struct Map {
    grid: Grid<Position>,
}

impl Map {
    fn explore(&mut self, times: u8) -> Result<()> {
        let mut current_positions = vec![self.start().ok_or_else(|| anyhow!("no start"))?];

        for i in 0..times {
            let mut next_positions = Vec::new();
            for position in current_positions {
                for next_coord in position.neighbours(false) {
                    if let Some(next_position) = self.grid.get_mut(&next_coord) {
                        if next_position.should_be_explored() {
                            next_position.explore_range = Some(i + 1);
                            next_positions.push(next_coord);
                        }
                    }
                }
            }

            current_positions = next_positions;
        }

        anyhow::Ok(())
    }

    fn start(&self) -> Option<Coord> {
        self.grid
            .find(|position| position.position_type == PositionType::Start)
    }

    fn count_explored(&self, turn: u8) -> usize {
        self.grid
            .nodes()
            .iter()
            .filter(|position| {
                matches!(position.explore_range, Some(range) if range %2 == turn %2 && range <= turn)
            })
            .count()
    }
}

impl FromStr for Map {
    type Err = CannotParseGrid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        std::prelude::rust_2015::Ok(Self { grid: s.parse()? })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_after_6_turns() -> Result<()> {
        let content = challenges_common::get_input_content(&["aoc", "2023", "21-test.txt"]);
        let mut map: Map = content.parse()?;
        map.explore(6)?;
        map.count_explored(6);

        assert_eq!(map.count_explored(6), 16);
        Ok(())
    }
}

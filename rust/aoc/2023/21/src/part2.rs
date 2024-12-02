use crate::PositionType;
use anyhow::*;
use challenges_common::graph;
use challenges_common::graph::{CannotParseGrid, Grid};
use std::collections::HashMap;
use std::str::FromStr;

type Res = usize;
pub(crate) fn run(content: &String) -> Result<Res> {
    let mut map: InfiniteMap = content.parse()?;
    map.explore(26501365)?;
    Ok(map.count_explored(26501365))
}

type Coord = graph::Coord<isize>;
type Position = crate::Position<u32>;

struct InfiniteMap {
    grid: Grid<PositionType, isize>,
    positions: HashMap<Coord, Position>,
}

impl InfiniteMap {
    fn explore(&mut self, times: u32) -> Result<()> {
        let mut current_coords = vec![self.start().ok_or_else(|| anyhow!("no start"))?];

        for i in 0..times {
            let mut next_positions = Vec::new();
            for current_coord in current_coords {
                for next_coord in current_coord.neighbours(false) {
                    let next_position = self.get_mut(next_coord);
                    if next_position.should_be_explored() {
                        next_position.explore_range = Some(i + 1);
                        next_positions.push(next_coord);
                    }
                }
            }

            current_coords = next_positions;
        }

        anyhow::Ok(())
    }

    fn start(&self) -> Option<Coord> {
        self.grid.find(|position| position == &PositionType::Start)
    }

    fn get_mut(&mut self, coord: Coord) -> &mut Position {
        self.positions.entry(coord).or_insert_with(|| {
            let coord_in_grid = Coord {
                x: coord.x.rem_euclid(self.grid.width()),
                y: coord.y.rem_euclid(self.grid.height()),
            };
            let mut position_type = *self
                .grid
                .get(&coord_in_grid)
                .expect(format!("no position at {:?}", coord_in_grid).as_str());
            if position_type == PositionType::Start {
                position_type = PositionType::Empty;
            }
            Position {
                position_type,
                explore_range: None,
            }
        })
    }

    fn count_explored(&self, turn: u32) -> usize {
        let turn_mod = turn % 2;
        self.positions.values()
            .filter(|position| {
                matches!(position.explore_range, Some(range) if range %2 == turn_mod && range <= turn)
            })
            .count()
    }
}

impl FromStr for InfiniteMap {
    type Err = CannotParseGrid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        std::prelude::rust_2015::Ok(Self {
            grid: s.parse()?,
            positions: HashMap::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infinite_map() {
        let mut map: InfiniteMap =
            challenges_common::get_input_content(&["aoc", "2023", "21-test.txt"])
                .parse()
                .unwrap();

        map.explore(100).unwrap();
        assert_eq!(map.count_explored(6), 16);
        assert_eq!(map.count_explored(10), 50);
        assert_eq!(map.count_explored(50), 1594);
        assert_eq!(map.count_explored(100), 6536);
        // assert_eq!(map.count_explored(500), 167004);
        // assert_eq!(map.count_explored(1000), 668697);
        // assert_eq!(map.count_explored(5000), 16733044);
    }
}

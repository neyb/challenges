use crate::PositionType;
use anyhow::*;
use challenges_common::graph;
use challenges_common::graph::{CannotParseGrid, Grid};
use std::collections::HashSet;
use std::str::FromStr;

type Res = u32;
pub(crate) fn run(content: &String, steps: u32) -> Result<Res> {
    let map: InfiniteMap = content.parse()?;

    let start = map.start().ok_or_else(|| anyhow!("no start"))?;
    let mut old_explored = HashSet::new();
    let mut previous_explored = HashSet::new();
    previous_explored.insert(start);

    let mut count = (steps + 1) % 2;

    for i_step in 0..steps {
        let mut discovered = HashSet::new();
        #[cfg(feature = "trace")]
        let original_count = count;

        for coord in &previous_explored {
            for neighbour in coord.neighbours(false) {
                if map.get_type(neighbour) == PositionType::Empty
                    && !old_explored.contains(&neighbour)
                {
                    if (i_step + 1) % 2 == steps % 2 && !discovered.contains(&neighbour) {
                        count += 1;
                    }
                    discovered.insert(neighbour);
                }
            }
        }

        old_explored = previous_explored;
        previous_explored = discovered;

        #[cfg(feature = "trace")]
        if (i_step + 1) % 2 == steps % 2 {
            print!("after step: {}", i_step + 1);

            let incr = count - original_count;
            print!(", incr: {incr}");
            print!(", mult: {}", count as f32 / original_count as f32);
            print!(", incr - step : {}", incr - i_step);
            print!(", incr - 2 step : {}", incr - (2 * i_step));
            println!(", incr - 3 step : {}", incr - (3 * i_step));
        }
    }

    Ok(count)
}

type Coord = graph::Coord<isize>;
struct InfiniteMap {
    grid: Grid<PositionType, isize>,
}

impl InfiniteMap {
    fn start(&self) -> Option<Coord> {
        self.grid.find(|position| position == &PositionType::Start)
    }

    fn get_type(&self, coord: Coord) -> PositionType {
        let coord_in_grid = Coord {
            x: coord.x.rem_euclid(self.grid.width()),
            y: coord.y.rem_euclid(self.grid.height()),
        };
        let position_type = *self
            .grid
            .get(&coord_in_grid)
            .unwrap_or_else(|| panic!("no position at {:?}", coord_in_grid));

        if position_type == PositionType::Start {
            PositionType::Empty
        } else {
            position_type
        }
    }
}

impl FromStr for InfiniteMap {
    type Err = CannotParseGrid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        std::prelude::rust_2015::Ok(Self { grid: s.parse()? })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infinite_map_6() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "21-test.txt"]);
        assert_eq!(run(&content, 6).unwrap(), 16);
    }

    #[test]
    fn test_infinite_map_10() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "21-test.txt"]);
        assert_eq!(run(&content, 10).unwrap(), 50);
    }

    #[test]
    fn test_infinite_map_50() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "21-test.txt"]);
        assert_eq!(run(&content, 50).unwrap(), 1594);
    }

    #[test]
    fn test_infinite_map_100() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "21-test.txt"]);
        assert_eq!(run(&content, 100).unwrap(), 6536);
    }

    #[test]
    fn test_infinite_map_500() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "21-test.txt"]);
        assert_eq!(run(&content, 500).unwrap(), 167004);
    }

    #[test]
    fn test_infinite_map_1000() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "21-test.txt"]);
        assert_eq!(run(&content, 1000).unwrap(), 668697);
    }

    #[test]
    fn test_infinite_map_5000() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "21-test.txt"]);
        assert_eq!(run(&content, 5000).unwrap(), 16733044);
    }
}

use anyhow::{anyhow, Result};
use challenges_common::graph::{
    astar, CannotParseElementFromChar, CannotParseGrid, Coord, Direction, Grid, Step,
};
use itertools::Itertools;
use std::hash::Hash;
use std::iter::once;
use std::str::FromStr;

type Res = u32;
pub(crate) fn run(content: &str) -> Result<Res> {
    let map: Map = content.parse()?;
    let start = map
        .start_path_element()
        .ok_or_else(|| anyhow::anyhow!("No start found"))?;
    let end = map.end_coord();
    let path = astar(
        start,
        |path_element| {
            let forbiden_dirs = path_element.forbidden_directions();

            Direction::all()
                .into_iter()
                .filter_map(|direction| {
                    if forbiden_dirs.contains(&direction) {
                        return None;
                    }
                    let new_coord = path_element.coord.try_at(direction)?;
                    let block = map.grid.get(&new_coord)?;
                    let next_path_element = PathElement {
                        coord: new_coord,
                        previous_directions: once(direction)
                            .chain(path_element.previous_directions.iter().take(2).cloned())
                            .collect(),
                    };
                    Some(Step {
                        to: next_path_element,
                        additional_cost: block.heat_loss as Res,
                    })
                })
                .collect_vec()
        },
        |path_element| path_element.coord == end,
        |path_element| path_element.coord.manhattan_dist_to(&end) as Res,
    );

    let path = path.ok_or_else(|| anyhow!("No path found"))?;
    Ok(path.cost)
}

struct Map {
    grid: Grid<Block>,
}

impl Map {
    fn start_path_element(&self) -> Option<PathElement> {
        let coord = Coord { x: 0, y: 0 };
        Some(PathElement {
            coord,
            previous_directions: Vec::new(),
        })
    }

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

#[derive(Debug, Hash, PartialEq, Eq)]
struct PathElement {
    coord: Coord,
    previous_directions: Vec<Direction>,
}

impl PathElement {
    fn forbidden_directions(&self) -> Vec<Direction> {
        let mut result = Vec::new();
        if let Some(&first) = self.previous_directions.first() {
            result.push(first.opposite());
        }
        if let Some(repeated) = self.too_much_repeated_direction() {
            result.push(repeated);
        }
        result
    }

    fn too_much_repeated_direction(&self) -> Option<Direction> {
        if self.previous_directions.len() >= 3 {
            self.previous_directions
                .iter()
                .take(3)
                .all_equal_value()
                .ok()
                .copied()
        } else {
            None
        }
    }
}

impl FromStr for Map {
    type Err = CannotParseGrid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = Grid::from_str(s)?;
        Ok(Self { grid })
    }
}

impl TryFrom<char> for Block {
    type Error = CannotParseElementFromChar;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        Ok(Self {
            heat_loss: char
                .to_digit(10)
                .ok_or_else(|| CannotParseElementFromChar::from(char))?
                as u8,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "17-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 102);
    }
}

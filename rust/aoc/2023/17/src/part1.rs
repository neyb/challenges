use crate::Map;
use anyhow::{anyhow, Result};
use challenges_common::graph::{astar, Coord, Direction, Step};
use itertools::Itertools;
use std::hash::Hash;

type Res = u32;
pub(crate) fn run(content: &str) -> Result<Res> {
    let map: Map = content.parse()?;
    let path = astar(
        PathElement {
            coord: Coord { x: 0, y: 0 },
            direction: Direction::Right,
            direction_repeat_count: 1,
        },
        |path_element| path_element.nexts(&map),
        |path_element| path_element.coord == map.end_coord(),
        |path_element| path_element.coord.manhattan_dist_to(&map.end_coord()) as Res,
    );

    let path = path.ok_or_else(|| anyhow!("No path found"))?;
    Ok(path.cost)
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct PathElement {
    coord: Coord,
    direction: Direction,
    direction_repeat_count: u8,
}

impl PathElement {
    fn nexts(&self, map: &Map) -> Vec<Step<Self, Res>> {
        let forbidden_dirs = self.forbidden_directions();

        Direction::all()
            .into_iter()
            .filter_map(|direction| {
                if forbidden_dirs.contains(&direction) {
                    return None;
                }
                let new_coord = self.coord.try_at(direction)?;
                let block = map.grid.get(&new_coord)?;
                let next_path_element = PathElement {
                    coord: new_coord,
                    direction,
                    direction_repeat_count: if direction == self.direction {
                        self.direction_repeat_count + 1
                    } else {
                        1
                    },
                };
                Some(Step {
                    to: next_path_element,
                    additional_cost: block.heat_loss as Res,
                })
            })
            .collect_vec()
    }

    fn forbidden_directions(&self) -> Vec<Direction> {
        let mut result = Vec::new();
        result.push(self.direction.opposite());
        if let Some(repeated) = self.too_much_repeated_direction() {
            result.push(repeated);
        }
        result
    }

    fn too_much_repeated_direction(&self) -> Option<Direction> {
        if self.direction_repeat_count >= 3 {
            Some(self.direction)
        } else {
            None
        }
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

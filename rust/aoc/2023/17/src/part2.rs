use crate::Map;
use anyhow::anyhow;
use challenges_common::graph::{astar, Coord, Direction, Step, Turn};
use itertools::Itertools;

type Res = usize;
pub(crate) fn run(content: &str) -> anyhow::Result<Res> {
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

#[derive(Hash, PartialEq, Eq)]
struct PathElement {
    coord: Coord,
    direction: Direction,
    direction_repeat_count: u8,
}

impl PathElement {
    fn nexts(&self, map: &Map) -> Vec<Step<Self, Res>> {
        let mut result = Vec::new();

        if self.too_much_repeated_direction().is_none() {
            let last = self.direction;
            if let Some(next_coord) = self.coord.try_at(last) {
                if let Some(block) = map.grid.get(&next_coord) {
                    result.push(Step {
                        to: PathElement {
                            coord: next_coord,
                            direction: last,
                            direction_repeat_count: self.direction_repeat_count + 1,
                        },
                        additional_cost: block.heat_loss as Res,
                    });
                }
            }
        }

        for turn in Turn::all() {
            let next_direction = self.direction.turn(turn);

            if let Some(next_coord) = self.coord.try_at_dist(next_direction, 4_usize) {
                if let Some(heat) = map.heat_loss_within(&self.coord, next_direction, 4) {
                    result.push(Step {
                        to: PathElement {
                            coord: next_coord,
                            direction: next_direction,
                            direction_repeat_count: 4,
                        },
                        additional_cost: heat,
                    });
                }
            }
        }

        result
    }

    fn too_much_repeated_direction(&self) -> Option<Direction> {
        if self.direction_repeat_count >= 10 {
            Some(self.direction)
        } else {
            None
        }
    }
}

trait MapPart2 {
    fn heat_loss_within(&self, coord: &Coord, direction: Direction, n: u8) -> Option<Res>;
}

impl MapPart2 for Map {
    fn heat_loss_within(&self, coord: &Coord, direction: Direction, n: u8) -> Option<Res> {
        let result = (1..=n)
            .map(|d| coord.try_at_dist(direction, d))
            .map(|coord| coord.and_then(|coord| self.grid.get(&coord)))
            .collect_vec();
        if result.iter().all(|block| block.is_some()) {
            Some(
                result
                    .into_iter()
                    .map(|block| block.unwrap().heat_loss as Res)
                    .sum(),
            )
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
        assert_eq!(super::run(&content).unwrap(), 94);
    }
    #[test]
    fn given_test_2() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "17-test2.txt"]);
        assert_eq!(super::run(&content).unwrap(), 71);
    }
}

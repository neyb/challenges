use crate::{Direction, Map};
use anyhow::*;
use challenges_common::graph::Coord;
use itertools::Itertools;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let map: Map = content.parse()?;
    Ok(map.count_mas())
}

trait Part2Map {
    fn count_mas(&self) -> Res;
}

impl Part2Map for Map {
    fn count_mas(&self) -> Res {
        self.grid
            .coords()
            .flat_map(|coord| {
                [Direction::U, Direction::D, Direction::L, Direction::R]
                    .into_iter()
                    .map(|dir| (coord, dir))
                    .collect_vec()
            })
            .filter_map(|(coord, dir)| {
                let chars = dir
                    .cross_coord(&coord)?
                    .into_iter()
                    .map(|coord| self.grid.get(&coord))
                    .collect::<Option<Vec<&char>>>()?;
                Some(chars.iter().copied().join(""))
            })
            .filter(|s| s == "MMASS")
            .count()
    }
}

trait Part2Direction {
    fn cross_coord(&self, coord: &Coord) -> Option<Vec<Coord>>;
}

impl Part2Direction for Direction {
    fn cross_coord(&self, coord: &Coord) -> Option<Vec<Coord>> {
        let coords = match self {
            Direction::U => vec![
                Direction::DL.move_from_coord_dist(coord, 1)?,
                Direction::DR.move_from_coord_dist(coord, 1)?,
                *coord,
                Direction::UR.move_from_coord_dist(coord, 1)?,
                Direction::UL.move_from_coord_dist(coord, 1)?,
            ],
            Direction::D => vec![
                Direction::UL.move_from_coord_dist(coord, 1)?,
                Direction::UR.move_from_coord_dist(coord, 1)?,
                *coord,
                Direction::DR.move_from_coord_dist(coord, 1)?,
                Direction::DL.move_from_coord_dist(coord, 1)?,
            ],
            Direction::L => vec![
                Direction::UR.move_from_coord_dist(coord, 1)?,
                Direction::DR.move_from_coord_dist(coord, 1)?,
                *coord,
                Direction::UL.move_from_coord_dist(coord, 1)?,
                Direction::DL.move_from_coord_dist(coord, 1)?,
            ],
            Direction::R => vec![
                Direction::UL.move_from_coord_dist(coord, 1)?,
                Direction::DL.move_from_coord_dist(coord, 1)?,
                *coord,
                Direction::UR.move_from_coord_dist(coord, 1)?,
                Direction::DR.move_from_coord_dist(coord, 1)?,
            ],
            Direction::UL => vec![
                Direction::D.move_from_coord_dist(coord, 1)?,
                Direction::R.move_from_coord_dist(coord, 1)?,
                *coord,
                Direction::U.move_from_coord_dist(coord, 1)?,
                Direction::L.move_from_coord_dist(coord, 1)?,
            ],
            Direction::UR => vec![
                Direction::D.move_from_coord_dist(coord, 1)?,
                Direction::L.move_from_coord_dist(coord, 1)?,
                *coord,
                Direction::U.move_from_coord_dist(coord, 1)?,
                Direction::R.move_from_coord_dist(coord, 1)?,
            ],
            Direction::DL => vec![
                Direction::U.move_from_coord_dist(coord, 1)?,
                Direction::R.move_from_coord_dist(coord, 1)?,
                *coord,
                Direction::D.move_from_coord_dist(coord, 1)?,
                Direction::L.move_from_coord_dist(coord, 1)?,
            ],
            Direction::DR => vec![
                Direction::U.move_from_coord_dist(coord, 1)?,
                Direction::L.move_from_coord_dist(coord, 1)?,
                *coord,
                Direction::D.move_from_coord_dist(coord, 1)?,
                Direction::R.move_from_coord_dist(coord, 1)?,
            ],
        };

        Some(coords)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "04-test.txt"]);
        assert_eq!(run(&content).unwrap(), 9);
    }
}

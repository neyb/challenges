use crate::{Coord, Map};
use itertools::Itertools;

pub fn antinodes_for_pair(coord_1: &Coord, coord_2: &Coord, map: &Map) -> Vec<Coord> {
    let coord1_based = Coord {
        x: coord_1.x - (coord_2.x - coord_1.x),
        y: coord_1.y - (coord_2.y - coord_1.y),
    };
    let coord2_based = Coord {
        x: coord_2.x - (coord_1.x - coord_2.x),
        y: coord_2.y - (coord_1.y - coord_2.y),
    };
    [coord1_based, coord2_based]
        .into_iter()
        .filter(|coord| map.grid.is_coord_inside(coord))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::antinodes_for_pair;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "08-test.txt"]);
        assert_eq!(crate::run(&content, antinodes_for_pair).unwrap(), 14);
    }
}

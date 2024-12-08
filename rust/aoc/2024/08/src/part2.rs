use crate::{Coord, Map};
use std::iter;

pub fn antinodes_for_pair(coord_1: &Coord, coord_2: &Coord, map: &Map) -> Vec<Coord> {
    return antinodes_from(coord_1, coord_2, map)
        .chain(antinodes_from(coord_2, coord_1, map))
        .collect();

    fn antinodes_from<'c1, 'c2, 'm>(
        from: &'c1 Coord,
        opposing: &'c2 Coord,
        map: &'m Map,
    ) -> impl Iterator<Item = Coord> + use<'c1, 'c2, 'm> {
        iter::successors(
            Some(*from).take_if(|coord| map.grid.is_coord_inside(coord)),
            |coord| {
                Some(Coord {
                    x: coord.x - (opposing.x - from.x),
                    y: coord.y - (opposing.y - from.y),
                })
                .take_if(|coord| map.grid.is_coord_inside(coord))
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::antinodes_for_pair;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "08-test.txt"]);
        assert_eq!(crate::run(&content, antinodes_for_pair).unwrap(), 34);
    }
}

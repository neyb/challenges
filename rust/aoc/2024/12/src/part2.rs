use crate::{Coord, Price};
use challenges_common::graph::Direction;
use itertools::Itertools;

pub trait Region {
    fn sides_count(&self) -> Option<usize>;
    fn price(&self) -> Price;
}

impl Region for crate::Region {
    fn sides_count(&self) -> Option<usize> {
        let (Coord { x: min_x, y: min_y }, Coord { x: max_x, y: max_y }) = self.square()?;

        Some(
            Direction::all()
                .into_iter()
                .map(|scan_dir| {
                    let (primary_iter, secondary_iter, get_coord) = match scan_dir {
                        Direction::Up | Direction::Down => (
                            min_y..=max_y,
                            min_x..=max_x,
                            &(|primary, secondary| Coord {
                                x: secondary,
                                y: primary,
                            }) as &dyn Fn(i16, i16) -> Coord,
                        ),
                        Direction::Left | Direction::Right => (
                            min_x..=max_x,
                            min_y..=max_y,
                            &(|primary, secondary| Coord {
                                x: primary,
                                y: secondary,
                            }) as &dyn Fn(i16, i16) -> Coord,
                        ),
                    };

                    primary_iter
                        .map(|primary| {
                            secondary_iter
                                .clone()
                                .map(|secondary| get_coord(primary, secondary))
                                .chunk_by(|coord| {
                                    self.cells.contains(coord)
                                        && !self.cells.contains(&coord.at(scan_dir))
                                })
                                .into_iter()
                                .map(|(has_border, _group)| has_border)
                                .filter(|&is_inside| is_inside)
                                .count()
                        })
                        .sum::<usize>()
                })
                .sum::<usize>(),
        )
    }

    fn price(&self) -> Price {
        self.area() * self.sides_count().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run;

    #[test]
    fn given_test_1() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test1.txt"]);
        assert_eq!(run(&content, Region::price).unwrap(), 80);
    }

    #[test]
    fn given_test_2() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test2.txt"]);
        assert_eq!(run(&content, Region::price).unwrap(), 436);
    }

    #[test]
    fn given_test_3() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test3.txt"]);
        assert_eq!(run(&content, Region::price).unwrap(), 1206);
    }

    #[test]
    fn test_eshape() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test-eshape.txt"]);
        assert_eq!(run(&content, Region::price).unwrap(), 236);
    }

    #[test]
    fn test_slash() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test-slash.txt"]);
        assert_eq!(run(&content, Region::price).unwrap(), 368);
    }
}

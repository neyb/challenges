use crate::{Coord, Direction};

pub(crate) trait Map {
    fn coord_at(&self, coord: &Coord, direction: &Direction) -> Coord;
}

impl Map for crate::Map {
    fn coord_at(&self, coord: &Coord, direction: &Direction) -> Coord {
        let primary_target = coord.at(direction);

        return match primary_target {
            Some(primary_target) if self.get(&primary_target).is_some() => primary_target,
            _ => {
                let direction1 = &direction.opposite();
                let mut result = coord.clone();

                loop {
                    match result.at(direction1) {
                        Some(coord) if self.get(&coord).is_some() => result = coord,
                        _ => break result,
                    }
                }
            }
        };
    }
}

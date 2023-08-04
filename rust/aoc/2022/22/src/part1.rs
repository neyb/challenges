use crate::{Coord, Direction};

pub(crate) trait Map {
    fn coord_at(&self, coord: &Coord, direction: &Direction) -> Coord;
}

impl Map for crate::Map {
    fn coord_at(&self, coord: &Coord, direction: &Direction) -> Coord {
        let primary_target = coord.at(direction);

        return if self.get(&primary_target).is_some() {
            primary_target
        } else {
            let direction1 = &direction.opposite();
            let mut result = coord.clone();

            loop {
                let coord = result.at(direction1);
                if self.get(&coord).is_some() {
                    result = coord
                } else {
                    break result;
                }
            }
        };
    }
}

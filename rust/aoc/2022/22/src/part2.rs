use crate::{Coord, Direction};

pub(crate) trait Map {
    fn coord_at(&self, coord: &Coord, direction: &Direction) -> Coord;
}

impl Map for crate::Map {
    fn coord_at(&self, coord: &Coord, direction: &Direction) -> Coord {
        use Direction::*;

        let primary_target = coord.at(direction);

        todo!()
    }
}

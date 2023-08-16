use crate::{Coord, Position};

pub(crate) trait Map {
    fn jump(&self, position: &Position) -> Coord;
}

impl Map for crate::Map {
    fn jump(&self, position: &Position) -> Coord {
        let opposite = &position.direction.opposite();
        let mut result = position.coord.clone();

        loop {
            let coord = result.at(opposite);
            if self.get(&coord).is_some() {
                result = coord
            } else {
                break result;
            }
        }
    }
}

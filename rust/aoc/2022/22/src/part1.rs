use crate::{Map, Position};

pub(super) fn jump(map: &Map, position: &Position) -> Position {
    let opposite = &position.direction.opposite();
    let mut result = position.coord.clone();

    let coord = loop {
        let coord = result.at(opposite);
        if map.get(&coord).is_some() {
            result = coord
        } else {
            break result;
        }
    };

    Position {
        coord,
        direction: position.direction,
    }
}

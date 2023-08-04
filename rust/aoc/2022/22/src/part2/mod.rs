use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Mul;

use space3d::*;

use crate::*;

mod space3d;

pub(crate) trait MapPart2 {
    fn coord_at(&self, coord: &Coord, direction: &Direction) -> Coord;
}

impl MapPart2 for Map {
    fn coord_at(&self, coord: &Coord, direction: &Direction) -> Coord {
        let primary_target = coord.at(direction);
        if self.get(&primary_target).is_some() {
            primary_target
        } else {
            use crate::Direction::*;
            todo!()
        }
    }
}

struct Cube {
    nodes: HashMap<Coord3D, (Node, Coord)>,
}

impl Cube {}

impl TryFrom<Map> for Cube {
    type Error = anyhow::Error;

    fn try_from(value: Map) -> Result<Self> {
        todo!()
    }
}

#[cfg(test)]
mod test {}

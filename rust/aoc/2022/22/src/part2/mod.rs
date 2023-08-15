use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;

use anyhow::Result;

use space3d::{Coord as Coord3D, Transformation as Transformation3D};

use crate::{Coord as Coord2D, CoordUnit, Direction as Direction2D, Map, Node};

mod space3d;

pub(crate) trait MapPart2 {
    fn coord_at(&self, coord: &Coord2D, direction: &Direction2D) -> Coord2D;
}

impl MapPart2 for Map {
    fn coord_at(&self, coord: &Coord2D, direction: &Direction2D) -> Coord2D {
        let primary_target = coord.at(direction);
        if self.get(&primary_target).is_some() {
            primary_target
        } else {
            use crate::Direction::*;
            todo!()
        }
    }
}

pub(crate) struct Cube {
    nodes: HashMap<Coord3D, (Node, Coord2D)>,
    faces_size: CoordUnit,
    transformations: HashMap<FaceCoord, Transformation3D>,
}

impl Cube {
    pub(crate) fn coord_at(&self, coord: &Coord2D, direction: &Direction2D) -> Coord2D {
        todo!()
    }

    fn face_coord(&self, coord: &Coord2D) -> FaceCoord {
        FaceCoord(Coord2D::new(
            coord.x / self.faces_size,
            coord.y / self.faces_size,
        ))
    }
}

impl TryFrom<&Map> for Cube {
    type Error = anyhow::Error;

    fn try_from(map: &Map) -> Result<Self> {
        let faces_size: CoordUnit = (map
            .nodes
            .keys()
            .fold(0 as CoordUnit, |max, coord| max.max(coord.x).max(coord.y))
            + 1)
            / 4;

        let nodes = HashMap::new();
        let transformations = HashMap::new();

        let origin = (0 as CoordUnit..)
            .map(|x| Coord2D::new(x, 0))
            .find(|coord| map.get(&coord).is_some())
            .unwrap();

        todo!();
        // let transformation = Transformation3D::translate(Vec3D::)

        Ok(Self {
            faces_size,
            nodes,
            transformations,
        })
    }
}

struct FaceCoord(Coord2D);

impl Deref for FaceCoord {
    type Target = Coord2D;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {}

use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;

use anyhow::Result;

use space3d::{Direction as Direction3D, Transformation as Transformation3D, Vec3D};

use crate::{Coord as Coord2D, CoordUnit, Direction as Direction2D, Map, Side as Side2D};

use crate as space2d;

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
    faces_size: CoordUnit,
    transformations: HashMap<FaceCoord, Transformation3D>,
}

impl Cube {
    pub(crate) fn coord_at(&self, _coord: &Coord2D, _direction: &Direction2D) -> Coord2D {
        todo!()
    }

    fn apply(&self, _position: &space2d::Position) -> space3d::Position {
        // let coord = space3d::Coord::from(&position.coord);
        //
        // let unmapped_position = space3d::Position::new(coord, space3d::Orientation::new(
        //     direction: ,
        //     space3d::Direction::Front
        // ));
        //
        // let mut position = self.transformations.get(&self.face_coord(&position.coord)).unwrap()
        //     .apply_position(&unmapped_position);

        todo!()
    }

    fn revert(&self, _position: &space3d::Position) -> space2d::Position {
        todo!()
    }

    fn face_coord(&self, origin: &Coord2D) -> FaceCoord {
        FaceCoord(Coord2D::new(
            origin.x / self.faces_size,
            origin.y / self.faces_size,
        ))
    }

    fn origin_of(&self, face: &FaceCoord) -> Coord2D {
        Coord2D::new(face.x * self.faces_size, face.y * self.faces_size)
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

        let mut cube = Self {
            faces_size,
            transformations: HashMap::new(),
        };

        let origin = (0 as CoordUnit..)
            .map(|x| Coord2D::new(x, 0))
            .find(|coord| map.get(coord).is_some())
            .unwrap();

        let origin_transformation =
            Transformation3D::translate(&Vec3D::new(1 - origin.x, 1 - origin.y, 0));

        let face_coord = cube.face_coord(&origin);
        cube.transformations
            .insert(face_coord.clone(), origin_transformation);

        fn explore_and_register_transformations(
            from: &FaceCoord,
            map: &Map,
            cube: &mut Cube,
        ) -> Result<()> {
            for dir in [
                Direction2D::Up,
                Direction2D::Left,
                Direction2D::Right,
                Direction2D::Down,
            ] {
                let face_coord = from.at(&dir);
                let origin = cube.origin_of(&face_coord);
                if map.get(&origin).is_some() && !cube.transformations.contains_key(&face_coord) {
                    let from_origin = cube.origin_of(from);
                    let from_transformation = cube.transformations.get(from).unwrap();
                    let origin_move: Vec3D = Vec3D::from_start_to_end(
                        &space3d::Coord::from(&from_origin),
                        &space3d::Coord::from(&origin),
                    ) + Vec3D::k();
                    let origin_move_in_from_referential =
                        from_transformation.apply_vec(&origin_move);

                    let left_of_direction_in_from_ref =
                        from_transformation.apply_vec(&Vec3D::from(&dir.turn(&Side2D::Left)));

                    let transformation: Transformation3D = from_transformation
                        .then(&Transformation3D::rotate_half_pi(&Direction3D::try_from(
                            &left_of_direction_in_from_ref,
                        )?))
                        .then(&Transformation3D::translate(
                            &origin_move_in_from_referential,
                        ));
                    cube.transformations
                        .insert(face_coord.clone(), transformation);

                    explore_and_register_transformations(&face_coord, map, cube)?;
                }
            }

            Ok(())
        }

        explore_and_register_transformations(&face_coord, map, &mut cube)?;

        Ok(cube)
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct FaceCoord(Coord2D);

impl FaceCoord {
    fn at(&self, direction: &Direction2D) -> FaceCoord {
        FaceCoord(self.0.at(direction))
    }
}

impl Deref for FaceCoord {
    type Target = Coord2D;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    mod creating_cube {
        use crate::part2::Cube;

        #[test]
        fn should_not_fail() {
            let (map, _) = crate::parse(&["aoc", "2022", "22-test.txt"]).unwrap();
            Cube::try_from(&map).unwrap();
        }

        #[test]
        fn should_map_first_origin_to_1_1_0() {
            let (map, _) = crate::parse(&["aoc", "2022", "22-test.txt"]).unwrap();
            let cube = Cube::try_from(&map).unwrap();
        }
    }
}

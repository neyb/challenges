use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;

use anyhow::Result;

use space3d::Vec3D;

use crate::{CoordUnit, Map};

use crate as space2d;

mod space3d;

pub(super) struct Cube {
    faces_size: CoordUnit,
    transformations: HashMap<FaceCoord, space3d::Transformation>,
    faces_by_direction: HashMap<space3d::Direction, FaceCoord>,
}

impl Cube {
    pub(super) fn jump(&self, position: &space2d::Position) -> space2d::Coord {
        let position2d = space2d::Position {
            coord: position.coord.clone(),
            direction: position.direction,
        };

        let mut position3d = self.apply(&position2d);
        position3d.move_front();
        position3d.turn(&space3d::Side::Down);
        position3d.move_front();

        self.revert(&position3d).coord
    }

    fn apply(&self, position: &space2d::Position) -> space3d::Position {
        self.transformations
            .get(&self.face_coord_of_2d_coord(&position.coord))
            .unwrap()
            .apply_position(&position.into())
    }

    fn revert(&self, position: &space3d::Position) -> space2d::Position {
        self.transformations
            .get(&self.face_coord_of_3d_coord(&position.coord))
            .unwrap()
            .revert_position(position)
            .try_into()
            .unwrap()
    }

    fn face_coord_of_2d_coord(&self, coord: &space2d::Coord) -> FaceCoord {
        FaceCoord(space2d::Coord::new(
            coord.x / self.faces_size,
            coord.y / self.faces_size,
        ))
    }

    fn face_coord_of_3d_coord(&self, coord: &space3d::Coord) -> FaceCoord {
        let space3d::Coord { x, y, z } = coord;
        use space3d::Direction::*;
        let coord_face_direction = match (x, y, z) {
            (0, _, _) => Left,
            (&x, _, _) if x == self.faces_size + 1 => Right,
            (_, 0, _) => Up,
            (_, &y, _) if y == self.faces_size + 1 => Down,
            (_, _, 0) => Front,
            (_, _, &z) if z == self.faces_size + 1 => Back,
            _ => panic!("oO"),
        };
        self.faces_by_direction
            .get(&coord_face_direction)
            .unwrap_or_else(|| panic!("no faces for direction {:?}", coord_face_direction))
            .clone()
    }

    fn origin_of(&self, face: &FaceCoord) -> space2d::Coord {
        space2d::Coord::new(face.x * self.faces_size, face.y * self.faces_size)
    }

    fn insert_transformation(
        &mut self,
        transformation: space3d::Transformation,
        face_coord: FaceCoord,
        direction: space3d::Direction,
    ) {
        self.transformations
            .insert(face_coord.clone(), transformation);
        self.faces_by_direction.insert(direction, face_coord);
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
            faces_by_direction: HashMap::new(),
        };

        let origin = (0 as CoordUnit..)
            .map(|x| space2d::Coord::new(x, 0))
            .find(|coord| map.get(coord).is_some())
            .unwrap();

        let origin_transformation =
            space3d::Transformation::translate(&Vec3D::new(1 - origin.x, 1 - origin.y, 0));

        let face_coord = cube.face_coord_of_2d_coord(&origin);
        cube.insert_transformation(
            origin_transformation,
            face_coord.clone(),
            space3d::Direction::Front,
        );

        explore_and_register_transformations(
            &face_coord,
            &space3d::Direction::Front,
            map,
            &mut cube,
        )?;

        fn explore_and_register_transformations(
            from: &FaceCoord,
            from_direction: &space3d::Direction,
            map: &Map,
            cube: &mut Cube,
        ) -> Result<()> {
            for dir in [
                space2d::Direction::Up,
                space2d::Direction::Left,
                space2d::Direction::Right,
                space2d::Direction::Down,
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

                    let left_of_direction_in_from_ref = from_transformation
                        .apply_vec(&Vec3D::from(&dir.turn(&space2d::Side::Left)));

                    let rotate_direction =
                        space3d::Direction::try_from(&left_of_direction_in_from_ref)?;

                    let rotation = space3d::Transformation::rotate_half_pi(&rotate_direction);
                    let transformation = from_transformation.then(&rotation).then(
                        &space3d::Transformation::translate(&origin_move_in_from_referential),
                    );
                    let direction = transformation.apply_direction(from_direction);
                    cube.insert_transformation(transformation, face_coord.clone(), direction);

                    explore_and_register_transformations(&face_coord, &direction, map, cube)?;
                }
            }

            Ok(())
        }

        Ok(cube)
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct FaceCoord(space2d::Coord);

impl FaceCoord {
    fn at(&self, direction: &space2d::Direction) -> FaceCoord {
        FaceCoord(self.0.at(direction))
    }
}

impl Deref for FaceCoord {
    type Target = space2d::Coord;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    mod creating_cube {
        use crate as space2d;
        use crate::part2::space3d;

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
            let position = space2d::Position {
                coord: space2d::Coord::new(8, 0),
                direction: space2d::Direction::Right,
            };

            let position3d = cube.apply(&position);

            assert_eq!(
                position3d,
                space3d::Position::new(
                    space3d::Coord::new(1, 1, 0),
                    space3d::Orientation::new(space3d::Direction::Right, space3d::Direction::Front)
                )
            );
        }

        #[test]
        fn should_map_a_random_point_in_first_face() {}
    }
}

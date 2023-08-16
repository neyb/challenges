use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;

use anyhow::Result;

use space3d::{Direction as Direction3D, Transformation as Transformation3D, Vec3D};

use crate::{CoordUnit, Map};

use crate as space2d;

mod space3d;

pub(crate) trait MapPart2 {
    fn coord_at(&self, coord: &space2d::Coord, direction: &space2d::Direction) -> space2d::Coord;
}

impl MapPart2 for Map {
    fn coord_at(&self, coord: &space2d::Coord, direction: &space2d::Direction) -> space2d::Coord {
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
    pub(crate) fn coord_at(
        &self,
        _coord: &space2d::Coord,
        _direction: &space2d::Direction,
    ) -> space2d::Coord {
        todo!()
    }

    fn apply(&self, position: &space2d::Position) -> space3d::Position {
        let coord = space3d::Coord::from(&position.coord);

        let unmapped_position = space3d::Position::new(
            coord,
            space3d::Orientation::new(
                space3d::Direction::from(&position.direction),
                space3d::Direction::Front,
            ),
        );

        self.transformations
            .get(&self.face_coord(&position.coord))
            .unwrap()
            .apply_position(&unmapped_position)
    }

    fn revert(&self, _position: &space3d::Position) -> space2d::Position {
        todo!()
    }

    fn face_coord(&self, origin: &space2d::Coord) -> FaceCoord {
        FaceCoord(space2d::Coord::new(
            origin.x / self.faces_size,
            origin.y / self.faces_size,
        ))
    }

    fn origin_of(&self, face: &FaceCoord) -> space2d::Coord {
        space2d::Coord::new(face.x * self.faces_size, face.y * self.faces_size)
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
            .map(|x| space2d::Coord::new(x, 0))
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
    }
}

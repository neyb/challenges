use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;

use anyhow::Result;

use crate::{CoordUnit, Map};

use crate as space2d;
use crate::part2::space3d::Transformation;

mod space3d;

pub(super) struct Cube {
    faces_size: CoordUnit,
    transformations: HashMap<FaceCoord, Transformation>,
    face_coords_by_direction: HashMap<space3d::Direction, FaceCoord>,
}

impl Cube {
    pub(super) fn jump(&self, position: &space2d::Position) -> space2d::Position {
        let mut position3d = self.apply(position);
        position3d.move_front();
        position3d.turn(&space3d::Side::Down);
        position3d.move_front();
        self.revert(&position3d)
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
        FaceCoord::new(coord.x / self.faces_size, coord.y / self.faces_size)
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
            _ => panic!("{coord:?} is not on the cube"),
        };
        self.face_coords_by_direction
            .get(&coord_face_direction)
            .unwrap_or_else(|| panic!("no faces for direction {:?}", coord_face_direction))
            .clone()
    }

    fn origin_of(&self, face: &FaceCoord) -> space2d::Coord {
        space2d::Coord::new(face.x * self.faces_size, face.y * self.faces_size)
    }

    fn insert_transformation(&mut self, transformation: Transformation, face_coord: FaceCoord) {
        let direction = transformation.apply_direction(&space3d::Direction::Front);
        self.transformations
            .insert(face_coord.clone(), transformation);
        self.face_coords_by_direction.insert(direction, face_coord);
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
            face_coords_by_direction: HashMap::new(),
        };

        let origin = (0 as CoordUnit..)
            .map(|x| space2d::Coord::new(x, 0))
            .find(|coord| map.get(coord).is_some())
            .unwrap();

        let origin_transformation =
            Transformation::translate(&space3d::Vec3D::new(1 - origin.x, 1 - origin.y, 0));

        let face_coord = cube.face_coord_of_2d_coord(&origin);
        cube.insert_transformation(origin_transformation, face_coord.clone());

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
                let face_origin = cube.origin_of(&face_coord);
                if map.get(&face_origin).is_some()
                    && !cube.transformations.contains_key(&face_coord)
                {
                    let from_transformation = cube.transformations.get(from).unwrap();

                    let fold_position = from_transformation
                        .apply_coord(&space3d::Coord::from(&first_not_in_face(from, &dir, cube)));

                    let translate_to_origin = Transformation::translate(
                        &space3d::Vec3D::from_start_to_end(&fold_position, &space3d::Coord::orig()),
                    );
                    let translate_dir = Transformation::translate(
                        &from_transformation.apply_vec(&space3d::Direction::from(&dir).as_vec()),
                    );
                    let rotate = get_rotation(&dir, from_transformation)?;

                    let translate_from_origin = Transformation::translate(
                        &space3d::Vec3D::from_start_to_end(&space3d::Coord::orig(), &fold_position),
                    );

                    let transformation = from_transformation
                        .then(&translate_to_origin)
                        .then(&translate_dir)
                        .then(&rotate)
                        .then(&translate_from_origin);

                    cube.insert_transformation(transformation, face_coord.clone());

                    explore_and_register_transformations(&face_coord, map, cube)?;
                }
            }

            fn get_rotation(
                dir: &space2d::Direction,
                from_transformation: &Transformation,
            ) -> Result<Transformation> {
                let left_of_direction_in_from_ref = from_transformation
                    .apply_vec(&space3d::Vec3D::from(&dir.turn(&space2d::Side::Left)));

                let rotate_direction =
                    space3d::Direction::try_from(&left_of_direction_in_from_ref)?;

                let rotation = Transformation::rotate_half_pi(&rotate_direction);
                Ok(rotation)
            }

            fn first_not_in_face(
                face_coord: &FaceCoord,
                direction: &space2d::Direction,
                cube: &Cube,
            ) -> space2d::Coord {
                let mut result = cube.origin_of(face_coord);

                while &cube.face_coord_of_2d_coord(&result) == face_coord {
                    result = result.at(direction)
                }

                result
            }

            Ok(())
        }

        explore_and_register_transformations(&face_coord, map, &mut cube)?;

        Ok(cube)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct FaceCoord(space2d::Coord);

impl FaceCoord {
    fn new(x: CoordUnit, y: CoordUnit) -> Self {
        Self(space2d::Coord::new(x, y))
    }
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
        use crate::part2::{space3d, FaceCoord};
        use std::collections::HashMap;

        use crate::part2::Cube;

        #[test]
        fn should_not_fail() {
            let (map, _) = crate::parse(&["aoc", "2022", "22-test.txt"]).unwrap();
            Cube::try_from(&map).unwrap();
        }

        #[test]
        fn should_have_right_faces_size() {
            let (map, _) = crate::parse(&["aoc", "2022", "22-test.txt"]).unwrap();
            let cube = Cube::try_from(&map).unwrap();
            assert_eq!(cube.faces_size, 4);
        }

        #[test]
        fn should_create_6_transformations() {
            let (map, _) = crate::parse(&["aoc", "2022", "22-test.txt"]).unwrap();
            let cube = Cube::try_from(&map).unwrap();
            assert_eq!(cube.transformations.len(), 6);
        }

        #[test]
        fn should_map_8_0_to_1_1_0() {
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

        fn test_mapping(coord: space2d::Coord, expected_position: space3d::Position) {
            let (map, _) = crate::parse(&["aoc", "2022", "22-test.txt"]).unwrap();
            let cube = Cube::try_from(&map).unwrap();
            let position = space2d::Position {
                coord,
                direction: space2d::Direction::Right,
            };

            let position3d = cube.apply(&position);

            assert_eq!(position3d, expected_position);
        }

        #[test]
        fn should_map_8_4_to_1_5_1() {
            use space3d::Direction::*;
            test_mapping(
                space2d::Coord::new(8, 4),
                space3d::Position::new(
                    space3d::Coord::new(1, 5, 1),
                    space3d::Orientation::new(Right, Down),
                ),
            );
        }

        #[test]
        fn should_map_4_4_to_0_1_1() {
            use space3d::Direction::*;
            test_mapping(
                space2d::Coord::new(4, 4),
                space3d::Position::new(
                    space3d::Coord::new(0, 1, 1),
                    space3d::Orientation::new(Down, Left),
                ),
            );
        }

        #[test]
        fn should_map_0_4_to_4_0_1() {
            use space3d::Direction::*;
            test_mapping(
                space2d::Coord::new(0, 4),
                space3d::Position::new(
                    space3d::Coord::new(4, 0, 1),
                    space3d::Orientation::new(Left, Up),
                ),
            );
        }

        #[test]
        fn should_map_8_8_to_1_4_5() {
            use space3d::Direction::*;
            test_mapping(
                space2d::Coord::new(8, 8),
                space3d::Position::new(
                    space3d::Coord::new(1, 4, 5),
                    space3d::Orientation::new(Right, Back),
                ),
            );
        }

        #[test]
        fn should_map_12_8_to_5_4_4() {
            use space3d::Direction::*;
            test_mapping(
                space2d::Coord::new(12, 8),
                space3d::Position::new(
                    space3d::Coord::new(5, 4, 4),
                    space3d::Orientation::new(Front, Right),
                ),
            );
        }

        #[test]
        fn should_have_6_face_coords_by_direction() {
            use space3d::Direction::*;

            let (map, _) = crate::parse(&["aoc", "2022", "22-test.txt"]).unwrap();
            let cube = Cube::try_from(&map).unwrap();
            assert_eq!(
                cube.face_coords_by_direction,
                HashMap::from([
                    (Front, FaceCoord::new(2, 0)),
                    (Down, FaceCoord::new(2, 1)),
                    (Left, FaceCoord::new(1, 1)),
                    (Up, FaceCoord::new(0, 1)),
                    (Back, FaceCoord::new(2, 2)),
                    (Right, FaceCoord::new(3, 2)),
                ])
            );
        }

        #[test]
        fn test_first_jump_from_given_test() {
            let (map, _) = crate::parse(&["aoc", "2022", "22-test.txt"]).unwrap();
            let cube = Cube::try_from(&map).unwrap();

            let from_position = space2d::Position {
                coord: space2d::Coord::new(11, 5),
                direction: space2d::Direction::Right,
            };

            let arrival = cube.jump(&from_position);

            assert_eq!(
                arrival,
                space2d::Position {
                    coord: space2d::Coord::new(14, 8),
                    direction: space2d::Direction::Down
                }
            )
        }

        #[test]
        fn test_other_jump_from_given_test() {
            let (map, _) = crate::parse(&["aoc", "2022", "22-test.txt"]).unwrap();
            let cube = Cube::try_from(&map).unwrap();

            let from_position = space2d::Position {
                coord: space2d::Coord::new(10, 11),
                direction: space2d::Direction::Down,
            };

            let arrival = cube.jump(&from_position);

            assert_eq!(
                arrival,
                space2d::Position {
                    coord: space2d::Coord::new(1, 7),
                    direction: space2d::Direction::Up
                }
            )
        }
    }
}

use anyhow::{bail, Error, Result};
use std::ops::{Add, Neg, Sub};

use crate as space2d;
use crate::{Coord as Coord2D, CoordUnit};

#[derive(Debug, PartialEq)]
pub(super) struct Position {
    pub(super) coord: Coord,
    orientation: Orientation,
}

impl Position {
    pub(super) fn new(coord: Coord, orientation: Orientation) -> Self {
        Self { coord, orientation }
    }

    pub(super) fn move_front(&mut self) {
        let direction = &self.orientation.front;
        self.coord.move_towards(direction);
    }

    pub(super) fn turn(&mut self, side: &Side) {
        self.orientation = self.orientation.turn(*side);
    }
}

impl From<&space2d::Position> for Position {
    fn from(position: &space2d::Position) -> Self {
        let coord = Coord::from(&position.coord);

        Position::new(
            coord,
            Orientation::new(Direction::from(&position.direction), Direction::Front),
        )
    }
}

impl TryInto<space2d::Position> for Position {
    type Error = Error;

    fn try_into(self) -> Result<space2d::Position> {
        Ok(space2d::Position {
            coord: self.coord.try_into()?,
            direction: self.orientation.front.try_into()?,
        })
    }
}

#[derive(Eq, PartialEq, Debug)]
pub(super) struct Orientation {
    front: Direction,
    up: Direction,
}

impl Orientation {
    pub(super) fn new(front: Direction, up: Direction) -> Self {
        Self { front, up }
    }

    fn turn(&self, turn: Side) -> Self {
        use Side::*;

        let up = match turn {
            Left | Right => self.up,
            Down => self.front,
            Up => self.front.opposite(),
        };

        let front = match turn {
            Up => self.up,
            Down => self.up.opposite(),
            Left => (&self.up.as_vec().cross(&self.front.as_vec()))
                .try_into()
                .unwrap(),
            Right => (&self.front.as_vec().cross(&self.up.as_vec()))
                .try_into()
                .unwrap(),
        };

        Self { front, up }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(super) enum Direction {
    Back,  // z
    Front, // -z
    Left,  // -x
    Right, // x
    Up,    // -y
    Down,  // y
}

impl Direction {
    fn opposite(&self) -> Self {
        use Direction::*;

        match self {
            Back => Front,
            Front => Back,
            Left => Right,
            Right => Left,
            Up => Down,
            Down => Up,
        }
    }

    pub(super) fn as_vec(&self) -> Vec3D {
        use Direction::*;
        match self {
            Right => Vec3D::i(),
            Down => Vec3D::j(),
            Back => Vec3D::k(),
            Left => -Vec3D::i(),
            Up => -Vec3D::j(),
            Front => -Vec3D::k(),
        }
    }
}

impl TryFrom<&Vec3D> for Direction {
    type Error = Error;

    fn try_from(vec @ Vec3D { x, y, z }: &Vec3D) -> Result<Self> {
        use Direction::*;

        Ok(match (x, y, z) {
            (1, 0, 0) => Right,
            (-1, 0, 0) => Left,
            (0, 1, 0) => Down,
            (0, -1, 0) => Up,
            (0, 0, 1) => Back,
            (0, 0, -1) => Front,
            _ => bail!("cannot find direction of {:?}", vec),
        })
    }
}

impl From<&space2d::Direction> for Direction {
    fn from(direction: &space2d::Direction) -> Self {
        use space2d::Direction::*;

        match direction {
            Up => Direction::Up,
            Left => Direction::Left,
            Right => Direction::Right,
            Down => Direction::Down,
        }
    }
}

impl TryInto<space2d::Direction> for Direction {
    type Error = Error;

    fn try_into(self) -> Result<crate::Direction> {
        use Direction::*;
        Ok(match self {
            Back | Front => {
                bail!("{:?} match no direction", self)
            }
            Left => space2d::Direction::Left,
            Right => space2d::Direction::Right,
            Up => space2d::Direction::Up,
            Down => space2d::Direction::Down,
        })
    }
}

#[allow(unused)]
#[derive(Copy, Clone)]
pub enum Side {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Vec3D {
    x: CoordUnit,
    y: CoordUnit,
    z: CoordUnit,
}

impl Vec3D {
    pub(super) fn new(x: CoordUnit, y: CoordUnit, z: CoordUnit) -> Self {
        Self { x, y, z }
    }

    pub(super) fn i() -> Self {
        Self::new(1, 0, 0)
    }

    pub(super) fn j() -> Self {
        Self::new(0, 1, 0)
    }

    pub(super) fn k() -> Self {
        Self::new(0, 0, 1)
    }

    pub(super) fn from_start_to_end(start: &Coord, end: &Coord) -> Self {
        Self::new(end.x - start.x, end.y - start.y, end.z - start.z)
    }

    fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for &Vec3D {
    type Output = Vec3D;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for Vec3D {
    type Output = Vec3D;

    fn add(mut self, other: Self) -> Self::Output {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }
}

impl Sub for &Vec3D {
    type Output = Vec3D;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for Vec3D {
    type Output = Vec3D;

    fn sub(mut self, other: Self) -> Self::Output {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self
    }
}

impl Neg for &Vec3D {
    type Output = Vec3D;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for Vec3D {
    type Output = Vec3D;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}

impl From<&crate::Direction> for Vec3D {
    fn from(direction: &crate::Direction) -> Self {
        match direction {
            crate::Direction::Up => -Vec3D::j(),
            crate::Direction::Left => -Vec3D::i(),
            crate::Direction::Right => Vec3D::i(),
            crate::Direction::Down => Vec3D::j(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct Coord {
    pub(super) x: CoordUnit,
    pub(super) y: CoordUnit,
    pub(super) z: CoordUnit,
}

impl Coord {
    pub(super) fn new(x: CoordUnit, y: CoordUnit, z: CoordUnit) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn orig() -> Self {
        Self::new(0, 0, 0)
    }

    fn get(&self, idx: usize) -> CoordUnit {
        match idx {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!(
                "only 3 elements in 3Dvec, trying accessing it with index {}",
                idx
            ),
        }
    }

    fn move_towards(&mut self, direction: &Direction) {
        let Vec3D { x, y, z } = direction.as_vec();
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

impl From<&Coord2D> for Coord {
    fn from(Coord2D { x, y }: &Coord2D) -> Self {
        Self::new(*x, *y, 0)
    }
}

impl TryInto<space2d::Coord> for Coord {
    type Error = Error;

    fn try_into(self) -> Result<Coord2D> {
        if self.z != 0 {
            bail!("cannot tranform {:?}", self)
        }

        Ok(space2d::Coord::new(self.x, self.y))
    }
}

#[derive(PartialEq, Debug)]
pub(super) struct Transformation {
    matrix: HomogeneousMatrix,
    invert_matrix: HomogeneousMatrix,
}

impl Transformation {
    pub(super) fn translate(vec: &Vec3D) -> Self {
        Self {
            matrix: HomogeneousMatrix::translate(vec),
            invert_matrix: HomogeneousMatrix::translate(&-vec),
        }
    }

    pub(super) fn rotate_half_pi(around: &Direction) -> Self {
        Self {
            matrix: HomogeneousMatrix::rotate_half_pi(around),
            invert_matrix: HomogeneousMatrix::rotate_half_pi(&around.opposite()),
        }
    }

    pub(super) fn then(&self, other: &Self) -> Self {
        Self {
            matrix: self.matrix.then(&other.matrix),
            invert_matrix: other.invert_matrix.then(&self.invert_matrix),
        }
    }

    pub(super) fn apply_coord(&self, coord: &Coord) -> Coord {
        self.matrix.apply_coord(coord)
    }

    pub(super) fn apply_direction(&self, direction: &Direction) -> Direction {
        (&self.apply_vec(&direction.as_vec())).try_into().unwrap()
    }

    pub(super) fn apply_vec(&self, vec: &Vec3D) -> Vec3D {
        self.matrix.apply_vec(vec)
    }

    pub(super) fn apply_position(&self, position: &Position) -> Position {
        self.matrix.apply_position(position)
    }

    pub(super) fn revert_position(&self, position: &Position) -> Position {
        self.invert_matrix.apply_position(position)
    }
}

#[derive(PartialEq, Debug)]
struct HomogeneousMatrix {
    values: [[CoordUnit; 4]; 4],
}

impl HomogeneousMatrix {
    fn zero() -> Self {
        Self {
            values: [[0; 4]; 4],
        }
    }

    fn id() -> Self {
        let mut result = Self::zero();
        for i in 0..4 {
            *result.get_mut(i, i) = 1;
        }
        result
    }

    pub(super) fn translate(vec: &Vec3D) -> Self {
        let mut result = Self::id();
        *result.get_mut(0, 3) = vec.x;
        *result.get_mut(1, 3) = vec.y;
        *result.get_mut(2, 3) = vec.z;
        result
    }

    fn rotate_half_pi(around: &Direction) -> Self {
        let mut result = Self::id();

        use Direction::*;
        match around {
            Right => {
                // Rx(pi/2)
                *result.get_mut(1, 1) = 0;
                *result.get_mut(1, 2) = -1;
                *result.get_mut(2, 1) = 1;
                *result.get_mut(2, 2) = 0;
            }
            Left => {
                // Rx(-pi/2)
                *result.get_mut(1, 1) = 0;
                *result.get_mut(1, 2) = 1;
                *result.get_mut(2, 1) = -1;
                *result.get_mut(2, 2) = 0;
            }
            Down => {
                // Ry(pi/2)
                *result.get_mut(0, 0) = 0;
                *result.get_mut(0, 2) = 1;
                *result.get_mut(2, 0) = -1;
                *result.get_mut(2, 2) = 0;
            }
            Up => {
                // Ry(-pi/2)
                *result.get_mut(0, 0) = 0;
                *result.get_mut(0, 2) = -1;
                *result.get_mut(2, 0) = 1;
                *result.get_mut(2, 2) = 0;
            }
            Back => {
                // Rz(pi/2)
                *result.get_mut(0, 0) = 0;
                *result.get_mut(0, 1) = -1;
                *result.get_mut(1, 0) = 1;
                *result.get_mut(1, 1) = 0;
            }
            Front => {
                // Rz(-pi/2)
                *result.get_mut(0, 0) = 0;
                *result.get_mut(0, 1) = 1;
                *result.get_mut(1, 0) = -1;
                *result.get_mut(1, 1) = 0;
            }
        };

        result
    }

    fn then(&self, other_transformation: &Self) -> Self {
        let mut result = Self::zero();

        for i in 0..4 {
            for j in 0..4 {
                for x in 0..4 {
                    *result.get_mut(i, j) += other_transformation.get(i, x) * self.get(x, j)
                }
            }
        }

        result
    }

    fn apply_coord(&self, vec: &Coord) -> Coord {
        let calc = |idx| {
            (0..3)
                .map(|i| vec.get(i) * self.get(idx, i))
                .sum::<CoordUnit>()
                + self.get(idx, 3)
        };
        let x = calc(0);
        let y = calc(1);
        let z = calc(2);
        Coord::new(x, y, z)
    }

    fn apply_vec(&self, vec: &Vec3D) -> Vec3D {
        let start = self.apply_coord(&Coord::orig());
        let end = self.apply_coord(&Coord::new(vec.x, vec.y, vec.z));
        Vec3D::from_start_to_end(&start, &end)
    }

    fn apply_position(&self, position: &Position) -> Position {
        Position {
            coord: self.apply_coord(&position.coord),
            orientation: Orientation {
                front: self.apply_direction(&position.orientation.front),
                up: self.apply_direction(&position.orientation.up),
            },
        }
    }

    fn apply_direction(&self, directon: &Direction) -> Direction {
        let vec = directon.as_vec();
        let modified_vec = self.apply_vec(&vec);
        Direction::try_from(&modified_vec).unwrap()
    }

    fn get(&self, a: usize, b: usize) -> &CoordUnit {
        self.values.get(a).unwrap().get(b).unwrap()
    }

    fn get_mut(&mut self, a: usize, b: usize) -> &mut CoordUnit {
        self.values.get_mut(a).unwrap().get_mut(b).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Direction::*;

    #[test]
    fn facing_right_turn_left() {
        let orientation = Orientation {
            front: Right,
            up: Up,
        };
        let new_orientation = orientation.turn(Side::Left);
        assert_eq!(
            new_orientation,
            Orientation {
                front: Back,
                up: Up,
            }
        )
    }

    #[test]
    fn facing_top_up_left_turn_down() {
        let orientation = Orientation {
            front: Back,
            up: Left,
        };
        let new_orientation = orientation.turn(Side::Down);
        assert_eq!(
            new_orientation,
            Orientation {
                up: Back,
                front: Right,
            }
        )
    }

    #[test]
    fn apply_translation() {
        let transformation = Transformation::translate(&Vec3D::new(5, 8, 13));
        let init = Coord::new(1, 2, 3);
        assert_eq!(transformation.apply_coord(&init), Coord::new(6, 10, 16))
    }

    #[test]
    fn apply_rotation() {
        let transformation = Transformation::rotate_half_pi(&Back);
        let init = Coord::new(1, 2, 3);
        assert_eq!(transformation.apply_coord(&init), Coord::new(-2, 1, 3))
    }

    #[test]
    fn apply_translate_then_rotate() {
        let transformation = Transformation::translate(&Vec3D::new(5, 8, 13))
            .then(&Transformation::rotate_half_pi(&Back));
        let init = Coord::new(1, 2, 3);
        let transformed = transformation.apply_coord(&init);
        assert_eq!(transformed, Coord::new(-10, 6, 16));
    }
}

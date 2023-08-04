use anyhow::{bail, Result};

use Direction::*;

use crate::{Coord as Coord2D, CoordUnit};

struct Position {
    coord: Coord,
    orientation: Orientation,
}

#[derive(Eq, PartialEq, Debug)]
struct Orientation {
    direction: Direction,
    up: Direction,
}

impl Orientation {
    fn turn(&self, turn: Side) -> Self {
        use Side::*;

        let up = match turn {
            Left | Right => self.up,
            Down => self.direction,
            Up => self.direction.opposite(),
        };

        let direction = match turn {
            Up => self.up,
            Down => self.up.opposite(),
            Left => (&self.up.as_vec().cross(&self.direction.as_vec()))
                .try_into()
                .unwrap(),
            Right => Direction::try_from(&self.up.as_vec().cross(&self.direction.as_vec()))
                .unwrap()
                .opposite(),
        };

        Self { direction, up }
    }
}

#[derive(Clone)]
pub struct Coord {
    vec: Vec3D,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    // z
    Down,
    // -z
    Left,
    // -x
    Right,
    // x
    Front,
    // -y
    Back, // y
}

impl Direction {
    fn opposite(&self) -> Self {
        use Direction::*;

        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
            Front => Back,
            Back => Front,
        }
    }

    fn as_vec(&self) -> Vec3D {
        match self {
            Up => Vec3D::new(0, 0, 1),
            Down => Vec3D::new(0, 0, -1),
            Left => Vec3D::new(-1, 0, 0),
            Right => Vec3D::new(1, 0, 0),
            Front => Vec3D::new(0, -1, 0),
            Back => Vec3D::new(0, 1, 0),
        }
    }
}

impl TryFrom<&Vec3D> for Direction {
    type Error = anyhow::Error;

    fn try_from(vec @ Vec3D { x, y, z }: &Vec3D) -> Result<Self> {
        use Direction::*;

        Ok(match (x, y, z) {
            (1, 0, 0) => Right,
            (-1, 0, 0) => Left,
            (0, 1, 0) => Back,
            (0, -1, 0) => Front,
            (0, 0, 1) => Up,
            (0, 0, -1) => Down,
            _ => bail!("cannot find direction of {:?}", vec),
        })
    }
}

#[derive(Copy, Clone)]
enum Side {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Vec3D {
    x: CoordUnit,
    y: CoordUnit,
    z: CoordUnit,
}

impl Vec3D {
    fn new(x: CoordUnit, y: CoordUnit, z: CoordUnit) -> Self {
        Self { x, y, z }
    }

    fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
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
}

impl From<&Coord2D> for Vec3D {
    fn from(Coord2D { x, y }: &Coord2D) -> Self {
        Self::new(*x, *y, 0)
    }
}

pub struct Transformation {
    values: [[CoordUnit; 4]; 4],
}

impl Transformation {
    fn zero() -> Self {
        Self {
            values: [[0; 4]; 4],
        }
    }

    fn id() -> Self {
        let mut result = Self::zero();
        for i in (0..4) {
            *result.get_mut(i, i) = 1;
        }
        result
    }

    pub fn translate(vec: &Vec3D) -> Self {
        let mut result = Self::id();
        *result.get_mut(0, 3) = vec.x;
        *result.get_mut(1, 3) = vec.y;
        *result.get_mut(2, 3) = vec.z;
        result
    }

    fn rotate_half_pi(around: &Direction) -> Self {
        let mut result = Self::id();

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
            Up => {
                // Rz(pi/2)
                *result.get_mut(0, 0) = 0;
                *result.get_mut(0, 1) = -1;
                *result.get_mut(1, 0) = 1;
                *result.get_mut(1, 1) = 0;
            }
            Down => {
                // Rz(-pi/2)
                *result.get_mut(0, 0) = 0;
                *result.get_mut(0, 1) = 1;
                *result.get_mut(1, 0) = -1;
                *result.get_mut(1, 1) = 0;
            }
            Back => {
                // Ry(pi/2)
                *result.get_mut(0, 0) = 0;
                *result.get_mut(0, 2) = 1;
                *result.get_mut(2, 0) = -1;
                *result.get_mut(2, 2) = 0;
            }
            Front => {
                // Ry(-pi/2)
                *result.get_mut(0, 0) = 0;
                *result.get_mut(0, 2) = -1;
                *result.get_mut(2, 0) = 1;
                *result.get_mut(2, 2) = 0;
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

    fn apply(&self, vec: &Vec3D) -> Vec3D {
        let calc = |idx| {
            (0..3)
                .map(|i| vec.get(i) * self.get(idx, i))
                .sum::<CoordUnit>()
                + self.get(idx, 3)
        };
        let x = calc(0);
        let y = calc(1);
        let z = calc(2);
        Vec3D::new(x, y, z)
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

    #[test]
    fn facing_right_turn_left() {
        let orientation = Orientation {
            direction: Right,
            up: Up,
        };
        let new_orientation = orientation.turn(Side::Left);
        assert_eq!(
            new_orientation,
            Orientation {
                up: Up,
                direction: Back,
            }
        )
    }

    #[test]
    fn facing_top_up_left_turn_down() {
        let orientation = Orientation {
            direction: Up,
            up: Left,
        };
        let new_orientation = orientation.turn(Side::Down);
        assert_eq!(
            new_orientation,
            Orientation {
                up: Up,
                direction: Right,
            }
        )
    }

    #[test]
    fn apply_translation() {
        let transformation = Transformation::translate(&Vec3D::new(5, 8, 13));
        let init = Vec3D::new(1, 2, 3);
        assert_eq!(transformation.apply(&init), Vec3D::new(6, 10, 16))
    }

    #[test]
    fn apply_rotation() {
        let transformation = Transformation::rotate_half_pi(&Up);
        let init = Vec3D::new(1, 2, 3);
        assert_eq!(transformation.apply(&init), Vec3D::new(-2, 1, 3))
    }

    #[test]
    fn apply_translate_then_rotate() {
        let transformation = Transformation::translate(&Vec3D::new(5, 8, 13))
            .then(&Transformation::rotate_half_pi(&Up));
        let init = Vec3D::new(1, 2, 3);
        assert_eq!(transformation.apply(&init), Vec3D::new(-10, 6, 16))
    }
}

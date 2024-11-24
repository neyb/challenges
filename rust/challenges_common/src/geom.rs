use crate::graph::Coord;
use itertools::Itertools;
use num_traits::{PrimInt, Signed};

pub struct Polygon<U = usize> {
    vertices: Vec<Point<U>>,
}

impl<U> Polygon<U>
where
    U: PrimInt,
{
    pub fn new(vertices: Vec<Point<U>>) -> Self {
        Self { vertices }
    }

    pub fn area<A: PrimInt + Signed + std::iter::Sum>(&self) -> A
    where
        U: Into<A>,
    {
        self.vertices
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| a.0.x.into() * b.0.y.into() - a.0.y.into() * b.0.x.into())
            .sum::<A>()
            .abs()
            / A::from(2).unwrap()
    }
}

pub struct Point<U = usize>(Coord<U>);

impl<U> From<Coord<U>> for Point<U> {
    fn from(coord: Coord<U>) -> Self {
        Self(coord)
    }
}

impl<U> From<&Coord<U>> for Point<U>
where
    U: Clone,
{
    fn from(coord: &Coord<U>) -> Self {
        Self(coord.clone())
    }
}

impl<U> From<Point<U>> for Coord<U> {
    fn from(val: Point<U>) -> Self {
        val.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_area() {
        let square = Polygon::new(vec![
            Point(Coord { x: 0, y: 0 }),
            Point(Coord { x: 4, y: 0 }),
            Point(Coord { x: 4, y: 4 }),
            Point(Coord { x: 0, y: 4 }),
        ]);

        assert_eq!(square.area::<i8>(), 16);
    }

    #[test]
    fn concave_polygon_with_only_right_angle_area() {
        let square = Polygon::new(vec![
            Point(Coord { x: 0, y: 0 }),
            Point(Coord { x: 4, y: 0 }),
            Point(Coord { x: 4, y: 2 }),
            Point(Coord { x: 2, y: 2 }),
            Point(Coord { x: 2, y: 4 }),
            Point(Coord { x: 0, y: 4 }),
        ]);

        assert_eq!(square.area::<i8>(), 12);
    }
}

use itertools::Itertools;
use num_traits::{zero, Num, PrimInt, Signed};
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Grid<N, U = usize> {
    width: U,
    content: Vec<N>,
}

impl<N, U> Grid<N, U>
where
    U: PrimInt,
{
    pub fn width(&self) -> U {
        self.width
    }

    pub fn height(&self) -> U {
        U::from(self.content.len()).unwrap() / self.width
    }

    pub fn get(&self, coord: &Coord<U>) -> Option<&N>
    where
        U: Num + Copy,
    {
        self.content.get(self.get_index(coord)?)
    }

    pub fn get_mut(&mut self, coord: &Coord<U>) -> Option<&mut N> {
        let i = self.get_index(coord);
        self.content.get_mut(i?)
    }

    fn get_coord_from_index(&self, index: usize) -> Option<Coord<U>> {
        if index < self.content.len() {
            Some(Coord {
                x: U::from(index % self.width.to_usize().unwrap()).unwrap(),
                y: U::from(index / self.width.to_usize().unwrap()).unwrap(),
            })
        } else {
            None
        }
    }

    fn get_index(&self, coord: &Coord<U>) -> Option<usize> {
        if self.is_coord_inside(coord) {
            Some((coord.x + self.width * coord.y).to_usize().unwrap())
        } else {
            None
        }
    }

    fn is_coord_inside(&self, coord: &Coord<U>) -> bool {
        U::zero() <= coord.y
            && coord.y < self.width
            && U::zero() <= coord.x
            && coord.x < self.width()
    }

    pub fn nodes(&self) -> &Vec<N> {
        &self.content
    }

    pub fn coords(&self) -> impl Iterator<Item = Coord<U>> + '_ {
        (0_usize..self.width().to_usize().unwrap()).flat_map(|x| {
            (0_usize..self.height().to_usize().unwrap()).map(move |y| Coord {
                x: U::from(x).unwrap(),
                y: U::from(y).unwrap(),
            })
        })
    }

    pub fn find(&self, predicate: impl Fn(&N) -> bool) -> Option<Coord<U>> {
        self.content
            .iter()
            .enumerate()
            .find(|(_i, n)| predicate(n))
            .and_then(|(i, _n)| self.get_coord_from_index(i))
    }

    pub fn neighbours(&self, coord: &Coord<U>) -> impl Iterator<Item = (Coord<U>, &N)> + '_ {
        coord
            .neighbours(false)
            .filter_map(|coord| self.get(&coord).map(move |n| (coord, n)))
    }
}

impl<IntoIt, N, U> From<IntoIt> for Grid<N, U>
where
    IntoIt: IntoIterator,
    IntoIt::Item: IntoIterator<Item = N>,
    U: PrimInt,
{
    fn from(into_it: IntoIt) -> Self {
        let mut width = None;

        let content = into_it
            .into_iter()
            .flat_map(|line| match width {
                None => {
                    let line = line.into_iter().collect_vec();
                    width = U::from(line.len());
                    line
                }
                Some(_) => line.into_iter().collect_vec(),
            })
            .collect();

        Self {
            width: width.unwrap_or(zero()),
            content,
        }
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CannotParseGrid {
    #[error("Cannot parse grid from \"{0}\": {1}")]
    CannotParseNode(String, #[source] CannotParseElementFromChar),
    #[error("Cannot parse grid from \"{str}\": all lines does not have the same length: line {line_index} has length {line_length}")]
    AllLinesDoesNotHaveSameLength {
        str: String,
        line_index: usize,
        line_length: usize,
    },
}

#[derive(Error, Debug)]
#[error("Cannot parse element from char: {char}")]
pub struct CannotParseElementFromChar {
    char: char,
}

impl From<char> for CannotParseElementFromChar {
    fn from(char: char) -> Self {
        Self { char }
    }
}

impl From<Infallible> for CannotParseElementFromChar {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

impl<N, U> FromStr for Grid<N, U>
where
    N: TryFrom<char>,
    N::Error: Into<CannotParseElementFromChar>,
    U: PrimInt,
{
    type Err = CannotParseGrid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut content = Vec::with_capacity(s.len());
        let mut width = None;

        for (line_index, line) in s.lines().enumerate() {
            match width {
                None => {
                    width = Some(line.len());
                }
                Some(width) if width != line.len() => {
                    return Err(CannotParseGrid::AllLinesDoesNotHaveSameLength {
                        str: s.to_string(),
                        line_index,
                        line_length: line.len(),
                    })
                }
                _ => (),
            };

            for c in line.chars() {
                content.push(
                    N::try_from(c)
                        .map_err(|e| CannotParseGrid::CannotParseNode(s.to_string(), e.into()))?,
                );
            }
        }

        Ok(Self {
            width: width.map(|width| U::from(width).unwrap()).unwrap_or(zero()),
            content,
        })
    }
}

impl<N, U> Display for Grid<N, U>
where
    N: Display,
    U: PrimInt,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, n) in self.content.iter().enumerate() {
            write!(f, "{}", n)?;
            if i % self.width.to_usize().unwrap() == self.width.to_usize().unwrap() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Coord<U = usize> {
    pub x: U,
    pub y: U,
}

impl<U: PrimInt> Coord<U> {
    pub fn try_at(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::Up => self
                .y
                .checked_sub(&U::one())
                .map(|y| Coord { x: self.x, y }),
            Direction::Down => Some(Coord {
                x: self.x,
                y: self.y + U::one(),
            }),
            Direction::Left => self
                .x
                .checked_sub(&U::one())
                .map(|x| Coord { x, y: self.y }),
            Direction::Right => Some(Coord {
                x: self.x + U::one(),
                y: self.y,
            }),
        }
    }

    pub fn try_at_dist(&self, dir: Direction, dist: impl Into<U>) -> Option<Self> {
        match dir {
            Direction::Up => Some(Coord {
                x: self.x,
                y: self.y.checked_sub(&dist.into())?,
            }),
            Direction::Down => Some(Coord {
                x: self.x,
                y: self.y + dist.into(),
            }),
            Direction::Left => Some(Coord {
                x: self.x.checked_sub(&dist.into())?,
                y: self.y,
            }),
            Direction::Right => Some(Coord {
                x: self.x + dist.into(),
                y: self.y,
            }),
        }
    }

    pub fn manhattan_dist_to(&self, to: &Self) -> U {
        let dist = |a: U, b: U| if a > b { a - b } else { b - a };
        dist(self.x, to.x) + dist(self.y, to.y)
    }
}

impl<U: PrimInt + Signed> Coord<U> {
    pub fn at(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Coord {
                x: self.x,
                y: self.y - U::one(),
            },
            Direction::Down => Coord {
                x: self.x,
                y: self.y + U::one(),
            },
            Direction::Left => Coord {
                x: self.x - U::one(),
                y: self.y,
            },
            Direction::Right => Coord {
                x: self.x + U::one(),
                y: self.y,
            },
        }
    }

    pub fn at_dist(&self, dir: Direction, dist: impl Into<U>) -> Self {
        match dir {
            Direction::Up => Coord {
                x: self.x,
                y: self.y - dist.into(),
            },
            Direction::Down => Coord {
                x: self.x,
                y: self.y + dist.into(),
            },
            Direction::Left => Coord {
                x: self.x - dist.into(),
                y: self.y,
            },
            Direction::Right => Coord {
                x: self.x + dist.into(),
                y: self.y,
            },
        }
    }
}

impl<U: PrimInt> Coord<U> {
    pub fn neighbours(&self, with_diag: bool) -> impl Iterator<Item = Self> {
        let mut result = Vec::with_capacity(if with_diag { 8 } else { 4 });

        use Direction::*;
        [Up, Down, Left, Right]
            .into_iter()
            .flat_map(|dir| self.try_at(dir))
            .for_each(|coord| result.push(coord));

        if with_diag {
            [Up, Down]
                .into_iter()
                .flat_map(|dir| self.try_at(dir))
                .flat_map(|coord| {
                    [Left, Right]
                        .into_iter()
                        .filter_map(move |dir| coord.try_at(dir))
                })
                .for_each(|coord| result.push(coord));
        }

        result.into_iter()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }

    pub fn turn(&self, turn: Turn) -> Self {
        match turn {
            Turn::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            Turn::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
        }
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Turn {
    Left,
    Right,
}

impl Turn {
    pub fn all() -> [Turn; 2] {
        [Turn::Left, Turn::Right]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn neighbours_of_11_with_diag_are_8() {
        let coord = Coord { x: 1, y: 1 };
        assert_eq!(coord.neighbours(true).count(), 8)
    }

    #[test]
    fn neighbours_of_11_are_4() {
        let coord = Coord { x: 1, y: 1 };
        assert_eq!(coord.neighbours(false).count(), 4)
    }

    #[test]
    fn neighbours_of_00_are_2() {
        let coord = Coord::<u32> { x: 0, y: 0 };
        assert_eq!(coord.neighbours(false).count(), 2)
    }

    #[test]
    fn grid_at_outside_of_grid_is_none() {
        let grid = Grid {
            width: 2,
            content: vec![1u8, 2, 3, 4],
        };
        assert_eq!(grid.get(&Coord { x: 2, y: 0 }), None);
        assert_eq!(grid.get(&Coord { x: -1, y: 0 }), None);
        assert_eq!(grid.get(&Coord { x: 0, y: 2 }), None);
        assert_eq!(grid.get(&Coord { x: 0, y: -1 }), None);
    }

    #[test]
    fn grid_at() {
        let grid = Grid {
            width: 2,
            content: vec![1u8, 2, 3, 4],
        };

        assert_eq!(grid.get(&Coord { x: 0, y: 0 }), Some(&1));
        assert_eq!(grid.get(&Coord { x: 1, y: 0 }), Some(&2));
        assert_eq!(grid.get(&Coord { x: 0, y: 1 }), Some(&3));
        assert_eq!(grid.get(&Coord { x: 1, y: 1 }), Some(&4));
    }
}

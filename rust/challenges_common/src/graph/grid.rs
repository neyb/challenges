use itertools::Itertools;
use num_traits::{CheckedSub, Num, PrimInt, Signed, ToPrimitive};
use std::str::FromStr;

#[derive(Debug)]
pub struct Grid<N> {
    width: usize,
    content: Vec<N>,
}

impl<N> Grid<N> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.content.len() / self.width
    }

    pub fn at(&self, coord: &Coord) -> Option<&N> {
        if coord.x < self.width {
            self.content.get(coord.x + self.width * coord.y)
        } else {
            None
        }
    }

    pub fn at_mut(&mut self, coord: &Coord) -> Option<&mut N> {
        if coord.x < self.width {
            self.content.get_mut(coord.x + self.width * coord.y)
        } else {
            None
        }
    }

    pub fn coords(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.width()).flat_map(|x| (0..self.height()).map(move |y| Coord { x, y }))
    }

    pub fn neighbours(&self, coord: &Coord) -> impl Iterator<Item = (Coord, &N)> + '_ {
        coord
            .neighbours(false)
            .filter_map(|coord| self.at(&coord).map(move |n| (coord, n)))
    }
}

impl<IntoIt, N> From<IntoIt> for Grid<N>
where
    IntoIt: IntoIterator,
    IntoIt::Item: IntoIterator<Item = N>,
{
    fn from(into_it: IntoIt) -> Self {
        let mut width = None;

        let content = into_it
            .into_iter()
            .flat_map(|line| match width {
                None => {
                    let line = line.into_iter().collect_vec();
                    width = Some(line.len());
                    line
                }
                Some(_) => line.into_iter().collect_vec(),
            })
            .collect();

        Self {
            width: width.unwrap_or(0),
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

impl<N> FromStr for Grid<N>
where
    N: TryFrom<char>,
    N::Error: Into<CannotParseElementFromChar>,
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
            width: width.unwrap_or(0),
            content,
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Coord<U: Num + Copy = usize> {
    pub x: U,
    pub y: U,
}

impl<U: Num + CheckedSub + Copy> Coord<U> {
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
}

impl<U: Num + Signed + Copy> Coord<U> {
    pub fn manhattan_dist_to(&self, to: &Self) -> U {
        (self.x - to.x).abs() + (self.y - to.y).abs()
    }

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
}

impl<U: Num + Copy + CheckedSub> Coord<U> {
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

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod test {
    use super::Coord;

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
}

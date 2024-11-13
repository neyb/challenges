use challenges_common::graph::{CannotParseElementFromChar, CannotParseGrid, Coord, Grid};
use challenges_common::MyIterTools;
use itertools::Itertools;
use std::ops::Deref;
use std::str::FromStr;

pub(crate) fn run(content: &str) -> anyhow::Result<usize> {
    let patterns: Patterns = content.parse()?;
    Ok(patterns.summarize())
}

struct Patterns {
    patterns: Vec<Pattern>,
}

impl Patterns {
    fn summarize(&self) -> usize {
        self.patterns
            .iter()
            .map(|pattern| pattern.summarize())
            .sum()
    }
}

impl FromStr for Patterns {
    type Err = CannotParseGrid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            patterns: s
                .lines()
                .split(|line| line.is_empty())
                .map(|s| s.join("\n").parse())
                .try_collect()?,
        })
    }
}

struct Pattern {
    grid: Grid<Place>,
}

impl Pattern {
    fn summarize(&self) -> usize {
        self.get_x_symmetry()
            .or(self.get_y_symmetry().map(|y_symm| y_symm * 100))
            .unwrap_or(0)
    }

    fn get_x_symmetry(&self) -> Option<usize> {
        'x_symm: for x_symm in 1..self.grid.width() {
            let max_x_diff = x_symm.min(self.grid.width() - x_symm);
            for x_diff in 1..=max_x_diff {
                let x_left = x_symm - x_diff;
                let x_right = x_symm + x_diff - 1;
                for y in 0..self.grid.height() {
                    if self.grid.at(&Coord { x: x_left, y }).unwrap().0
                        != self.grid.at(&Coord { x: x_right, y }).unwrap().0
                    {
                        continue 'x_symm;
                    }
                }
            }
            return Some(x_symm);
        }
        None
    }

    fn get_y_symmetry(&self) -> Option<usize> {
        'y_symm: for y_symm in 1..self.grid.height() {
            for y_diff in 1..y_symm.min(self.grid.height() - y_symm) + 1 {
                let y_left = y_symm - y_diff;
                let y_right = y_symm + y_diff - 1;
                for x in 0..self.grid.width() {
                    if self.grid.at(&Coord { x, y: y_left }).map(|place| place.0)
                        != self.grid.at(&Coord { x, y: y_right }).map(|place| place.0)
                    {
                        continue 'y_symm;
                    }
                }
            }
            return Some(y_symm);
        }
        None
    }
}

impl FromStr for Pattern {
    type Err = CannotParseGrid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { grid: s.parse()? })
    }
}

struct Place(bool);

impl Deref for Place {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<char> for Place {
    type Error = CannotParseElementFromChar;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        Ok(match char {
            '.' => Self(false),
            '#' => Self(true),
            _ => Err(Self::Error::from(char))?,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "13-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 405)
    }
}

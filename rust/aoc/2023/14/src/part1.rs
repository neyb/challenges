use crate::Load;
use challenges_common::graph::{
    CannotParseElementFromChar, CannotParseGrid, Coord, Direction, Grid,
};
use std::str::FromStr;

pub(crate) fn run(content: &String) -> anyhow::Result<Load> {
    let mut map: Map = content.parse()?;
    map.tilt_top()?;
    Ok(map.get_north_load())
}

struct Map {
    grid: Grid<Place>,
}

impl Map {
    fn tilt_top(&mut self) -> anyhow::Result<()> {
        for y in 0..self.grid.height() {
            for x in 0..self.grid.width() {
                let orig_coord = Coord { x, y };
                if matches!(self.grid.at(&orig_coord), Some(Place::RoundRock)) {
                    *self.grid.at_mut(&orig_coord).unwrap() = Place::Empty;

                    let mut target_coord = orig_coord;

                    while matches!(
                        target_coord
                            .try_at(Direction::Up)
                            .and_then(|coord| self.grid.at(&coord)),
                        Some(Place::Empty)
                    ) {
                        target_coord = target_coord.try_at(Direction::Up).unwrap();
                    }
                    *self.grid.at_mut(&target_coord).unwrap() = Place::RoundRock;
                }
            }
        }
        Ok(())
    }

    fn get_north_load(&self) -> usize {
        self.grid
            .coords()
            .filter(|coord| matches!(self.grid.at(&coord), Some(Place::RoundRock)))
            .map(|coord| self.grid.height() - coord.y)
            .sum()
    }
}

impl FromStr for Map {
    type Err = CannotParseGrid;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { grid: s.parse()? })
    }
}

enum Place {
    Empty,
    RoundRock,
    SquareRock,
}

impl TryFrom<char> for Place {
    type Error = CannotParseElementFromChar;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            '.' => Ok(Self::Empty),
            'O' => Ok(Self::RoundRock),
            '#' => Ok(Self::SquareRock),
            char => Err(CannotParseElementFromChar::from(char)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 136);
    }
}

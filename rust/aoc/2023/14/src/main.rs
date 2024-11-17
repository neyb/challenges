use challenges_common::graph::{
    CannotParseElementFromChar, CannotParseGrid, Coord, Direction, Grid,
};
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "14.txt"]);
    println!("part 1: {:?}", part1::run(&content).unwrap());
    println!("part 2: {:?}", part2::run(&content).unwrap());
}

type Load = usize;

mod part1;
mod part2;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Map {
    grid: Grid<Place>,
}

impl Map {
    fn spin_cycle(&mut self) {
        use Direction::*;
        self.tilt(Up);
        self.tilt(Left);
        self.tilt(Down);
        self.tilt(Right);
    }

    fn tilt(&mut self, direction: Direction) {
        for mut y in 0..self.grid.height() {
            if direction == Direction::Down {
                y = self.grid.height() - y - 1;
            }
            for mut x in 0..self.grid.width() {
                if direction == Direction::Right {
                    x = self.grid.width() - x - 1;
                }
                let orig_coord = Coord { x, y };
                if matches!(self.grid.at(&orig_coord), Some(Place::RoundRock)) {
                    *self.grid.at_mut(&orig_coord).unwrap() = Place::Empty;

                    let mut target_coord = orig_coord;

                    while matches!(
                        target_coord
                            .try_at(direction)
                            .and_then(|coord| self.grid.at(&coord)),
                        Some(Place::Empty)
                    ) {
                        target_coord = target_coord.try_at(direction).unwrap();
                    }
                    *self.grid.at_mut(&target_coord).unwrap() = Place::RoundRock;
                }
            }
        }
    }

    fn get_north_load(&self) -> usize {
        self.grid
            .coords()
            .filter(|coord| matches!(self.grid.at(coord), Some(Place::RoundRock)))
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

#[derive(Debug, Eq, PartialEq, Clone)]
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

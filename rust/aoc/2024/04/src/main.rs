use challenges_common::graph::{grid, CannotParseGrid, Coord, Grid};
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "04.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

struct Map {
    grid: Grid<char>,
}

impl FromStr for Map {
    type Err = CannotParseGrid;

    fn from_str(s: &str) -> anyhow::Result<Self, CannotParseGrid> {
        anyhow::Result::Ok(Self { grid: s.parse()? })
    }
}

enum Direction {
    U,
    R,
    D,
    L,
    UR,
    UL,
    DR,
    DL,
}

impl Direction {
    fn all() -> [Self; 8] {
        [
            Self::U,
            Self::R,
            Self::D,
            Self::L,
            Self::UR,
            Self::UL,
            Self::DR,
            Self::DL,
        ]
    }

    fn move_from_coord_dist(&self, coord: &Coord, dist: u8) -> Option<Coord> {
        let dirs = match self {
            Self::U => vec![grid::Direction::Up],
            Self::R => vec![grid::Direction::Right],
            Self::D => vec![grid::Direction::Down],
            Self::L => vec![grid::Direction::Left],
            Self::UR => vec![grid::Direction::Up, grid::Direction::Right],
            Self::UL => vec![grid::Direction::Up, grid::Direction::Left],
            Self::DR => vec![grid::Direction::Down, grid::Direction::Right],
            Self::DL => vec![grid::Direction::Down, grid::Direction::Left],
        };

        dirs.into_iter()
            .try_fold(*coord, |coord, dir| coord.try_at_dist(dir, dist))
    }
}

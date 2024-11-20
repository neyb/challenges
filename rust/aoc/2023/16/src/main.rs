use challenges_common::graph::{
    CannotParseElementFromChar, CannotParseGrid, Coord, Direction, Grid,
};
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "16.txt"]);
    println!("part 1: {}", part1::run(&content).unwrap());
    println!("part 2: {}", part2::run(&content).unwrap());
}

mod part1;
mod part2;

struct Contraption {
    grid: Grid<Place>,
}

impl Contraption {
    pub(crate) fn count_energized(&self, beam_head: BeamHead) -> usize {
        let mut explored_beam_heads = HashSet::new();
        let mut beam_heads = VecDeque::from([beam_head]);

        while let Some(
            beam_head @ BeamHead {
                coord: head_coord,
                direction: head_direction,
            },
        ) = beam_heads.pop_front()
        {
            if let Some(place) = self.grid.get(&head_coord) {
                if explored_beam_heads.insert(beam_head) {
                    for direction in place.next(head_direction) {
                        if let Some(next_coord) = head_coord.try_at(direction) {
                            if self.grid.get(&next_coord).is_some() {
                                beam_heads.push_back(BeamHead::new(next_coord, direction));
                            }
                        }
                    }
                }
            }
        }

        explored_beam_heads
            .iter()
            .map(|beam_head| beam_head.coord)
            .collect::<HashSet<_>>()
            .len()
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct BeamHead {
    coord: Coord,
    direction: Direction,
}

impl BeamHead {
    fn new(coord: Coord, direction: Direction) -> Self {
        Self { coord, direction }
    }
}

enum Place {
    Empty,
    Mirror(MirrorDirection),
    Splitter(SplitterDirection),
}

impl Place {
    fn next(&self, direction: Direction) -> Vec<Direction> {
        match self {
            Self::Empty => vec![direction],
            Self::Mirror(mirror_direction) => vec![mirror_direction.next(direction)],
            Self::Splitter(splitter_direction) => splitter_direction.next(direction),
        }
    }
}

enum MirrorDirection {
    UpRightToDownLeft,
    UpLeftToDownRight,
}

impl MirrorDirection {
    fn next(&self, direction: Direction) -> Direction {
        use Direction::*;

        match self {
            Self::UpRightToDownLeft => match direction {
                Up => Right,
                Right => Up,
                Left => Down,
                Down => Left,
            },
            Self::UpLeftToDownRight => match direction {
                Up => Left,
                Left => Up,
                Down => Right,
                Right => Down,
            },
        }
    }
}

enum SplitterDirection {
    Horizontal,
    Vertical,
}

impl SplitterDirection {
    fn next(&self, direction: Direction) -> Vec<Direction> {
        use Direction::*;
        use SplitterDirection::*;

        match (self, direction) {
            (Horizontal, Down | Up) => vec![Left, Right],
            (Vertical, Left | Right) => vec![Up, Down],
            _ => vec![direction],
        }
    }
}

impl FromStr for Contraption {
    type Err = CannotParseGrid;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        Ok(Self { grid: s.parse()? })
    }
}

impl TryFrom<char> for Place {
    type Error = CannotParseElementFromChar;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '/' => Self::Mirror(MirrorDirection::UpRightToDownLeft),
            '\\' => Self::Mirror(MirrorDirection::UpLeftToDownRight),
            '-' => Self::Splitter(SplitterDirection::Horizontal),
            '|' => Self::Splitter(SplitterDirection::Vertical),
            _ => Err(Self::Error::from(value))?,
        })
    }
}

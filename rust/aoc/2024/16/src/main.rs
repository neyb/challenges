use anyhow::{anyhow, Error};
use challenges_common::graph::{
    astar, grid, CannotParseElementFromChar, Coord, Direction, Path, Step, Turn,
};
use std::fmt::Display;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "16.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

type Grid = grid::Grid<Block>;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Reindeer {
    coord: Coord,
    orientation: Direction,
}

impl Reindeer {
    fn get_best_path(&self, map: &Map) -> Option<Path<Self, usize>> {
        astar(
            self.clone(),
            |reindeer| reindeer.next(map),
            |reindeer| reindeer.coord == map.end,
            |reindeer| reindeer.coord.manhattan_dist_to(&map.end),
        )
    }

    fn next(&self, map: &Map) -> Vec<Step<Self, usize>> {
        let mut result = Vec::with_capacity(3);

        if let Some(coord_ahead) = self.coord.try_at(self.orientation) {
            if let Some(block) = map.grid.get(&coord_ahead) {
                if block == &Block::Empty {
                    result.push(Step {
                        to: Reindeer {
                            coord: coord_ahead,
                            orientation: self.orientation,
                        },
                        additional_cost: 1,
                    })
                }
            }
        }

        result.push(Step {
            to: Reindeer {
                coord: self.coord,
                orientation: self.orientation.turn(Turn::Right),
            },
            additional_cost: 1000,
        });

        result.push(Step {
            to: Reindeer {
                coord: self.coord,
                orientation: self.orientation.turn(Turn::Left),
            },
            additional_cost: 1000,
        });

        result
    }
}

struct Map {
    grid: Grid,
    start: Coord,
    end: Coord,
}

impl Map {
    fn reindeer_start(&self) -> Reindeer {
        Reindeer {
            coord: self.start,
            orientation: Direction::Right,
        }
    }
}

#[derive(PartialEq)]
enum Block {
    Empty,
    Wall,
}

#[derive(PartialEq)]
enum ParsedBlock {
    Empty,
    Wall,
    Start,
    End,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Wall => '#',
        };
        write!(f, "{}", c)
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let grid: grid::Grid<ParsedBlock> = s.parse()?;

        anyhow::Ok(Self {
            start: grid
                .find(|block| block == &ParsedBlock::Start)
                .ok_or_else(|| anyhow!("no start found"))?,
            end: grid
                .find(|block| block == &ParsedBlock::End)
                .ok_or_else(|| anyhow!("no end found"))?,
            grid: grid.map(|block| block.into()),
        })
    }
}

impl From<&ParsedBlock> for Block {
    fn from(value: &ParsedBlock) -> Self {
        match value {
            ParsedBlock::Wall => Self::Wall,
            _ => Self::Empty,
        }
    }
}

impl TryFrom<char> for ParsedBlock {
    type Error = CannotParseElementFromChar;

    fn try_from(value: char) -> anyhow::Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => return Err(value.into()),
        })
    }
}

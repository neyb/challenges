use anyhow::{anyhow, Error};
use challenges_common::graph::{astar, grid, CannotParseElementFromChar, Step};
use std::ops::Deref;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "20.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

type Unit = usize;
type Grid = grid::Grid<Block, Unit>;
type Coord = grid::Coord<Unit>;
type Path = challenges_common::graph::Path<Coord, Unit>;

struct Map {
    grid: Grid,
    start: grid::Coord<Unit>,
    end: grid::Coord<Unit>,
}

impl Map {
    fn get_path(&self) -> Option<Path> {
        astar(
            self.start,
            |coord| {
                coord
                    .neighbours(false)
                    .filter(|coord| self.get(coord) == Some(&Block::Empty))
                    .map(|coord| Step {
                        to: coord,
                        additional_cost: 1,
                    })
            },
            |coord| *coord == self.end,
            |coord| coord.manhattan_dist_to(&self.end),
        )
    }
}

impl Deref for Map {
    type Target = Grid;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

struct PathAnalyzer {
    path: Path,
}

impl PathAnalyzer {
    fn get_shortcuts(&self, shortcut_max_len: Unit, min_win: Unit) -> Vec<Shortcut> {
        let mut shortcuts = Vec::new();

        for (i, start) in self.path.nodes.iter().enumerate() {
            for (dest_time, end) in self.path.nodes.iter().skip(i).enumerate().skip(min_win) {
                let shortcut_length = start.manhattan_dist_to(end);
                let win_time = dest_time - shortcut_length;
                if shortcut_length <= shortcut_max_len && win_time >= min_win {
                    shortcuts.push(Shortcut {
                        #[cfg(test)]
                        win_time,
                    });
                }
            }
        }

        shortcuts
    }
}

struct Shortcut {
    #[cfg(test)]
    win_time: Unit,
}

#[derive(Eq, PartialEq)]
enum Block {
    Empty,
    Wall,
}

impl From<&ParsedBlock> for Block {
    fn from(parsed: &ParsedBlock) -> Self {
        match parsed {
            ParsedBlock::Empty => Block::Empty,
            ParsedBlock::Wall => Block::Wall,
            ParsedBlock::Start => Block::Empty,
            ParsedBlock::End => Block::Empty,
        }
    }
}

enum ParsedBlock {
    Empty,
    Wall,
    Start,
    End,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let grid: grid::Grid<ParsedBlock> = s.parse()?;
        let start = grid
            .find(|block| matches!(block, ParsedBlock::Start))
            .ok_or_else(|| anyhow!("No start block"))?;
        let end = grid
            .find(|block| matches!(block, ParsedBlock::End))
            .ok_or_else(|| anyhow!("No end block"))?;

        anyhow::Ok(Self {
            grid: grid.map(|parsed| parsed.into()),
            start,
            end,
        })
    }
}

impl TryFrom<char> for ParsedBlock {
    type Error = CannotParseElementFromChar;

    fn try_from(value: char) -> anyhow::Result<Self, Self::Error> {
        use ParsedBlock::*;

        Ok(match value {
            '.' => Empty,
            '#' => Wall,
            'S' => Start,
            'E' => End,
            _ => Err(CannotParseElementFromChar::from(value))?,
        })
    }
}

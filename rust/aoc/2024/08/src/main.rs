use challenges_common::graph;
use challenges_common::graph::CannotParseGrid;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "08.txt"]);
    println!("part1: {:?}", run(&content, part1::antinodes_for_pair));
    println!("part2: {:?}", run(&content, part2::antinodes_for_pair));
}

mod part1;
mod part2;

type Coord = graph::Coord<i32>;

fn run(
    content: &str,
    antinodes_for_pair: impl Fn(&Coord, &Coord, &Map) -> Vec<Coord>,
) -> anyhow::Result<usize> {
    let map: Map = content.parse()?;
    let antinodes = map.antinodes(antinodes_for_pair);
    anyhow::Ok(antinodes.len())
}

struct Map {
    grid: graph::Grid<Block, i32>,
}

impl Map {
    fn by_frequency(&self) -> HashMap<Block, Vec<Coord>> {
        self.grid
            .coords()
            .filter(|coord| self.grid.get(coord).unwrap().0.is_some())
            .into_group_map_by(|coord| *self.grid.get(coord).unwrap())
    }

    fn antinodes(
        &self,
        antinodes_for_pair: impl Fn(&Coord, &Coord, &Map) -> Vec<Coord>,
    ) -> HashSet<Coord> {
        self.by_frequency()
            .iter()
            .flat_map(|(_frequency, coords)| {
                coords
                    .iter()
                    .combinations(2)
                    .flat_map(|coords| antinodes_for_pair(coords[0], coords[1], self))
            })
            .collect()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Block(Option<Antenna>);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Antenna(char);

impl FromStr for Map {
    type Err = CannotParseGrid<Infallible>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { grid: s.parse()? })
    }
}

impl TryFrom<char> for Block {
    type Error = Infallible;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Self(if value == '.' {
            None
        } else {
            Some(Antenna(value))
        }))
    }
}

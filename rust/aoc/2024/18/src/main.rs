use anyhow::{anyhow, Error};
use challenges_common::graph;
use challenges_common::graph::{astar, Path, Step};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "18.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

type Coord = graph::Coord<i16>;
struct Map {
    falled: HashSet<Coord>,
    falling: Vec<Coord>,
    bot_right: Coord,
}

impl Map {
    fn sim(&mut self, size: usize) -> Option<Coord> {
        let res = self.falling.get(size - 1).copied();
        self.falled.extend(self.falling.drain(0..size));
        res
    }

    fn path(&self) -> Option<Path<Coord, u32>> {
        astar(
            Coord { x: 0, y: 0 },
            |coord| {
                coord
                    .neighbours(false)
                    .filter(|coord| self.contains(coord) && !self.falled.contains(coord))
                    .map(|coord| Step {
                        to: coord,
                        additional_cost: 1_u32,
                    })
            },
            |coord| coord == &self.bot_right,
            |coord| coord.manhattan_dist_to(&self.bot_right) as u32,
        )
    }

    fn contains(&self, coord: &Coord) -> bool {
        coord.x >= 0 && coord.y >= 0 && coord.x <= self.bot_right.x && coord.y <= self.bot_right.y
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let falling = s
            .lines()
            .map(|line| {
                let (x, y) = line
                    .split_once(",")
                    .ok_or_else(|| anyhow!("no comma found"))?;
                anyhow::Ok(Coord {
                    x: x.parse()?,
                    y: y.parse()?,
                })
            })
            .try_collect()?;
        let bot_right = Coord { x: 70, y: 70 };
        anyhow::Ok(Self {
            falled: HashSet::new(),
            falling,
            bot_right,
        })
    }
}

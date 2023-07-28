use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{anyhow, Result};

mod part1;
mod part2;

fn main() {
    let cubes = parse_cubes(&["aoc", "2022", "18.txt"]).unwrap();

    println!("part1 : {}", part1::count_sized(&cubes));
    println!("part2 : {}", part2::count_exterior_sized(&cubes));
}

fn parse_cubes(path: &[&str]) -> Result<Vec<Cube>> {
    challenges_common::get_input_lines(&path)
        .map(|line| line.parse::<Cube>())
        .collect::<Result<Vec<_>>>()
}

type Unit = i8;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Cube {
    x: Unit,
    y: Unit,
    z: Unit,
}

impl FromStr for Cube {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let splitted = s.split(",").collect::<Vec<_>>();
        let get = |i: usize| -> Result<Unit> {
            let part = splitted
                .get(i)
                .ok_or_else(|| anyhow!("cannot parse {}, cannot get x", s))?;
            Ok(Unit::from_str(part)?)
        };

        Ok(Self {
            x: get(0)?,
            y: get(1)?,
            z: get(2)?,
        })
    }
}

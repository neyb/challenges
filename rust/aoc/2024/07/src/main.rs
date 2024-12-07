use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "07.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

type Res = usize;

struct Equation {
    result: Res,
    operands: Vec<Res>,
}

impl FromStr for Equation {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (result, operands) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("cannot parse line"))?;

        anyhow::Ok(Self {
            result: result.parse()?,
            operands: operands.split(" ").map(|op| op.parse()).try_collect()?,
        })
    }
}

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "13.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

use anyhow::*;
use challenges_common::graph::grid;
use challenges_common::MyIterTools;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::str::FromStr;

type Unit = i64;

type Vec2 = grid::Vec2<Unit>;

struct Machines(Vec<Machine>);

impl Machines {
    fn min_cost(&self) -> Unit {
        self.0.iter().filter_map(|machine| machine.min_cost()).sum()
    }
}

struct Machine {
    a: Vec2,
    b: Vec2,
    price: Vec2,
}

impl Machine {
    fn min_cost(&self) -> Option<Unit> {
        self.guess_presses()
            .map(|(a_presses, b_presses)| a_presses * 3 + b_presses)
    }

    fn guess_presses(&self) -> Option<(Unit, Unit)> {
        let x = self.price.x;
        let y = self.price.y;
        let ax = self.a.x;
        let ay = self.a.y;
        let bx = self.b.x;
        let by = self.b.y;

        let b_div = ay * bx - by * ax;
        if b_div == 0 {
            // rarely happens...
            panic!("cannot divide by 0");
        }

        let b_presses = (ay * x - ax * y) / b_div;
        let a_presses = (x - bx * b_presses) / ax;

        if a_presses >= 0
            && b_presses >= 0
            && (a_presses * ax + b_presses * bx) == x
            && (a_presses * ay + b_presses * by) == y
        {
            Some((a_presses as Unit, b_presses as Unit))
        } else {
            None
        }
    }
}

impl FromStr for Machines {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .split(|line| line.is_empty())
                .map(|lines| lines.join("\n").parse())
                .try_collect()?,
        ))
    }
}

impl FromStr for Machine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        static REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"X[+=](?<x>\d+), Y[+=](?<y>\d+)").unwrap());

        let (a_button, b_button, price) = s
            .lines()
            .collect_tuple()
            .ok_or_else(|| anyhow!("cannot parse machine"))?;

        let parse = |s| {
            let captures = REGEX
                .captures(s)
                .ok_or_else(|| anyhow!("cannot parse machine, capture does not match: {s}"))?;

            Ok(Vec2 {
                x: captures["x"].parse::<Unit>()?,
                y: captures["y"].parse::<Unit>()?,
            })
        };

        Ok(Self {
            a: parse(a_button)?,
            b: parse(b_button)?,
            price: parse(price)?,
        })
    }
}

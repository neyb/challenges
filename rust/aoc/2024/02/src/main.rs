fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "02.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

use anyhow::*;
use itertools::Itertools;
use std::str::FromStr;

type Res = usize;
pub(crate) fn run(content: &String) -> Result<Res> {
    let lines: Vec<_> = content
        .lines()
        .map(|line| line.parse::<Line>())
        .try_collect()?;

    Ok(lines.iter().filter(|line| line.is_safe()).count())
}

#[derive(Clone)]
struct Line {
    numbers: Vec<isize>,
}

impl Line {
    fn is_safe(&self) -> bool {
        let mut sign = None;
        self.numbers.windows(2).map(|w| w[1] - w[0]).all(|diff| {
            diff.abs() <= 3
                && diff.abs() > 0
                && match sign {
                    None => {
                        sign = if diff > 0 {
                            Some(Sign::Positive)
                        } else {
                            Some(Sign::Negative)
                        };
                        true
                    }
                    Some(Sign::Positive) => diff > 0,
                    Some(Sign::Negative) => diff < 0,
                }
        })
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            numbers: s.split(" ").map(|s| s.parse()).try_collect()?,
        })
    }
}

enum Sign {
    Positive,
    Negative,
}

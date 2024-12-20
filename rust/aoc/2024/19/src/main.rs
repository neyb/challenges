use anyhow::{anyhow, bail, Error};
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Deref;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "19.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

struct Onsen {
    towels: Towels,
    designs: Vec<Pattern>,
}

struct Towels {
    towels: HashSet<Pattern>,
    max_len: usize,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Pattern {
    colors: Vec<Color>,
}

impl Pattern {
    fn start(&self, size: usize) -> Pattern {
        Pattern {
            colors: self[..size].to_vec(),
        }
    }

    fn skip(&self, n: usize) -> Self {
        Self {
            colors: self[n..].to_vec(),
        }
    }
}

impl Deref for Pattern {
    type Target = [Color];

    fn deref(&self) -> &Self::Target {
        &self.colors
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Color {
    White,
    Black,
    Blue,
    Red,
    Green,
}

impl FromStr for Onsen {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut lines = s.lines();
        let towels: HashSet<Pattern> = lines
            .next()
            .ok_or_else(|| anyhow!("cannot get towels"))?
            .split(", ")
            .map(Pattern::from_str)
            .try_collect()?;
        let designs = lines.skip(1).map(Pattern::from_str).try_collect()?;

        let max_len = towels.iter().map(|towel| towel.colors.len()).max().unwrap();
        anyhow::Ok(Self {
            towels: Towels { max_len, towels },
            designs,
        })
    }
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        anyhow::Ok(Self {
            colors: s.chars().map(Color::try_from).try_collect()?,
        })
    }
}

impl TryFrom<char> for Color {
    type Error = Error;

    fn try_from(value: char) -> anyhow::Result<Self> {
        anyhow::Ok(match value {
            'w' => Self::White,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            'u' => Self::Blue,
            _ => bail!("invalid color: {value}"),
        })
    }
}

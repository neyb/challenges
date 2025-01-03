use anyhow::*;
use itertools::Itertools;
use std::num::Wrapping;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "22.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

fn parse(content: &str) -> Result<Vec<Secret>> {
    content.lines().map(|line| line.parse()).try_collect()
}

struct Secret(Wrapping<i32>);

impl Secret {
    fn new(s: i32) -> Self {
        Self(Wrapping(s))
    }

    fn next(&mut self) {
        self.0 ^= self.0 << 6;
        self.prune();
        self.0 ^= self.0 >> 5;
        self.prune();
        self.0 ^= self.0 << 11;
        self.prune();
    }

    fn prune(&mut self) {
        self.0 &= (1 << 24) - 1;
    }
}

impl From<&Secret> for i32 {
    fn from(s: &Secret) -> Self {
        s.0 .0
    }
}

impl FromStr for Secret {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self::new(s.parse()?))
    }
}

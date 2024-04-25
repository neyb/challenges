use std::ops::Range;
use std::result;
use std::str::FromStr;

use anyhow::{anyhow, Result};

use challenges_common::MyIterTools;
use itertools::Itertools;
use lazy_regex::{lazy_regex, regex};

fn main() {
    println!("Hello, world!");
}

struct Almanac {
    seeds: Category,
    mappings: Vec<Mapping>,
}

struct Category {
    numbers: Vec<u32>,
}

struct Mapping {
    transformations: Vec<Transformation>,
}

struct Transformation {
    source_range: Range<u32>,
    transformation: u32,
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut group_iterator = s.lines().split(|line| line.is_empty());

        let seeds = group_iterator
            .next()
            .ok_or_else(|| anyhow!("cannot get seeds group"))?
            .get(0)
            .ok_or_else(|| anyhow!("cannot get seeds line"))?
            .parse()?            ;

        let mappings = group_iterator
            .map(|lines| lines.iter().skip(1).map(|line| line.parse()))
            .collect();

        Ok(Self{seeds,mappings})
    }
}

impl FromStr for Mapping{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, [source,_,_]) = lazy_regex::regex_captures!(r"(\d+) (\d+) (\d+)", s).ok_or_else(||anyhow!("cannot extract Mapping from {}", s))?;

        let transformations =

        Ok(Self{transformations})
    }
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let regex = lazy_regex::regex!(r"\d+");
        let numbers = regex
            .captures_iter(s)
            .map(|capt| {
                let (number_str, []) = capt.extract();
                number_str.parse::<u32>().into()
            })
            .collect::<Result<_>>()?;

        Ok(Self { numbers })
    }
}

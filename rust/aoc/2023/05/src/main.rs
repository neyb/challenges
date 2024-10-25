use std::ops::Range;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use challenges_common::MyIterTools;
use itertools::Itertools;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "05.txt"]);
    println!("part 1: {}", part1(&content));
}

type Position = u64;

fn part1(content: &str)-> Position {
    let almanac = content.parse().unwrap();
    get_min_location(&almanac)
}

fn get_min_location(almanac: &Almanac) -> Position {
    almanac.locations().min().unwrap()
}

struct Almanac {
    seeds: Category,
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn locations(&self) -> impl Iterator<Item =Position> + '_ {
        self.seeds.numbers.iter().map(move |seed| {
            self.mappings
                .iter()
                .fold(*seed, |value, mapping| mapping.map(value))
        })
    }
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
            .parse()?;

        let mappings = group_iterator.map(|lines| lines.try_into()).try_collect()?;

        Ok(Self { seeds, mappings })
    }
}

#[derive(Debug, PartialEq)]
struct Category {
    numbers: Vec<Position>,
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let regex = lazy_regex::regex!(r"\d+");
        let numbers = regex
            .captures_iter(s)
            .map(|capt| {
                let (number_str, []) = capt.extract();
                number_str.parse::<Position>()
            })
            .try_collect()?;

        Ok(Self { numbers })
    }
}

struct Mapping {
    transformations: Vec<Transformation>,
}

impl Mapping {
    fn map(&self, value: Position) -> Position {
        match self
            .transformations
            .iter()
            .find_or_first(|transformation| transformation.concerns(value))
        {
            Some(transformation) => transformation.map(value),
            None => value,
        }
    }
}

impl<S> TryFrom<Vec<S>> for Mapping
where
    S: AsRef<str>,
{
    type Error = anyhow::Error;

    fn try_from(lines: Vec<S>) -> Result<Self> {
        let transformations = lines
            .iter()
            .skip(1)
            .map(|line| line.as_ref().parse::<Transformation>())
            .try_collect()?;

        Ok(Self { transformations })
    }
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let transformations = s.lines().skip(1).map(|line| line.parse()).try_collect()?;
        Ok(Self { transformations })
    }
}

#[derive(Debug, PartialEq)]
struct Transformation {
    source_range: Range<Position>,
    transformation: i64,
}

impl FromStr for Transformation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let elements = s.split(' ').collect_vec();
        let [destination_start, source_start, source_range] = elements.as_slice() else {
            anyhow::bail!("cannot parse transformation {}", s)
        };

        let source_start: u64 = source_start.parse()?;
        let source_range: u64 = source_range.parse()?;
        let destination_start: u64 = destination_start.parse()?;

        Ok(Self::new(
            source_start..source_start + source_range,
            destination_start as i64 - source_start as i64,
        ))
    }
}

impl Transformation {
    fn new(source_range: Range<Position>, transformation: i64) -> Self {
        Self {
            source_range,
            transformation,
        }
    }

    fn concerns(&self, value: Position) -> bool {
        self.source_range.contains(&value)
    }

    fn map(&self, value: Position) -> Position {
        if self.source_range.contains(&value) {
            (value as i64 + self.transformation) as Position
        } else {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_a_category() {
        let category = "seeds: 79 14 55 13";
        let category = Category::from_str(category).unwrap();
        assert_eq!(
            category,
            Category {
                numbers: vec![79, 14, 55, 13]
            }
        );
    }

    #[test]
    fn parsing_a_transformation() {
        let transformation = "50 98 2";
        let transformation = Transformation::from_str(transformation).unwrap();
        assert_eq!(transformation, Transformation::new(98..100, -48));
    }

    #[test]
    fn part1_given_test() {
        let almanac = challenges_common::get_input_content(&["aoc", "2023", "05-test.txt"]);
        let almanac = Almanac::from_str(&almanac).unwrap();
        assert_eq!(almanac.locations().min(), Some(35));
    }
}

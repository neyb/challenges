use crate::common::*;
use anyhow::anyhow;
use challenges_common::MyIterTools;
use itertools::Itertools;
use std::str::FromStr;

pub fn run(content: &str) -> Position {
    let almanac: Almanac = content.parse().unwrap();
    almanac.locations().min().unwrap()
}

pub struct Almanac {
    seeds: Category,
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn locations(&self) -> impl Iterator<Item = Position> + '_ {
        self.seeds.numbers.iter().map(move |seed| {
            self.mappings
                .iter()
                .fold(*seed, |value, mapping| mapping.map_position(value))
        })
    }
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut group_iterator = s.lines().split(|line| line.is_empty());

        let seeds = group_iterator
            .next()
            .ok_or_else(|| anyhow!("cannot get seeds group"))?
            .first()
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

    fn from_str(s: &str) -> anyhow::Result<Self> {
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
    fn part1_given_test() {
        let almanac = challenges_common::get_input_content(&["aoc", "2023", "05-test.txt"]);
        let almanac = Almanac::from_str(&almanac).unwrap();
        assert_eq!(almanac.locations().min(), Some(35));
    }
}

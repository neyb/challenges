use crate::common::*;
use anyhow::{anyhow, Result};
use challenges_common::ranges::{discontinuous, Ranges as OrigRanges};
use challenges_common::MyIterTools;
use itertools::Itertools;
use std::str::FromStr;

type Ranges = OrigRanges<Range>;
type Range = discontinuous::Range<Position>;

pub fn run(content: &str) -> Position {
    let almanac: Almanac = content.parse().unwrap();
    almanac
        .locations()
        .ranges
        .ranges
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

struct Almanac {
    seeds: Category,
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn locations(&self) -> Category {
        self.mappings
            .iter()
            .fold(self.seeds.clone(), |category, mapping| {
                category.apply_mapping(mapping)
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
            .first()
            .ok_or_else(|| anyhow!("cannot get seeds line"))?
            .parse()?;

        let mappings = group_iterator.map(|lines| lines.try_into()).try_collect()?;

        Ok(Self { seeds, mappings })
    }
}

#[derive(Clone)]
pub struct Category {
    ranges: Ranges,
}

impl Category {
    fn new(ranges: Ranges) -> Self {
        Self { ranges }
    }

    fn apply_mapping(&self, mapping: &Mapping) -> Category {
        let mut orig_ranges = self.ranges.clone();
        let mut target_ranges = Ranges::empty();

        for transformation in &mapping.transformations {
            let removed = orig_ranges.remove_range(&Range::from(&transformation.source_range));
            let to_add = removed.map(|range| {
                Range::new_inclusive(
                    (range.start as PositionTransformation + transformation.transformation)
                        as Position,
                    (range.end as PositionTransformation + transformation.transformation)
                        as Position,
                )
                .unwrap()
            });
            target_ranges.merge(to_add)
        }
        target_ranges.merge(orig_ranges);
        Category::new(target_ranges)
    }
}

impl FromStr for Category {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let regex = lazy_regex::regex!(r"(\d+) (\d+)");
        let ranges = regex
            .captures_iter(s)
            .map(|capt| -> Result<Range> {
                let (_, [start, length]) = capt.extract();
                let start = start.parse()?;
                let length = length.parse()?;
                Range::with_length(start, length).ok_or_else(|| anyhow!(""))
            })
            .try_collect()?;

        Ok(Self {
            ranges: Ranges::new(ranges),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_given_test() {
        let almanac = challenges_common::get_input_content(&["aoc", "2023", "05-test.txt"]);
        let result = run(&almanac);
        assert_eq!(result, 46);
    }
}

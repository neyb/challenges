use crate::{Onsen, Pattern, Towels};
use anyhow::*;
use std::collections::HashMap;

type Res = u64;
pub(crate) fn run(content: &str) -> Result<Res> {
    let onsen: Onsen = content.parse()?;
    Ok(onsen.sum_possible_ways())
}

trait Part2Onsen {
    fn sum_possible_ways(&self) -> Res;
}

impl Part2Onsen for Onsen {
    fn sum_possible_ways(&self) -> Res {
        let mut cache = HashMap::new();
        self.designs
            .iter()
            .map(|design| self.towels.count_ways(design, &mut cache))
            .sum()
    }
}

trait Part2Towels {
    fn count_ways(&self, design: &Pattern, cache: &mut HashMap<Pattern, Res>) -> Res;
}

impl Part2Towels for Towels {
    fn count_ways(&self, design: &Pattern, cache: &mut HashMap<Pattern, Res>) -> Res {
        if design.is_empty() {
            return 1;
        }

        match cache.get(design) {
            Some(&result) => result,
            None => {
                let sum = (1..=self.max_len.min(design.len()))
                    .filter(|size| self.towels.contains(&design.start(*size)))
                    .map(|size| self.count_ways(&design.skip(size), cache))
                    .sum();

                cache.insert(design.clone(), sum);
                sum
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "19-test.txt"]);
        assert_eq!(run(&content).unwrap(), 16);
    }
}

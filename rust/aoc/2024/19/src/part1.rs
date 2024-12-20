use crate::{Onsen, Pattern, Towels};
use anyhow::*;
use std::collections::HashMap;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let onsen: Onsen = content.parse()?;
    Ok(onsen.count_possible_designs())
}

trait Part1Onsen {
    fn count_possible_designs(&self) -> usize;
}

impl Part1Onsen for Onsen {
    fn count_possible_designs(&self) -> usize {
        let mut cache = HashMap::new();
        self.designs
            .iter()
            .filter(|design| self.towels.can_create(design, &mut cache))
            .count()
    }
}

trait Part1Towels {
    fn can_create(&self, design: &Pattern, cache: &mut HashMap<Pattern, bool>) -> bool;
}

impl Part1Towels for Towels {
    fn can_create(&self, design: &Pattern, cache: &mut HashMap<Pattern, bool>) -> bool {
        if design.is_empty() {
            return true;
        }

        match cache.get(design) {
            Some(&result) => result,
            None => {
                let res = (1..=self.max_len.min(design.len())).rev().any(|size| {
                    self.towels.contains(&design.start(size))
                        && self.can_create(&design.skip(size), cache)
                });
                cache.insert(design.clone(), res);
                res
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
        assert_eq!(run(&content).unwrap(), 6);
    }
}

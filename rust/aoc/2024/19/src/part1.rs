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
        if let Some(&result) = cache.get(design) {
            return result;
        }
        if design.is_empty() {
            return true;
        }

        for size in (1..=self.max_len.min(design.len())).rev() {
            let towel = design.start(size);
            if self.towels.contains(&towel) && self.can_create(&design.skip(size), cache) {
                cache.insert(design.clone(), true);
                return true;
            }
        }

        cache.insert(design.clone(), false);
        false
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

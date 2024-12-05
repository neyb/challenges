use crate::{PageOrderingRules, Updates};
use anyhow::*;
use challenges_common::MyIterTools;
use itertools::Itertools;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let (rules, updates) = content
        .lines()
        .split(|line| line.is_empty())
        .collect_tuple()
        .ok_or_else(|| anyhow!("Invalid input"))?;
    let rules: PageOrderingRules = rules.join("\n").parse()?;
    let updates: Updates = updates.join("\n").parse()?;

    Ok(updates
        .updates
        .iter()
        .filter(|update| rules.satisfies(update))
        .map(|update| update.middle_page())
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "05-test.txt"]);
        assert_eq!(run(&content).unwrap(), 143);
    }
}

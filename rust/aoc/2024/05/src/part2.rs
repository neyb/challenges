use crate::{Page, PageOrderingRule, PageOrderingRules, Update, Updates};
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
    let mut updates: Updates = updates.join("\n").parse()?;

    Ok(updates
        .updates
        .iter_mut()
        .filter(|update| !rules.satisfies(update))
        .update(|update| update.sort(&rules))
        .map(|update| update.middle_page())
        .sum())
}

trait Part2Rules {
    fn get_rule(&self, page1: Page, page2: Page) -> Option<&PageOrderingRule>;
}

impl Part2Rules for PageOrderingRules {
    fn get_rule(&self, page1: Page, page2: Page) -> Option<&PageOrderingRule> {
        self.rules.iter().find(|rule| {
            (rule.before == page1 && rule.after == page2)
                || (rule.before == page2 && rule.after == page1)
        })
    }
}

trait SortByRules {
    fn sort(&mut self, rules: &PageOrderingRules);
}

impl SortByRules for Updates {
    fn sort(&mut self, rules: &PageOrderingRules) {
        for update in self.updates.iter_mut() {
            update.sort(rules)
        }
    }
}

impl SortByRules for Update {
    fn sort(&mut self, rules: &PageOrderingRules) {
        self.pages.sort_by(|a, b| match rules.get_rule(*a, *b) {
            None => std::cmp::Ordering::Equal,
            Some(rule) => {
                if rule.before == *a && rule.after == *b {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "05-test.txt"]);
        assert_eq!(run(&content).unwrap(), 123);
    }
}

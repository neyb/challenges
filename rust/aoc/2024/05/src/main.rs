use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "05.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

type Page = usize;
struct PageOrderingRules {
    rules: Vec<PageOrderingRule>,
}

impl PageOrderingRules {
    fn satisfies(&self, update: &Update) -> bool {
        self.rules.iter().all(|rule| rule.satisfies(update))
    }
}

struct PageOrderingRule {
    before: Page,
    after: Page,
}

impl PageOrderingRule {
    fn satisfies(&self, update: &Update) -> bool {
        let before = update.pages.iter().position(|&page| page == self.before);
        let after = update.pages.iter().position(|&page| page == self.after);
        match (before, after) {
            (Some(before), Some(after)) => before < after,
            _ => true,
        }
    }
}

struct Updates {
    updates: Vec<Update>,
}

struct Update {
    pages: Vec<Page>,
}

impl Update {
    fn middle_page(&self) -> Page {
        let index = self.pages.len() / 2;
        self.pages[index]
    }
}

impl FromStr for Updates {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        anyhow::Ok(Self {
            updates: s.lines().map(Update::from_str).try_collect()?,
        })
    }
}

impl FromStr for PageOrderingRule {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (before, after) = s
            .split_once("|")
            .ok_or_else(|| anyhow!("Invalid rule {s}"))?;
        anyhow::Ok(Self {
            before: before.parse()?,
            after: after.parse()?,
        })
    }
}

impl FromStr for PageOrderingRules {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        anyhow::Ok(Self {
            rules: s.lines().map(PageOrderingRule::from_str).try_collect()?,
        })
    }
}

impl FromStr for Update {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        anyhow::Ok(Self {
            pages: s.split(",").map(Page::from_str).try_collect()?,
        })
    }
}

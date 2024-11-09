fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "09.txt"]);
    println!("part1: {}", run_part1(&content).unwrap());
    println!("part2: {}", run_part2(&content).unwrap());
}

fn run_part1(content: &str) -> Result<Value> {
    let lines: Lines = content.parse()?;
    lines.nexts_sums()
}

fn run_part2(content: &str) -> Result<Value> {
    let lines: Lines = content.parse()?;
    lines.prev_sums()
}

use anyhow::Result;
use itertools::Itertools;
use std::str::FromStr;

type Value = i32;

struct Lines {
    lines: Vec<Line>,
}

impl Lines {
    fn new(lines: Vec<Line>) -> Self {
        Self { lines }
    }

    fn nexts_sums(&self) -> Result<Value> {
        self.lines
            .iter()
            .map(|l| l.next_value())
            .try_fold(0, |acc, v| Ok(acc + v?))
    }

    fn prev_sums(&self) -> Result<Value> {
        self.lines
            .iter()
            .map(|l| l.prev_value())
            .try_fold(0, |acc, v| Ok(acc + v?))
    }
}

impl FromStr for Lines {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Lines::new(s.lines().map(|l| l.parse()).try_collect()?))
    }
}

struct Line {
    values: Vec<Value>,
}

impl Line {
    fn new(values: Vec<Value>) -> Self {
        Self { values }
    }

    fn next_value(&self) -> Result<Value> {
        if self.next_is_null() {
            Ok(0)
        } else {
            let derive = self.derive();
            let last = *self
                .values
                .last()
                .ok_or_else(|| anyhow::anyhow!("No value"))?;
            let derive_next = derive.next_value()?;
            Ok(derive_next + last)
        }
    }

    fn prev_value(&self) -> Result<Value> {
        if self.next_is_null() {
            Ok(0)
        } else {
            let derive = self.derive();
            let first = *self
                .values
                .first()
                .ok_or_else(|| anyhow::anyhow!("No value"))?;
            let derive_next = derive.prev_value()?;
            Ok(first - derive_next)
        }
    }

    fn derive(&self) -> Self {
        let (values, _last_value) =
            self.values
                .iter()
                .fold(
                    (Vec::new(), None),
                    |(mut values, previous), curr_value| match previous {
                        None => (values, Some(curr_value)),
                        Some(prev_value) => {
                            values.push(curr_value - prev_value);
                            (values, Some(curr_value))
                        }
                    },
                );
        Self::new(values)
    }

    fn next_is_null(&self) -> bool {
        self.values.iter().all(|v| *v == 0)
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Line::new(
            s.split(' ').into_iter().map(|v| v.parse()).try_collect()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::run_part1;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "09-test.txt"]);
        assert_eq!(run_part1(&content).unwrap(), 114);
    }
}

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "12.txt"]);
    println!("part 1: {:?}", part1::run(&content).unwrap());
    println!("part 2: {:?}", part2::run(&content).unwrap());
}

mod part1;
mod part2;

use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

type Len = u32;

struct Line {
    springs: Vec<Spring>,
    groups: Vec<Len>,
}

type Memo = HashMap<(usize, usize), Len>;
impl Line {
    fn nb_arrangement(&self) -> Len {
        let mut memo = HashMap::new();
        self.nb_arrangements_rec(0, 0, &mut memo)
    }

    fn duplicate(&mut self, times: usize) -> Self {
        let mut springs = Vec::with_capacity(self.springs.len() * times);
        let mut groups = Vec::with_capacity(self.groups.len() * times);

        for i in 0..times {
            if i > 0 {
                springs.push(Spring::Unknown);
            }
            springs.extend(self.springs.clone());
            groups.extend(self.groups.clone());
        }

        Self { springs, groups }
    }

    fn nb_arrangements_rec(&self, from_spring: usize, from_record: usize, memo: &mut Memo) -> Len {
        if let Some(&nb_arrangements) = memo.get(&(from_spring, from_record)) {
            return nb_arrangements;
        }

        if not_enough_springs(self, from_spring, from_record) {
            memo.insert((from_spring, from_record), 0);
            return 0;
        }

        if not_enough_remaining_potential_damaged(self, from_spring, from_record) {
            memo.insert((from_spring, from_record), 0);
            return 0;
        }

        if has_too_many_remaining_damaged(self, from_spring, from_record) {
            memo.insert((from_spring, from_record), 0);
            return 0;
        }

        return match self.springs.get(from_spring) {
            None => {
                if from_record >= self.groups.len() {
                    memo.insert((from_spring, from_record), 1);
                    1
                } else {
                    memo.insert((from_spring, from_record), 0);
                    0
                }
            }
            Some(spring) => match spring {
                Spring::Operational => {
                    let r = self.nb_arrangements_rec(from_spring + 1, from_record, memo);
                    memo.insert((from_spring, from_record), r);
                    r
                }
                Spring::Damaged => {
                    let r = handle_damaged(self, from_spring, from_record, memo);
                    memo.insert((from_spring, from_record), r);
                    r
                }
                Spring::Unknown => {
                    let r = self.nb_arrangements_rec(from_spring + 1, from_record, memo)
                        + handle_damaged(self, from_spring, from_record, memo);
                    memo.insert((from_spring, from_record), r);
                    r
                }
            },
        };

        fn handle_damaged(
            line: &Line,
            from_spring: usize,
            from_record: usize,
            memo: &mut Memo,
        ) -> Len {
            match line.groups.get(from_record) {
                None => 0,
                Some(damaged_len) => {
                    let damaged_len = *damaged_len as usize;

                    if (0..damaged_len).any(|i| {
                        matches!(
                            line.springs.get(from_spring + i),
                            None | Some(Spring::Operational)
                        )
                    }) {
                        memo.insert((from_spring, from_record), 0);
                        return 0;
                    }

                    if matches!(
                        line.springs.get(from_spring + damaged_len),
                        Some(Spring::Damaged)
                    ) {
                        memo.insert((from_spring, from_record), 0);
                        return 0;
                    }

                    let r = line.nb_arrangements_rec(
                        from_spring + damaged_len + 1,
                        from_record + 1,
                        memo,
                    );
                    memo.insert((from_spring, from_record), r);
                    r
                }
            }
        }

        fn not_enough_springs(line: &Line, from_spring: usize, from_record: usize) -> bool {
            let nb_needed_springs = line
                .groups
                .iter()
                .skip(from_record)
                .map(|&len| len as usize)
                .fold(0, |acc, len| if acc == 0 { len } else { acc + len + 1 });
            line.springs.len() >= from_spring
                && line.springs.len() - from_spring < nb_needed_springs
        }

        fn not_enough_remaining_potential_damaged(
            line: &Line,
            from_spring: usize,
            from_record: usize,
        ) -> bool {
            let nb_needed_springs = line
                .groups
                .iter()
                .skip(from_record)
                .map(|&len| len as usize)
                .sum();
            let remaining_potential_damaged_springs = line
                .springs
                .iter()
                .skip(from_spring)
                .filter(|spring| matches!(spring, Spring::Damaged | Spring::Unknown))
                .count();

            remaining_potential_damaged_springs < nb_needed_springs
        }

        fn has_too_many_remaining_damaged(
            line: &Line,
            from_spring: usize,
            from_record: usize,
        ) -> bool {
            let max_damaged: Len = line.groups.iter().skip(from_record).sum();
            let damaged_count = line
                .springs
                .iter()
                .skip(from_spring)
                .filter(|&&s| s == Spring::Damaged)
                .count();

            damaged_count > max_damaged as usize
        }
    }
}

impl FromStr for Line {
    type Err = CannotParseLine;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, records) = s
            .split_once(" ")
            .ok_or(CannotParseLine::NoSplitSpringsFromRecords(s.to_string()))?;

        let springs = springs.chars().map(|c| c.try_into()).try_collect()?;
        let records = records.split(",").map(|s| s.parse()).try_collect()?;
        Ok(Self {
            springs,
            groups: records,
        })
    }
}

#[derive(Error, Debug)]
enum CannotParseLine {
    #[error("Cannot find spring and records from line: {0}")]
    NoSplitSpringsFromRecords(String),
    #[error("Cannot parse spring: {0}")]
    CannotParseSpring(
        #[from]
        #[source]
        CannotParseSpring,
    ),
    #[error("Cannot parse record: {0}")]
    CannotParseRecord(
        #[from]
        #[source]
        std::num::ParseIntError,
    ),
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = CannotParseSpring;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => return Err(CannotParseSpring { char: value }),
        })
    }
}

#[derive(Error, Debug)]
#[error("Cannot parse spring from char: {char}")]
struct CannotParseSpring {
    char: char,
}

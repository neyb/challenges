use anyhow::*;
use challenges_common::MyIterTools;
use itertools::Itertools;
use std::str::FromStr;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let (locks, keys) = content
        .lines()
        .split(|line| line.is_empty())
        .map(|lines| Schematic::from_str(&lines.join("\n")))
        .try_fold(
            (Vec::new(), Vec::new()),
            |(mut locks, mut keys), schematic| {
                match schematic? {
                    Schematic::Lock(lock) => locks.push(lock),
                    Schematic::Key(key) => keys.push(key),
                };
                Ok((locks, keys))
            },
        )?;

    Ok(keys
        .iter()
        .flat_map(|key| locks.iter().map(move |lock| (key, lock)))
        .filter(|(key, lock)| !overlap(key, lock))
        .count())
}

struct Lock {
    shape: [u8; 5],
}

struct Key {
    shape: [u8; 5],
}

fn overlap(key: &Key, lock: &Lock) -> bool {
    key.shape
        .iter()
        .zip(lock.shape.iter())
        .any(|(k, l)| k + l > 5)
}

enum Schematic {
    Lock(Lock),
    Key(Key),
}

impl FromStr for Schematic {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.lines().next() == Some("#####") {
            Ok(Self::Lock(s.parse()?))
        } else {
            Ok(Self::Key(s.parse()?))
        }
    }
}

impl FromStr for Lock {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines = s.lines().collect_vec();
        let shape = parse_shape_from_top(&lines)?;
        Ok(Self { shape })
    }
}

impl FromStr for Key {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines().collect_vec();
        lines.reverse();
        let shape = parse_shape_from_top(&lines)?;
        Ok(Self { shape })
    }
}

fn parse_shape_from_top(lines: &Vec<&str>) -> Result<[u8; 5]> {
    let mut shape = [0; 5];

    for line in lines.iter().skip(1) {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                *shape.get_mut(i).ok_or_else(|| anyhow!("too long line"))? += 1;
            }
        }
    }

    Ok(shape)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "25-test.txt"]);
        assert_eq!(run(&content).unwrap(), 3);
    }
}

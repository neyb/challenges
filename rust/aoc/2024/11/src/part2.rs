use anyhow::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

type Res = TStone;
pub(crate) fn run(content: &str) -> Result<Res> {
    let mut stone_line: StoneLine = content.parse()?;
    for _ in 0..75 {
        stone_line.blink();
    }

    Ok(stone_line.len())
}

struct StoneLine {
    current_stones: HashMap<Stone, usize>,
}

impl StoneLine {
    fn blink(&mut self) {
        let stones = self
            .current_stones
            .iter()
            // .copied()
            .map(|(&stone, &count)| (stone, count))
            .collect_vec();
        for (stone, count) in stones {
            let (first, second) = stone.blink();
            *self.current_stones.entry(stone).or_insert(0) -= count;
            *self.current_stones.entry(first).or_insert(0) += count;
            if let Some(second_stone) = second {
                *self.current_stones.entry(second_stone).or_insert(0) += count;
            }
        }
    }

    fn len(&self) -> TStone {
        self.current_stones
            .iter()
            .map(|(stone, count)| *count as TStone)
            .sum()
    }
}

type TStone = u64;
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Stone(TStone);

impl Stone {
    fn blink(&self) -> (Self, Option<Self>) {
        match self.0 {
            0 => (Stone(1), None),
            _ if (self.0.ilog10() + 1) % 2 == 0 => {
                let mask = (10 as TStone).pow((self.0.ilog10() + 1) / 2);
                (Stone(self.0 / mask), Some(Stone(self.0 % mask)))
            }
            _ => (Stone(self.0 * 2024), None),
        }
    }
}

impl FromStr for StoneLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let stones: Vec<Stone> = s
            .split(" ")
            .map(|s| s.parse().map(|n| Stone(n)))
            .try_collect()?;

        Ok(Self {
            current_stones: stones.into_iter().counts(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "11-test.txt"]);
        let mut line: StoneLine = content.parse().unwrap();

        for _ in 0..25 {
            line.blink();
        }
        assert_eq!(line.len(), 55312);
    }

    #[test]
    fn test_part1_0_blink() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "11-test.txt"]);
        let mut line: StoneLine = content.parse().unwrap();
        assert_eq!(line.len(), 2);
    }

    #[test]
    fn test_part1_1_blink() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "11-test.txt"]);
        let mut line: StoneLine = content.parse().unwrap();
        line.blink();
        assert_eq!(line.len(), 3);
    }

    #[test]
    fn test_part1_6_blink() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "11-test.txt"]);
        let mut line: StoneLine = content.parse().unwrap();
        for _ in 0..6 {
            line.blink();
        }
        assert_eq!(line.len(), 22);
    }
}

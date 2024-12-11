use anyhow::*;
use itertools::Itertools;
use std::str::FromStr;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let mut stone_line: StoneLine = content.parse()?;
    for _ in 0..25 {
        stone_line.blink();
    }

    Ok(stone_line.stones.len())
}

type Stone = u64;

struct StoneLine {
    stones: Vec<Stone>,
}

impl StoneLine {
    fn blink(&mut self) {
        for i in (0..self.stones.len()).rev() {
            let mut stone = &mut self.stones[i];
            match stone {
                0 => *stone = 1,
                _ if (stone.ilog10() + 1) % 2 == 0 => {
                    let mask = (10 as Stone).pow((stone.ilog10() + 1) / 2);
                    let next_stone = *stone % mask;
                    *stone /= mask;
                    self.stones.insert(i + 1, next_stone)
                }
                _ => *stone *= 2024,
            }
        }
    }
}

impl FromStr for StoneLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let stones = s.split(" ").map(|s| s.parse()).try_collect()?;
        Ok(Self { stones })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "11-test.txt"]);
        assert_eq!(run(&content).unwrap(), 55312);
    }
}

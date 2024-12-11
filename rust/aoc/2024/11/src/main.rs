use anyhow::Error;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "11.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

type TStone = u64;

struct StoneLine {
    current_stones: HashMap<Stone, usize>,
}

impl StoneLine {
    fn blink(&mut self) {
        let stones = self
            .current_stones
            .iter()
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
            .values()
            .map(|count| *count as TStone)
            .sum()
    }
}

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

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let stones: Vec<Stone> = s.split(" ").map(|s| s.parse().map(Stone)).try_collect()?;

        anyhow::Ok(Self {
            current_stones: stones.into_iter().counts(),
        })
    }
}

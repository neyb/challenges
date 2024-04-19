use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use lazy_regex::regex_captures;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "04.txt"]);
    let table: Table = content.parse().unwrap();
    println!("part 1: {}", table.value_sums());
    println!("part 2: {}", part_2::run(&table));
}

struct Table {
    cards: Vec<Card>,
}

struct Card {
    winning_numbers: HashSet<u32>,
    actual_numbers: HashSet<u32>,
}

impl Table {
    fn value_sums(&self) -> u32 {
        self.cards.iter().map(|card| card.value()).sum()
    }
}

impl Card {
    fn value(&self) -> u32 {
        (0..self.nb_wins()).fold(0, |acc, _| match acc {
            0 => 1,
            x => x * 2,
        })
    }

    fn nb_wins(&self) -> usize {
        self.actual_numbers
            .iter()
            .filter(|actual| self.winning_numbers.contains(actual))
            .count()
    }
}

impl FromStr for Table {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let cards = s.lines().map(Card::from_str).collect::<Result<_>>()?;
        Ok(Table { cards })
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, winning_str, actual_str) =
            regex_captures!(r"Card +\d+: ([\d ]+) \| ([\d ]+)", s)
                .ok_or_else(|| anyhow!("cannot parse card from {}", s))?;

        let parse_numbers = |s: &str| -> Result<HashSet<u32>> {
            let regex = lazy_regex::regex!(r"\d+");
            let numbers = regex
                .captures_iter(s)
                .map(|captures| {
                    let (number_as_str, []) = captures.extract();
                    number_as_str.parse().context("error parsing number")
                })
                .collect::<Result<_>>()?;
            Ok(numbers)
        };

        Ok(Self {
            winning_numbers: parse_numbers(winning_str)?,
            actual_numbers: parse_numbers(actual_str)?,
        })
    }
}

mod part_1 {
    #[cfg(test)]
    mod test {
        use crate::Table;

        #[test]
        fn given_test() {
            let table: Table = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
                .parse()
                .unwrap();

            assert_eq!(table.value_sums(), 13)
        }
    }
}

mod part_2 {
    use crate::Table;

    struct Scratchcards {
        count: Vec<u32>,
    }

    impl Scratchcards {
        fn of_size(size: usize) -> Self {
            Self {
                count: vec![1; size],
            }
        }

        fn add(&mut self, index: usize, nb_cards: u32, nb_wins: usize) {
            for i in 1..=nb_wins {
                self.count[index + i] += nb_cards
            }
        }
    }

    pub fn run(table: &Table) -> u32 {
        let mut scratchcards = Scratchcards::of_size(table.cards.len());

        for (i, card) in table.cards.iter().enumerate() {
            let nb_wins = card.nb_wins();
            let &x = scratchcards.count.get(i).unwrap();
            scratchcards.add(i, x, nb_wins);
        }

        scratchcards.count.iter().sum()
    }
}

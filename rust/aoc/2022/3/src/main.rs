use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let lines =
        challenges_common::get_input_lines(vec!["aoc", "2022", "3.txt"]).expect("cannot read file");

    let rucksaks: Vec<Rucksack> = lines.iter().map(Rucksack::parse).collect();
    println!("part1: {}", sum_part1(&rucksaks));
    println!("part2: {}", sum_part2(&rucksaks));
}

fn sum_part1(rucksaks: &[Rucksack]) -> u32 {
    rucksaks
        .iter()
        .map(Rucksack::common_item)
        .map(|common_item| common_item.expect("no common item found"))
        .map(value_of)
        .sum()
}

fn sum_part2(rucksaks: &[Rucksack]) -> u32 {
    rucksaks
        .iter()
        .chunks(3)
        .into_iter()
        .map(|group| {
            if let Some((r1, r2, r3)) = group.collect_tuple() {
                *r1.items()
                    .intersection(&r2.items())
                    .copied()
                    .collect::<HashSet<_>>()
                    .intersection(&r3.items())
                    .exactly_one()
                    .expect("no single common item in group")
            } else {
                panic!("not 3 members in group");
            }
        })
        .map(value_of)
        .sum()
}

fn value_of(item: char) -> u32 {
    match item {
        'a'..='z' => 1 + item as u32 - 'a' as u32,
        'A'..='Z' => 27 + item as u32 - 'A' as u32,
        _ => panic!("unsupported char: {}", item),
    }
}
struct Rucksack<'a> {
    first_compartment: &'a str,
    second_compartment: &'a str,
}

impl<'a> Rucksack<'a> {
    fn parse(line: &'a String) -> Self {
        let len = line.len();
        Rucksack {
            first_compartment: &line[..len / 2],
            second_compartment: &line[len / 2..],
        }
    }

    fn common_item(&self) -> Option<char> {
        let first_items: HashSet<char> = self.first_compartment.chars().collect();
        let second_items: HashSet<char> = self.second_compartment.chars().collect();

        first_items
            .intersection(&second_items)
            .exactly_one()
            .ok()
            .copied()
    }

    fn items(&self) -> HashSet<char> {
        self.first_compartment
            .chars()
            .chain(self.second_compartment.chars())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::{sum_part1, Rucksack};

    #[test]
    fn first_rucksak() {
        let s = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        let r = Rucksack::parse(&s);
        assert_eq!(r.common_item(), Some('p'));
    }

    #[test]
    fn given_test_part1() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        let rucksaks: Vec<Rucksack> = lines.iter().map(Rucksack::parse).collect();
        assert_eq!(157, sum_part1(&rucksaks));
    }
}

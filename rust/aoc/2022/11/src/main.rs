use std::{path::Path, str::FromStr};

use anyhow::{anyhow, Error, Ok, Result};
use challenges_common::MyIterTools;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let monkeys = monkeys_from(&["aoc", "2022", "11.txt"]);
    println!("part1: {}", part1(monkeys.clone()));
    println!("part2: {}", part2(monkeys.clone()));
}

fn monkeys_from(location: &[impl AsRef<Path>]) -> Monkeys {
    Monkeys::new(
        challenges_common::get_input_lines(location)
            .split(|line| line.is_empty())
            .map(|lines| lines.join("\n"))
            .map(|lines| Monkey::from_str(&lines).unwrap())
            .collect_vec(),
    )
}

fn part1(monkeys: Monkeys) -> u64 {
    run(monkeys, 20, 3)
}

fn part2(mut monkeys: Monkeys) -> u64 {
    monkeys.optimize();
    run(monkeys, 10000, 1)
}

fn run(mut monkeys: Monkeys, rounds: u32, divider: ItemValue) -> u64 {
    monkeys.apply_divider(divider);
    for _ in 0..rounds {
        monkeys.run_round();
    }
    monkeys
        .monkeys
        .iter()
        .map(|monkey| monkey.inspected_items_count)
        .sorted()
        .rev()
        .collect_vec()
        .iter()
        .take(2)
        .map(|&count| count as u64)
        .reduce(|a, b| a * b)
        .unwrap()
}

#[derive(Clone)]
struct Monkeys {
    monkeys: Vec<Monkey>,
}

fn gcd(a: ItemValue, b: ItemValue) -> ItemValue {
    let max = a.max(b);
    let min = a.min(b);

    let rest = max % min;

    if rest == 0 {
        min
    } else {
        gcd(min, rest)
    }
}

fn lcm(a: ItemValue, b: ItemValue) -> ItemValue {
    a / gcd(a, b) * b
}

impl Monkeys {
    fn new(monkeys: Vec<Monkey>) -> Self {
        Self { monkeys }
    }

    fn optimize(&mut self) {
        let scm = self
            .monkeys
            .iter()
            .map(|monkey| monkey.throw.divisible_test)
            .reduce(lcm)
            // .reduce(|a, b| a * b)
            .unwrap();

        for monkey in &mut self.monkeys {
            monkey.set_scm(scm)
        }
    }

    fn apply_divider(&mut self, divider: ItemValue) {
        for monkey in &mut self.monkeys {
            monkey.set_divider(divider);
        }
    }

    fn run_round(&mut self) {
        for i in 0..self.monkeys.len() {
            let throws = self.monkeys[i].do_its_things();
            for (to, item) in throws {
                self.monkeys[to].items.push(item)
            }
        }
    }
}

type ItemValue = u64;
#[derive(Clone, PartialEq, Debug)]
struct Monkey {
    items: Vec<ItemValue>,
    operation: Operation,
    throw: ConditionalThrow,
    inspected_items_count: u32,
    divider: ItemValue,
    scm: ItemValue,
}

impl Monkey {
    fn new(items: Vec<ItemValue>, operation: Operation, throw: ConditionalThrow) -> Self {
        Self {
            items,
            operation,
            throw,
            inspected_items_count: 0,
            divider: 1,
            scm: ItemValue::MAX,
        }
    }

    fn do_its_things(&mut self) -> Vec<(usize, ItemValue)> {
        let result = self
            .items
            .iter()
            .map(|&item| self.calc_worry_level(item) / self.divider)
            .map(|item| self.throw(item))
            .collect();
        self.inspected_items_count += self.items.len() as u32;
        self.items.clear();
        result
    }

    fn calc_worry_level(&self, item: ItemValue) -> ItemValue {
        self.operation.calc_worry_level(item) % self.scm
    }

    fn throw(&self, item: ItemValue) -> (usize, ItemValue) {
        self.throw.throw(item)
    }

    fn set_divider(&mut self, divider: ItemValue) {
        self.divider = divider;
    }

    fn set_scm(&mut self, scm: ItemValue) {
        self.scm = scm;
    }
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(
                r" *Monkey \d+:
 *Starting items: (?P<items>.*)
 *Operation: new = old (?P<op>.*)
 *Test: divisible by (?P<div>\d+)
 *If true: throw to monkey (?P<truemonkey>\d+)
 *If false: throw to monkey (?P<falsemonkey>\d+)"
            )
            .unwrap();
        }

        let captures = PATTERN
            .captures(s)
            .ok_or(anyhow!("cannot parse monkey from {}", s))?;

        let items = captures["items"]
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        Ok(Self::new(
            items,
            captures["op"].parse()?,
            ConditionalThrow {
                divisible_test: captures["div"].parse()?,
                true_monkey_index: captures["truemonkey"].parse()?,
                false_monkey_index: captures["falsemonkey"].parse()?,
            },
        ))
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Operation {
    Add(ItemValue),
    Mult(ItemValue),
    Square,
}
impl Operation {
    fn calc_worry_level(&self, item: ItemValue) -> ItemValue {
        match self {
            Operation::Add(to_add) => item + to_add,
            Operation::Mult(to_mult) => item * to_mult,
            Operation::Square => item * item,
        }
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let var_name = &s.split(' ').collect_vec()[..];
        match var_name {
            ["+", to_add] => Ok(Self::Add(to_add.parse()?)),
            ["*", "old"] => Ok(Self::Square),
            ["*", to_mult] => Ok(Self::Mult(to_mult.parse()?)),
            _ => Err(anyhow!("\"{}\" cannot be parsed to Operation", s)),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct ConditionalThrow {
    divisible_test: ItemValue,
    true_monkey_index: usize,
    false_monkey_index: usize,
}
impl ConditionalThrow {
    fn throw(&self, item: ItemValue) -> (usize, ItemValue) {
        (
            if item % self.divisible_test == 0 {
                self.true_monkey_index
            } else {
                self.false_monkey_index
            },
            item,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn parsing_a_monkey() {
        let monkey: Monkey = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"
            .parse()
            .unwrap();

        assert_eq!(
            monkey,
            Monkey::new(
                vec![79, 98],
                Operation::Mult(19),
                ConditionalThrow {
                    divisible_test: 23,
                    true_monkey_index: 2,
                    false_monkey_index: 3
                }
            )
        )
    }

    fn given_test_monkeys() -> Monkeys {
        monkeys_from(&["aoc", "2022", "11-test.txt"])
    }

    #[test]
    fn given_test_first_monkey_first_round() {
        let mut monkeys = given_test_monkeys();
        let first_monkey = monkeys.monkeys.first_mut().unwrap();
        first_monkey.set_divider(3);

        assert_eq!(first_monkey.items, vec![79, 98]);
        let throws = first_monkey.do_its_things();

        assert_eq!(first_monkey.items, vec![]);
        assert_eq!(throws, vec![(3, 500), (3, 620)])
    }

    #[test]
    fn given_test_first_round() {
        let mut monkeys = given_test_monkeys();
        monkeys.apply_divider(3);

        monkeys.run_round();

        assert_eq!(monkeys.monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(
            monkeys.monkeys[1].items,
            vec![2080, 25, 167, 207, 401, 1046]
        );
        assert_eq!(monkeys.monkeys[2].items, vec![]);
        assert_eq!(monkeys.monkeys[3].items, vec![]);
    }

    #[test]
    fn given_test_20_rounds() {
        let mut monkeys = given_test_monkeys();
        monkeys.apply_divider(3);

        for _ in 0..20 {
            monkeys.run_round();
        }

        assert_eq!(monkeys.monkeys[0].items, vec![10, 12, 14, 26, 34]);
        assert_eq!(monkeys.monkeys[1].items, vec![245, 93, 53, 199, 115]);
        assert_eq!(monkeys.monkeys[2].items, vec![]);
        assert_eq!(monkeys.monkeys[3].items, vec![]);
    }

    #[test]
    fn given_test_part1() {
        let monkeys = given_test_monkeys();
        assert_eq!(part1(monkeys), 10605)
    }

    #[test]
    fn given_monkey_part2_round1() {
        let mut monkeys = given_test_monkeys();
        monkeys.optimize();
        monkeys.run_round();

        assert_eq!(
            monkeys
                .monkeys
                .iter()
                .map(|m| m.inspected_items_count)
                .collect_vec(),
            vec![2, 4, 3, 6]
        );
    }

    #[test]
    fn given_monkey_part2_round20() {
        let mut monkeys = given_test_monkeys();
        monkeys.optimize();
        for _ in 0..20 {
            monkeys.run_round();
        }
        assert_eq!(
            monkeys
                .monkeys
                .iter()
                .map(|m| m.inspected_items_count)
                .collect_vec(),
            vec![99, 97, 8, 103]
        );
    }

    #[test]
    fn given_test_part2() {
        let monkeys = given_test_monkeys();
        assert_eq!(part2(monkeys), 2713310158)
    }
}

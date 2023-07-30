use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use lazy_regex::regex_captures;

fn main() {
    let monkeys = parse(&["aoc", "2022", "21.txt"]);
    println!("part1: {}", part1(&monkeys).unwrap());
    println!("part2: {}", part2(&monkeys).unwrap());
}

fn part1(monkeys: &Monkeys) -> Result<Number> {
    monkeys.get("root")?.yell(&monkeys)
}

fn part2(monkeys: &Monkeys) -> Result<Number> {
    use Operation::*;

    let mut monkeys = monkeys.clone();
    monkeys.monkeys.insert(
        "humn".to_string(),
        Monkey {
            name: "humn".to_string(),
            job: MonkeyJob::You,
        },
    );

    let (name_a, name_b) = match &monkeys.get("root")?.job {
        MonkeyJob::Calc(Sum(a, b) | Sub(a, b) | Mult(a, b) | Div(a, b)) => (a, b),
        _ => bail!("root should have 2 operands"),
    };

    Monkey {
        name: "root".to_string(),
        job: MonkeyJob::Calc(Sub(name_a.clone(), name_b.clone())),
    }
    .deduct_value_to_yell(0, "humn", &monkeys)
}

fn parse(path: &[&str]) -> Monkeys {
    let monkeys = challenges_common::get_input_lines(path)
        .map(|line| line.parse::<Monkey>().unwrap())
        .map(|monkey| (monkey.name.clone(), monkey))
        .collect();
    Monkeys { monkeys }
}

#[derive(Clone)]
struct Monkeys {
    monkeys: HashMap<String, Monkey>,
}

impl Monkeys {
    fn find(&self, name: &str) -> Option<&Monkey> {
        self.monkeys.get(name)
    }

    fn get(&self, name: &str) -> Result<&Monkey> {
        self.find(name)
            .ok_or_else(|| anyhow!("no such monkey {}", name))
    }
}

#[derive(Clone)]
struct Monkey {
    name: String,
    job: MonkeyJob,
}

impl Monkey {
    fn yell(&self, monkeys: &Monkeys) -> Result<Number> {
        self.job.evaluate(monkeys)
    }

    fn deduct_value_to_yell(
        &self,
        result: Number,
        searched: &str,
        monkeys: &Monkeys,
    ) -> Result<Number> {
        if self.name == searched {
            Ok(result)
        } else {
            use Operation::*;

            fn deduct_value_for(
                a: &str,
                b: &str,
                expected_a: impl FnOnce(Number) -> Number,
                expected_b: impl FnOnce(Number) -> Number,
                searched: &str,
                monkeys: &Monkeys,
            ) -> Result<Number> {
                let a = monkeys.get(&a)?;
                let b = monkeys.get(&b)?;

                match (a.yell(monkeys), b.yell(monkeys)) {
                    (Ok(n), Err(_)) => b.deduct_value_to_yell(expected_b(n), searched, monkeys),
                    (Err(_), Ok(n)) => a.deduct_value_to_yell(expected_a(n), searched, monkeys),
                    _ => bail!("cannot solve..."),
                }
            }

            match &self.job {
                MonkeyJob::Calc(Sum(a, b)) => {
                    deduct_value_for(a, b, |b| result - b, |a| result - a, searched, monkeys)
                }
                MonkeyJob::Calc(Mult(a, b)) => {
                    deduct_value_for(a, b, |b| result / b, |a| result / a, searched, monkeys)
                }
                MonkeyJob::Calc(Sub(a, b)) => {
                    deduct_value_for(a, b, |b| result + b, |a| a - result, searched, monkeys)
                }
                MonkeyJob::Calc(Div(a, b)) => {
                    deduct_value_for(a, b, |b| result * b, |a| a / result, searched, monkeys)
                }
                _ => bail!("cannot solve..."),
            }
        }
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, name, job) = regex_captures!(r"^(\w+): (.+)$", s)
            .ok_or_else(|| anyhow!("cannot parse monkey: {}", s))?;
        Ok(Self {
            name: name.to_owned(),
            job: job.parse()?,
        })
    }
}

type Number = i64;

#[derive(Clone)]
enum MonkeyJob {
    Number(Number),
    Calc(Operation),
    You,
}

impl FromStr for MonkeyJob {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        s.parse::<Number>().map_or_else(
            |_| {
                let operation = s.parse()?;
                Ok(Self::Calc(operation))
            },
            |n| Ok(Self::Number(n)),
        )
    }
}

impl MonkeyJob {
    fn evaluate(&self, monkeys: &Monkeys) -> Result<Number> {
        match self {
            MonkeyJob::Number(n) => Ok(*n),
            MonkeyJob::Calc(op) => op.calc(monkeys),
            MonkeyJob::You => bail!("dunno what to yell !"),
        }
    }
}

#[derive(Clone)]
enum Operation {
    Sum(String, String),
    Sub(String, String),
    Mult(String, String),
    Div(String, String),
}

impl Operation {
    fn calc(&self, monkeys: &Monkeys) -> Result<Number> {
        let get = |name: &str| monkeys.get(name)?.yell(&monkeys);

        let result = match self {
            Operation::Sum(a, b) => get(a)? + get(b)?,
            Operation::Sub(a, b) => get(a)? - get(b)?,
            Operation::Mult(a, b) => get(a)? * get(b)?,
            Operation::Div(a, b) => get(a)? / get(b)?,
        };

        Ok(result)
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        use Operation::*;

        match &s.split(" ").collect::<Vec<_>>()[..] {
            [a, "+", b] => Ok(Sum(a.to_string(), b.to_string())),
            [a, "-", b] => Ok(Sub(a.to_string(), b.to_string())),
            [a, "*", b] => Ok(Mult(a.to_string(), b.to_string())),
            [a, "/", b] => Ok(Div(a.to_string(), b.to_string())),
            _ => bail!("cannot parse operation from {}", s),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{parse, part1, part2};

    #[test]
    fn given_test_part1() {
        let monkeys = parse(&["aoc", "2022", "21-test.txt"]);
        assert_eq!(part1(&monkeys).unwrap(), 152)
    }

    #[test]
    fn given_test_part2() {
        let monkeys = parse(&["aoc", "2022", "21-test.txt"]);
        assert_eq!(part2(&monkeys).unwrap(), 301)
    }
}

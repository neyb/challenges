use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "07.txt"]);
    println!("part1: {:?}", run::<part1::Operator>(&content));
    println!("part2: {:?}", run::<part2::Operator>(&content));
}

mod part1;
mod part2;

type Res = u64;

fn run<Op: Operator>(content: &str) -> anyhow::Result<Res> {
    let equations: Equations = content.parse()?;
    anyhow::Ok(equations.get_solvable_result_sum::<Op>())
}

struct Equations(Vec<Equation>);

impl Equations {
    fn get_solvable_result_sum<Op: Operator>(&self) -> Res {
        self.0
            .iter()
            .filter(|eq| eq.can_be_solved::<Op>())
            .map(|equation| equation.result)
            .sum()
    }
}

struct Equation {
    result: Res,
    operands: Vec<Res>,
}

impl Equation {
    fn can_be_solved<Op: Operator>(&self) -> bool {
        return can_be_solved_with::<Op>(self.result, &self.operands);

        fn can_be_solved_with<Op: Operator>(res: Res, operands: &[Res]) -> bool {
            match operands {
                [] => panic!("no operands"),
                [operand] => *operand == res,
                [rest @ .., last_operand] => Op::all().iter().any(|operator| {
                    match operator.resolve_first_operand(res, *last_operand) {
                        None => false,
                        Some(a) => can_be_solved_with::<Op>(a, rest),
                    }
                }),
            }
        }
    }
}

impl FromStr for Equations {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(Self(s.lines().map(|line| line.parse()).try_collect()?))
    }
}

impl FromStr for Equation {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (result, operands) = s
            .split_once(": ")
            .ok_or_else(|| anyhow!("cannot parse line"))?;

        anyhow::Ok(Self {
            result: result.parse()?,
            operands: operands.split(" ").map(|op| op.parse()).try_collect()?,
        })
    }
}

trait Operator: Sized {
    fn all() -> Vec<Self>;
    fn resolve_first_operand(&self, res: Res, b: Res) -> Option<Res>;
}

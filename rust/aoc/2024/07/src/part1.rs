use crate::{Equation, Res};
use anyhow::*;
use itertools::Itertools;

pub(crate) fn run(content: &str) -> Result<Res> {
    let equations: Vec<Equation> = content.lines().map(|line| line.parse()).try_collect()?;

    Ok(equations
        .iter()
        .filter(|eq| eq.can_be_solved())
        .map(|equation| equation.result)
        .sum())
}

trait Part1Equation {
    fn can_be_solved(&self) -> bool;
    fn can_be_solved_with(&self, operators: &mut Vec<Operation>) -> bool;
}

impl Part1Equation for Equation {
    fn can_be_solved(&self) -> bool {
        self.can_be_solved_with(&mut Vec::new())
    }

    fn can_be_solved_with(&self, mut operators: &mut Vec<Operation>) -> bool {
        if self.operands.len() == operators.len() + 1 {
            self.operands
                .iter()
                .tuple_windows()
                .zip(operators)
                .fold(None, |sum, ((a, b), op)| match sum {
                    None => Some(op.apply(*a, *b)),
                    Some(res) => Some(op.apply(res, *b)),
                })
                == Some(self.result)
        } else {
            operators.push(Operation::Add);
            if self.can_be_solved_with(&mut operators) {
                return true;
            }
            operators.pop();
            operators.push(Operation::Multiply);
            if self.can_be_solved_with(&mut operators) {
                return true;
            }
            operators.pop();
            false
        }
    }
}

enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(&self, a: crate::part1::Res, b: crate::part1::Res) -> crate::part1::Res {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "07-test.txt"]);
        assert_eq!(run(&content).unwrap(), 3749);
    }
}

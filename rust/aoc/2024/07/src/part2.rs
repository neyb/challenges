use crate::Equation;
use anyhow::*;
use itertools::Itertools;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let equations: Vec<Equation> = content.lines().map(|line| line.parse()).try_collect()?;

    Ok(equations
        .iter()
        .filter(|eq| eq.can_be_solved())
        .map(|equation| equation.result)
        .sum())
}

trait Part2Equation {
    fn can_be_solved(&self) -> bool;
    fn can_be_solved_with(&self, operators: &mut Vec<Operation>) -> bool;
}

impl Part2Equation for Equation {
    fn can_be_solved(&self) -> bool {
        self.can_be_solved_with(&mut Vec::new())
    }

    fn can_be_solved_with(&self, mut operators: &mut Vec<Operation>) -> bool {
        if self.operands.len() == operators.len() + 1 {
            let result = self.operands.iter().tuple_windows().zip(operators).fold(
                None,
                |sum, ((a, b), op)| match sum {
                    None => Some(op.apply(*a, *b)),
                    Some(res) => Some(op.apply(res, *b)),
                },
            );
            result == Some(self.result)
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
            operators.push(Operation::Concatenate);
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
    Concatenate,
}

impl Operation {
    fn apply(&self, a: Res, b: Res) -> Res {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
            Self::Concatenate => {
                let mut a = a;
                let mut b_tmp = b;
                while b_tmp >= 10 {
                    a *= 10;
                    b_tmp /= 10;
                }
                a * 10 + b
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_concat_test() -> Result<()> {
        let equation: Equation = "156: 15 6".parse()?;
        assert_eq!(equation.can_be_solved(), true);
        Ok(())
    }

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "07-test.txt"]);
        assert_eq!(run(&content).unwrap(), 11387);
    }
}

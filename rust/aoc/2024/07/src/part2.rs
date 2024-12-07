use crate::Res;

#[derive(Clone)]
pub enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl crate::Operator for Operator {
    fn all() -> Vec<Self> {
        vec![Self::Add, Self::Multiply, Self::Concatenate]
    }

    fn resolve_first_operand(&self, res: Res, b: Res) -> Option<Res> {
        match self {
            Self::Add => {
                if res > b {
                    Some(res - b)
                } else {
                    None
                }
            }
            Self::Multiply => {
                if res % b == 0 {
                    Some(res / b)
                } else {
                    None
                }
            }
            Self::Concatenate => {
                let mask = 10_u64.pow(b.ilog10() + 1);
                if res % mask == b {
                    Some(res / mask)
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::*;

    #[test]
    fn basic_concat_test() -> Result<()> {
        let equation: crate::Equation = "156: 15 6".parse()?;
        assert!(equation.can_be_solved::<Operator>());
        Ok(())
    }

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "07-test.txt"]);
        assert_eq!(crate::run::<Operator>(&content).unwrap(), 11387);
    }
}

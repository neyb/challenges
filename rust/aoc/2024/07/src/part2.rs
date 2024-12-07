use crate::Res;

pub enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl crate::Operator for Operator {
    fn all() -> Vec<Self> {
        vec![Self::Add, Self::Multiply, Self::Concatenate]
    }

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
    use anyhow::*;

    #[test]
    fn basic_concat_test() -> Result<()> {
        let equation: crate::Equation = "156: 15 6".parse()?;
        assert_eq!(equation.can_be_solved::<Operator>(), true);
        Ok(())
    }

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "07-test.txt"]);
        assert_eq!(crate::run::<Operator>(&content).unwrap(), 11387);
    }
}

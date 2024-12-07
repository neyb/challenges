use crate::Res;

#[derive(Clone)]
pub enum Operator {
    Add,
    Multiply,
}

impl crate::Operator for Operator {
    fn all() -> Vec<Self> {
        vec![Self::Add, Self::Multiply]
    }

    fn resolve_first_operand(&self, res: Res, b: Res) -> Option<Res> {
        match self {
            Operator::Add => {
                if res > b {
                    Some(res - b)
                } else {
                    None
                }
            }
            Operator::Multiply => {
                if res % b == 0 {
                    Some(res / b)
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

    #[test]
    fn simple_test() {
        let equation: crate::Equation = "3267: 81 40 27".parse().unwrap();
        assert!(equation.can_be_solved::<Operator>());
    }

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "07-test.txt"]);
        assert_eq!(crate::run::<Operator>(&content).unwrap(), 3749);
    }
}

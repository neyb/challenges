use crate::Res;

pub enum Operator {
    Add,
    Multiply,
}

impl crate::Operator for Operator {
    fn all() -> Vec<Self> {
        vec![Self::Add, Self::Multiply]
    }

    fn apply(&self, a: Res, b: Res) -> Res {
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
        assert_eq!(crate::run::<Operator>(&content).unwrap(), 3749);
    }
}

use crate::{Code, RobotsSystem};
use anyhow::*;
use itertools::Itertools;
use std::str::FromStr;

type Res = usize;
pub fn run(content: &str) -> Result<Res> {
    let codes: Vec<Code> = content.lines().map(Code::from_str).try_collect()?;

    let mut system = RobotsSystem::new(3)?;
    codes
        .iter()
        .map(|code| -> Result<Res> {
            let found = system.find_a_shortest_sequence_len_for_code(code);
            Ok(code.num_part()? * found)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_you_type_on_num_pad() {
        let mut system = RobotsSystem::new(0).unwrap();
        let s = system.find_a_shortest_sequence_len_for_code(&Code::from_str("0123A").unwrap());
        assert_eq!(s, 5);
    }

    #[test]
    fn with_1_robot() {
        let mut system = RobotsSystem::new(1).unwrap();
        let s = system.find_a_shortest_sequence_len_for_code(&Code::from_str("0123A").unwrap());
        assert_eq!(s, 5);
    }

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "21-test.txt"]);
        assert_eq!(run(&content).unwrap(), 126384);
    }
}

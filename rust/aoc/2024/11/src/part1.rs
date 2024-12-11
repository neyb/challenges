use crate::{StoneLine, TStone};
use anyhow::*;

type Res = TStone;
pub(crate) fn run(content: &str) -> Result<Res> {
    let mut stone_line: StoneLine = content.parse()?;
    for _ in 0..25 {
        stone_line.blink();
    }

    Ok(stone_line.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "11-test.txt"]);
        assert_eq!(run(&content).unwrap(), 55312);
    }
}

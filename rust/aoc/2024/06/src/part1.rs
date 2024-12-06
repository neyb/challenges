use crate::Map;
use anyhow::*;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let map: Map = content.parse()?;
    Ok(map.all_guard_positions().len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "06-test.txt"]);
        assert_eq!(run(&content).unwrap(), 41);
    }
}

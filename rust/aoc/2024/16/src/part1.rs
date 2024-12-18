use crate::Map;
use anyhow::*;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let map: Map = content.parse()?;
    let reindeer = map.reindeer_start();
    let path = reindeer
        .get_best_path(&map)
        .ok_or_else(|| anyhow!("no path found"))?;
    Ok(path.cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "16-test-1.txt"]);
        assert_eq!(run(&content).unwrap(), 7036);
    }

    #[test]
    fn test2_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "16-test-2.txt"]);
        assert_eq!(run(&content).unwrap(), 11048);
    }
}

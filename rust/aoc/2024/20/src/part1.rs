use crate::{Map, PathAnalyzer};
use anyhow::*;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let map: Map = content.parse()?;
    let path = map.get_path().ok_or_else(|| anyhow!("No path found"))?;
    let path_analyzer = PathAnalyzer { path };
    let shortcuts = path_analyzer.get_shortcuts(2, 100);

    Ok(shortcuts.len())
}

#[cfg(test)]
mod tests {
    use crate::*;
    use itertools::Itertools;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "20-test.txt"]);
        let map: Map = content.parse().unwrap();
        let path = map.get_path().unwrap();
        let path_analyzer = PathAnalyzer { path };
        let shortcuts = path_analyzer.get_shortcuts(2, 10);
        let by_len = shortcuts.iter().counts_by(|shortcut| shortcut.win_time);

        assert_eq!(by_len.get(&10).unwrap(), &2);
        assert_eq!(by_len.get(&12).unwrap(), &3);
        assert_eq!(by_len.get(&20).unwrap(), &1);
        assert_eq!(by_len.get(&36).unwrap(), &1);
        assert_eq!(by_len.get(&38).unwrap(), &1);
        assert_eq!(by_len.get(&40).unwrap(), &1);
        assert_eq!(by_len.get(&64).unwrap(), &1);
    }
}

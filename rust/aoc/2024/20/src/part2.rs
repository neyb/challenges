use crate::{Map, PathAnalyzer};
use anyhow::*;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let map: Map = content.parse()?;
    let path = map.get_path().ok_or_else(|| anyhow!("No path found"))?;
    let path_analyzer = PathAnalyzer { path };
    let shortcuts = path_analyzer.get_shortcuts(20, 100);

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
        let shortcuts = path_analyzer.get_shortcuts(20, 50);
        let by_len = shortcuts.iter().counts_by(|shortcut| shortcut.win_time);

        assert_eq!(by_len.get(&52).unwrap(), &31);
        assert_eq!(by_len.get(&54).unwrap(), &29);
        assert_eq!(by_len.get(&56).unwrap(), &39);
        assert_eq!(by_len.get(&58).unwrap(), &25);
        assert_eq!(by_len.get(&60).unwrap(), &23);
        assert_eq!(by_len.get(&62).unwrap(), &20);
        assert_eq!(by_len.get(&64).unwrap(), &19);
        assert_eq!(by_len.get(&66).unwrap(), &12);
        assert_eq!(by_len.get(&68).unwrap(), &14);
        assert_eq!(by_len.get(&70).unwrap(), &12);
        assert_eq!(by_len.get(&72).unwrap(), &22);
        assert_eq!(by_len.get(&74).unwrap(), &4);
        assert_eq!(by_len.get(&76).unwrap(), &3);
    }
}

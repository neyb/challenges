use crate::Map;
use anyhow::*;
use itertools::Itertools;
use std::str::FromStr;

type Res = u32;
pub(crate) fn run(content: &str) -> Result<Res> {
    let mut map: Map = content.parse()?;
    map.sim(1024);
    let path = map.path().ok_or_else(|| anyhow!("no path found"))?;
    Ok(path.cost)
}

#[cfg(test)]
mod tests {
    use super::*;
    use challenges_common::graph::Coord;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "18-test.txt"]);
        let mut map: Map = content.parse().unwrap();
        map.bot_right = Coord { x: 6, y: 6 };
        map.sim(12);

        let path = map.path().unwrap();

        assert_eq!(path.cost, 22);
    }
}

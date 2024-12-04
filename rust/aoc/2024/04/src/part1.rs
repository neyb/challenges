use crate::{Direction, Map};
use anyhow::*;
use itertools::Itertools;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let map: Map = content.parse()?;
    Ok(map.count_xmas())
}

trait Part1Map {
    fn count_xmas(&self) -> Res;
}

impl Part1Map for Map {
    fn count_xmas(&self) -> Res {
        self.grid
            .coords()
            .flat_map(|coord| {
                Direction::all()
                    .into_iter()
                    .map(|dir| (coord.clone(), dir))
                    .collect_vec()
            })
            .filter_map(|(coord, dir)| {
                let chars = (0..4)
                    .map(|i| self.grid.get(&dir.move_from_coord_dist(&coord, i)?))
                    .collect::<Option<Vec<&char>>>()?;
                Some(chars.iter().copied().join(""))
            })
            .filter(|s| s == "XMAS")
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "04-test.txt"]);
        assert_eq!(run(&content).unwrap(), 18);
    }
}

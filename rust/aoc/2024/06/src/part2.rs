use crate::{Block, Map};
use anyhow::*;
use rayon::prelude::*;
use std::collections::HashSet;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let map: Map = content.parse()?;
    let guard_positions = map.all_guard_positions();

    Ok(guard_positions
        .par_iter()
        .map(|coord| {
            let mut map = map.clone();
            *map.grid.get_mut(&coord).unwrap() = Block::Obstruction;
            map
        })
        .filter(|map| map.does_loop())
        .count())
}

trait Part2Map {
    fn does_loop(&self) -> bool;
}

impl Part2Map for Map {
    fn does_loop(&self) -> bool {
        let guard = self.init_guard.clone();
        let mut positions = HashSet::new();

        std::iter::successors(Some(guard), |guard| guard.clone().next(self))
            .any(|guard_state| !positions.insert(guard_state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "06-test.txt"]);
        assert_eq!(run(&content).unwrap(), 6);
    }
}

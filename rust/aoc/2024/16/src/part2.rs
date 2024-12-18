use crate::{Map, Reindeer};
use anyhow::*;
use challenges_common::graph::astar_multiple;
use std::collections::HashSet;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let map: Map = content.parse()?;
    let reindeer = map.reindeer_start();
    let path_nodes = reindeer
        .get_best_paths(&map)
        .ok_or_else(|| anyhow!("no path found"))?
        .0
        .into_iter()
        .map(|reindeer| reindeer.coord)
        .collect::<HashSet<_>>();

    Ok(path_nodes.len())
}

trait Part2Reindeer {
    fn get_best_paths(&self, map: &Map) -> Option<(HashSet<Reindeer>, usize)>;
}

impl Part2Reindeer for Reindeer {
    fn get_best_paths(&self, map: &Map) -> Option<(HashSet<Reindeer>, usize)> {
        astar_multiple(
            self.clone(),
            |reindeer| reindeer.next(map),
            |reindeer| reindeer.coord == map.end,
            |reindeer| reindeer.coord.manhattan_dist_to(&map.end),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "16-test-1.txt"]);
        assert_eq!(run(&content).unwrap(), 45);
    }

    #[test]
    fn test2_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "16-test-2.txt"]);
        assert_eq!(run(&content).unwrap(), 64);
    }
}

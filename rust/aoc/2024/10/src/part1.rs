use anyhow::*;
use challenges_common::graph::{CannotParseElementFromChar, Coord, Grid};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let map: Map = content.parse()?;
    let trailheads = map.trailheads();

    Ok(trailheads
        .par_iter()
        .map(|trailhead| map.trail_score(trailhead))
        .sum())
}

struct Map {
    grid: Grid<Height>,
    links: HashMap<Coord, Vec<Coord>>,
}

impl Map {
    fn trailheads(&self) -> Vec<Coord> {
        self.grid
            .coords()
            .filter(|coord| self.grid.get(coord).unwrap().0 == 0)
            .collect()
    }

    fn trail_score(&self, start: &Coord) -> usize {
        self.trail_ends(start).len()
    }

    fn trail_ends(&self, start: &Coord) -> HashSet<Coord> {
        let mut result = HashSet::new();
        let mut visited = HashSet::new();

        trail_ends_recursive(self, *start, &mut result, &mut visited);

        return result;

        fn trail_ends_recursive(
            map: &Map,
            from: Coord,
            result: &mut HashSet<Coord>,
            visited: &mut HashSet<Coord>,
        ) {
            if visited.insert(from) {
                if map.grid.get(&from) == Some(&Height(9)) {
                    result.insert(from);
                } else if let Some(neighbours) = map.links.get(&from) {
                    for neighbour in neighbours {
                        trail_ends_recursive(map, *neighbour, result, visited);
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Height(u32);

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid: Grid<Height> = s.parse()?;
        let links = grid
            .coords()
            .map(|coord| {
                let target_height = grid.get(&coord).unwrap().0 + 1;
                let targets = coord
                    .neighbours(false)
                    .filter(|neighbour| grid.get(neighbour) == Some(&Height(target_height)))
                    .collect_vec();
                (coord, targets)
            })
            .collect();
        Ok(Self { grid, links })
    }
}

impl TryFrom<char> for Height {
    type Error = CannotParseElementFromChar;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Result::Ok(Self(
            value
                .to_digit(10)
                .ok_or_else(|| CannotParseElementFromChar::from(value))?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "10-test.txt"]);
        assert_eq!(run(&content).unwrap(), 36);
    }
}

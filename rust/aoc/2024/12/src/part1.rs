use anyhow::*;
use challenges_common::graph;
use challenges_common::graph::Grid;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let map: Map = content.parse()?;
    let regions = map.regions();
    Ok(regions.price())
}

type Price = usize;
type Coord = graph::Coord<i16>;

struct Regions {
    regions: HashMap<Plant, Vec<Region>>,
}

impl Regions {
    fn new() -> Self {
        Self {
            regions: HashMap::new(),
        }
    }

    fn add_cell(&mut self, plant: Plant, coord: Coord) {
        let regions = self.regions.remove(&plant).unwrap_or_else(|| Vec::new());
        let neighbors = coord.neighbours(false).collect_vec();
        let mut groups = regions.into_iter().into_group_map_by(|region| {
            region
                .cells
                .iter()
                .any(|current_cell| neighbors.contains(current_cell))
        });

        let mut neighbors = groups.remove(&true).unwrap_or_else(Vec::new);
        let mut not_neighbors = groups.remove(&false).unwrap_or_else(Vec::new);

        let mut region = Region::merge(neighbors);
        region.add(coord);

        not_neighbors.push(region);

        self.regions.insert(plant, not_neighbors);
    }

    fn price(&self) -> Price {
        self.regions
            .iter()
            .flat_map(|(plant, regions)| regions.into_iter().map(|region| (region, *plant)))
            .map(|(region, plant)| {
                dbg!(plant);
                dbg!(region.price())
            })
            .sum()
    }
}

struct Region {
    cells: Vec<Coord>,
}

impl Region {
    fn add(&mut self, coord: Coord) {
        self.cells.push(coord)
    }

    fn merge(regions: Vec<Self>) -> Self {
        let cells = regions.into_iter().flat_map(|r| r.cells).collect();
        Self { cells }
    }

    fn price(&self) -> Price {
        dbg!(dbg!(self.area()) * dbg!(self.perimeter()))
    }

    fn area(&self) -> usize {
        self.cells.len()
    }

    fn perimeter(&self) -> usize {
        self.cells
            .iter()
            .map(|&cell| {
                cell.neighbours(false)
                    .filter(|neighbor| !self.cells.contains(neighbor))
                    .count()
            })
            .sum()
    }
}

type Plant = char;

struct Map {
    grid: Grid<Plant, i16>,
}

impl Map {
    fn regions(&self) -> Regions {
        let mut regions = Regions::new();
        for (coord, &plant) in self.grid.entries() {
            regions.add_cell(plant, coord);
        }
        regions
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            grid: Grid::from_str(s)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_1() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test1.txt"]);
        assert_eq!(run(&content).unwrap(), 140);
    }
    #[test]
    fn given_test_2() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test2.txt"]);
        assert_eq!(run(&content).unwrap(), 772);
    }
    #[test]
    fn given_test_3() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test3.txt"]);
        assert_eq!(run(&content).unwrap(), 1930);
    }
}

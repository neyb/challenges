use anyhow::Error;
use challenges_common::graph;
use challenges_common::graph::Grid;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "12.txt"]);
    println!("part1: {:?}", run(&content, part1::Region::price));
    println!("part2: {:?}", run(&content, part2::Region::price));
}

mod part1;
mod part2;

type Res = usize;
// type GetPrice = fn(&Region) -> Price;
// type GetPrice = impl Fn(&Region) -> Price;

pub(crate) fn run(content: &str, get_price: impl Fn(&Region) -> Price) -> anyhow::Result<Res> {
    let map: Map = content.parse()?;
    let regions = map.regions();
    anyhow::Ok(regions.price(get_price))
}

type Price = usize;
type Coord = graph::Coord<i16>;
type Plant = char;

struct Regions {
    regions: HashMap<Plant, Vec<Region>>,
}

impl Regions {
    fn new() -> Self {
        Self {
            regions: HashMap::new(),
        }
    }

    // this is trash performance wise (quick-win should use square to limit possibilities)...
    fn add_cell(&mut self, plant: Plant, coord: Coord) {
        let regions = self.regions.remove(&plant).unwrap_or_default();
        let neighbors = coord.neighbours(false).collect_vec();
        let mut groups = regions.into_iter().into_group_map_by(|region| {
            region
                .cells
                .iter()
                .any(|current_cell| neighbors.contains(current_cell))
        });

        let neighbors = groups.remove(&true).unwrap_or_else(Vec::new);
        let mut not_neighbors = groups.remove(&false).unwrap_or_else(Vec::new);

        let mut region = Region::merge(neighbors);
        region.add(coord);

        not_neighbors.push(region);

        self.regions.insert(plant, not_neighbors);
    }

    fn price(&self, get_price: impl Fn(&Region) -> Price) -> Price {
        self.regions.values().flatten().map(get_price).sum()
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

    fn area(&self) -> usize {
        self.cells.len()
    }

    fn square(&self) -> Option<(Coord, Coord)> {
        let min_x = self.cells.iter().map(|coord| coord.x).min()?;
        let max_x = self.cells.iter().map(|coord| coord.x).max()?;
        let min_y = self.cells.iter().map(|coord| coord.y).min()?;
        let max_y = self.cells.iter().map(|coord| coord.y).max()?;
        Some((Coord { x: min_x, y: min_y }, Coord { x: max_x, y: max_y }))
    }
}

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

    fn from_str(s: &str) -> anyhow::Result<Self> {
        anyhow::Ok(Self {
            grid: Grid::from_str(s)?,
        })
    }
}

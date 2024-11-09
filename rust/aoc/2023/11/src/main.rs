fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "11.txt"]);
    println!("part 1: {:?}", part1::run(&content).unwrap());
    println!("part 2: {:?}", part2::run(&content).unwrap());
}

mod part1;
mod part2;

fn run(content: &str, expansion_rate: usize) -> anyhow::Result<usize> {
    let mut universe: Universe = content.parse()?;
    universe.expansion_rate = expansion_rate;
    let vec = universe.galaxies_pairs();
    Ok(vec
        .iter()
        .map(|(galaxy1, galaxy2)| universe.distance_between(galaxy1, galaxy2))
        .sum())
}

struct Universe {
    galaxies: Vec<Galaxy>,
    expanded_columns: Vec<usize>,
    expanded_rows: Vec<usize>,
    expansion_rate: usize,
}

impl Universe {
    fn with_galaxies(galaxies: Vec<Galaxy>) -> Self {
        use std::collections::HashSet;

        let mut xs = HashSet::new();
        let mut ys = HashSet::new();

        for galaxy in &galaxies {
            xs.insert(galaxy.coord.x);
            ys.insert(galaxy.coord.y);
        }

        match (
            xs.iter().min(),
            xs.iter().max(),
            ys.iter().min(),
            ys.iter().max(),
        ) {
            (Some(&min_x), Some(&max_x), Some(&min_y), Some(&max_y)) => {
                let free_columns = (min_x..max_x).filter(|x| !xs.contains(x)).collect();
                let free_rows = (min_y..max_y).filter(|y| !ys.contains(y)).collect();
                Self {
                    galaxies,
                    expanded_columns: free_columns,
                    expanded_rows: free_rows,
                    expansion_rate: 1,
                }
            }
            _ => Self {
                galaxies,
                expanded_columns: Vec::new(),
                expanded_rows: Vec::new(),
                expansion_rate: 1,
            },
        }
    }

    fn galaxies_pairs(&self) -> Vec<(&Galaxy, &Galaxy)> {
        let mut pairs = Vec::new();
        for i in 0..self.galaxies.len() {
            for j in i + 1..self.galaxies.len() {
                pairs.push((&self.galaxies[i], &self.galaxies[j]));
            }
        }
        pairs
    }

    fn distance_between(&self, galaxy1: &Galaxy, galaxy2: &Galaxy) -> usize {
        let x_distance = distance_1_between(
            galaxy1.coord.x,
            galaxy2.coord.x,
            &self.expanded_columns,
            self.expansion_rate,
        );
        let y_distance = distance_1_between(
            galaxy1.coord.y,
            galaxy2.coord.y,
            &self.expanded_rows,
            self.expansion_rate,
        );
        return x_distance + y_distance;

        fn distance_1_between(
            v1: usize,
            v2: usize,
            expanded: &[usize],
            expansion_rate: usize,
        ) -> usize {
            let min = v1.min(v2);
            let max = v1.max(v2);
            let mut distance = max - min;
            for v in min..max {
                if expanded.contains(&v) {
                    distance -= 1;
                    distance += expansion_rate;
                }
            }

            distance
        }
    }
}

use std::str::FromStr;
impl FromStr for Universe {
    type Err = CannotParseUniverse;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let mut galaxies = Vec::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {}
                    '#' => {
                        let galaxy = Galaxy::at(x, y);
                        galaxies.push(galaxy);
                    }
                    _ => return Err(CannotParseUniverse::UnexpectedChar(c)),
                }
            }
        }

        Ok(Self::with_galaxies(galaxies))
    }
}

use thiserror::Error;
#[derive(Error, Debug)]
enum CannotParseUniverse {
    #[error("Unexpected char: {0}")]
    UnexpectedChar(char),
}

use challenges_common::graph::Coord;
struct Galaxy {
    coord: Coord,
}

impl Galaxy {
    fn at(x: usize, y: usize) -> Self {
        Self {
            coord: Coord { x, y },
        }
    }
}

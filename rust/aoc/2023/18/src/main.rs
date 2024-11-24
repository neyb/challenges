use anyhow::*;
use challenges_common::geom::{Point, Polygon};
use challenges_common::graph::*;
use itertools::Itertools;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "18.txt"]);
    println!("{:?}", part1::run(&content).unwrap());
    println!("{:?}", part2::run(&content).unwrap());
}

mod part1;
mod part2;

struct DigPlan {
    digs: Vec<Dig>,
}

type Coord = challenges_common::graph::Coord<isize>;

impl DigPlan {
    fn path(&self) -> Result<Path> {
        let mut len = 0;
        let mut last_coord = Coord { x: 0, y: 0 };
        Path::new(
            self.digs
                .iter()
                .map(|dig| {
                    let next = last_coord.at_dist(dig.direction, dig.count as isize);
                    len += dig.count;
                    last_coord = next;
                    next
                })
                .collect(),
            len,
        )
    }
}

struct Path {
    path_elements: Vec<Coord>,
    len: usize,
}

impl Path {
    fn new(coords: Vec<Coord>, len: usize) -> Result<Self> {
        if coords.is_empty() {
            bail!("empty path")
        }
        Ok(Self {
            path_elements: coords,
            len,
        })
    }

    fn polygon(&self) -> Polygon<isize> {
        let points: Vec<Point<isize>> = self
            .path_elements
            .iter()
            .map(|&coord| coord.into())
            .collect();
        Polygon::new(points)
    }

    fn area(&self) -> usize {
        self.polygon().area::<isize>() as usize + self.len / 2 + 1
    }
}

struct Dig {
    direction: Direction,
    count: usize,
    #[allow(dead_code)]
    color: String,
}

impl std::str::FromStr for DigPlan {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let dig = s.lines().map(|line| line.parse()).try_collect()?;
        Ok(Self { digs: dig })
    }
}

impl std::str::FromStr for Dig {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, direction, distance, color) =
            lazy_regex::regex_captures!(r"(U|D|L|R) (\d+) \(#([0-9a-f]{6})\)", s)
                .ok_or_else(|| anyhow!("Cannot parse dig:{s}"))?;
        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => bail!("unknown direction {direction}"),
        };
        Ok(Self {
            direction,
            count: distance.parse()?,
            color: color.to_string(),
        })
    }
}

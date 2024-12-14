use anyhow::{anyhow, Error};
use challenges_common::graph::grid;
use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "14.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

type Unit = isize;
type Coord = grid::Coord<Unit>;
type Vec2 = grid::Vec2<Unit>;

struct Map {
    width: Unit,
    height: Unit,
}

impl Map {
    fn new(width: Unit, height: Unit) -> Self {
        Self { width, height }
    }

    fn safety_factor(&self, robots: &Robots) -> usize {
        let mid_x = self.width / 2;
        let mid_y = self.height / 2;
        let robots_by_quarter = robots.robots.iter().into_group_map_by(|robot| {
            (robot.position.x.cmp(&mid_x), robot.position.y.cmp(&mid_y))
        });

        use std::cmp::Ordering::*;
        robots_by_quarter
            .iter()
            .flat_map(|(cmps, robots)| match cmps {
                (Equal, _) | (_, Equal) => None,
                _ => Some(robots.len()),
            })
            .product()
    }
}

struct Robots {
    robots: Vec<Robot>,
}

impl Robots {
    fn r#move(&mut self, map: &Map) {
        for robot in &mut self.robots {
            robot.r#move(map);
        }
    }
}

struct Robot {
    position: Coord,
    speed: Vec2,
}

impl Robot {
    fn r#move(&mut self, map: &Map) {
        self.position += self.speed;
        self.position.x = self.position.x.rem_euclid(map.width);
        self.position.y = self.position.y.rem_euclid(map.height);
    }
}

impl FromStr for Robots {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let robots = s.lines().map(|line| line.parse()).try_collect()?;
        anyhow::Ok(Self { robots })
    }
}

impl FromStr for Robot {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        static REGEX2: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"p=(.+) v=(.+)").unwrap());

        let captures = REGEX2
            .captures(s)
            .ok_or_else(|| anyhow!("invalide robot format : \"{s}\""))?;
        let (_, [position, speed]) = captures.extract();

        let parse = |s: &str| {
            let res: Vec<Unit> = s.split(',').map(|s| s.parse()).try_collect()?;
            anyhow::Ok(
                res.into_iter()
                    .collect_tuple()
                    .ok_or_else(|| anyhow!("invalid format"))?,
            )
        };

        let (px, py) = parse(position)?;
        let (sx, sy) = parse(speed)?;

        anyhow::Ok(Self {
            position: Coord { x: px, y: py },
            speed: Vec2 { x: sx, y: sy },
        })
    }
}

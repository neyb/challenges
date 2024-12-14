use crate::*;
use anyhow::*;
use std::fmt::{Display, Formatter};

pub(crate) fn run(content: &str) -> Result<usize> {
    let robots: Robots = content.parse()?;

    let mut world = World {
        time_elapsed: 0,
        map: Map::new(101, 103),
        robots,
    };

    let mut last_best_score = 0;
    let mut best_last_turn = 0;
    for _ in 0..10_000 {
        world.wait();
        let score = world.score();
        if score > last_best_score {
            last_best_score = score;
            best_last_turn = world.time_elapsed;
            println!("");
            println!("{world}");
            println!("Time elapsed: {}, score: {}", world.time_elapsed, score);
        }
    }
    Ok(best_last_turn)
}

struct World {
    time_elapsed: usize,
    map: Map,
    robots: Robots,
}

impl World {
    fn wait(&mut self) {
        self.time_elapsed += 1;
        self.robots.r#move(&self.map);
    }

    fn score(&self) -> usize {
        let robots = &self.robots.robots;

        robots
            .iter()
            .filter(|robot| {
                robots.iter().any(|other| {
                    robot.position != other.position
                        && (other.position.x - robot.position.x).abs() <= 1
                        && (other.position.y - robot.position.y).abs() <= 1
                })
            })
            .count()
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let robot = self
                    .robots
                    .robots
                    .iter()
                    .find(|robot| robot.position == Coord { x, y });
                let c = match robot {
                    Some(_) => 'x',
                    None => ' ',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Result::Ok(())
    }
}

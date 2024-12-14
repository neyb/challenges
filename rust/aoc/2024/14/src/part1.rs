use crate::{Map, Robots};
use anyhow::*;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let mut robots: Robots = content.parse()?;
    let map = Map::new(101, 103);
    for _ in 0..100 {
        robots.r#move(&map);
    }
    Ok(map.safety_factor(&robots))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_solo_robot() {
        let mut robot: Robot = "p=2,4 v=2,-3".parse().unwrap();
        let map = Map::new(11, 7);

        robot.r#move(&map);
        assert_eq!(robot.position, Coord { x: 4, y: 1 });

        robot.r#move(&map);
        assert_eq!(robot.position, Coord { x: 6, y: 5 });

        robot.r#move(&map);
        assert_eq!(robot.position, Coord { x: 8, y: 2 });

        robot.r#move(&map);
        assert_eq!(robot.position, Coord { x: 10, y: 6 });

        robot.r#move(&map);
        assert_eq!(robot.position, Coord { x: 1, y: 3 });
    }

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "14-test.txt"]);
        let mut robots: Robots = content.parse().unwrap();
        let map = Map::new(11, 7);
        for _ in 0..100 {
            robots.r#move(&map);
        }

        assert_eq!(map.safety_factor(&robots), 12);
    }
}

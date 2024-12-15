use crate::{Move, Moves};
use anyhow::*;
use challenges_common::graph::{grid, CannotParseElementFromChar, Coord};
use challenges_common::MyIterTools;
use itertools::Itertools;
use std::fmt::Display;
use std::str::FromStr;

type Res = Unit;
pub(crate) fn run(content: &str) -> Result<Res> {
    let (mut map, moves) = parse(content)?;

    for r#move in moves.iter() {
        map.move_robot(r#move);
    }

    Ok(map.boxes_gps_sum())
}

fn parse(content: &str) -> Result<(Map, Moves)> {
    let (map, moves) = content
        .lines()
        .split(|line| line.is_empty())
        .map(|lines| lines.join("\n"))
        .collect_tuple()
        .ok_or_else(|| anyhow!("Cannot split input"))?;

    let map: Map = map.parse()?;
    let moves: Moves = moves.parse()?;

    Ok((map, moves))
}

type Unit = usize;
type Grid = grid::Grid<Item>;

struct Map {
    grid: Grid,
    robot: Coord,
}

impl Map {
    fn move_robot(&mut self, r#move: &Move) {
        match self.next_non_box(r#move) {
            Some((coord, item)) if item == Item::Empty => {
                *self.grid.get_mut(&self.robot).unwrap() = Item::Empty;
                self.robot = self.robot.try_at(r#move.0).unwrap();
                *self.grid.get_mut(&coord).unwrap() = Item::Box;
                *self.grid.get_mut(&self.robot).unwrap() = Item::Robot;
            }
            _ => {}
        }
    }

    fn next_non_box(&self, r#move: &Move) -> Option<(Coord, Item)> {
        let mut coord = self.robot.try_at(r#move.0)?;
        let mut item = self.grid.get(&coord)?;
        while item == &Item::Box {
            coord = coord.try_at(r#move.0)?;
            item = self.grid.get(&coord)?;
        }
        Some((coord, *item))
    }

    fn boxes_gps_sum(&self) -> Unit {
        self.grid
            .entries()
            .filter(|(_coord, &item)| item == Item::Box)
            .map(|(coord, _item)| coord.x + 100 * coord.y)
            .sum()
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Item {
    Empty,
    Wall,
    Box,
    Robot,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Item::*;
        let c = match self {
            Empty => '.',
            Wall => '#',
            Box => 'O',
            Robot => '@',
        };
        write!(f, "{c}")
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid: Grid = s.parse()?;
        let robot = grid
            .find(|item| item == &Item::Robot)
            .ok_or_else(|| anyhow!("No robot"))?;
        Ok(Self { grid, robot })
    }
}

impl TryFrom<char> for Item {
    type Error = CannotParseElementFromChar;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Item::*;

        match value {
            '.' => Result::Ok(Empty),
            '#' => Result::Ok(Wall),
            'O' => Result::Ok(Box),
            '@' => Result::Ok(Robot),
            _ => Err(value.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_test() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "15-test-small.txt"]);

        let (mut map, moves) = parse(&content).unwrap();

        for r#move in moves.iter() {
            println!("{}", map.grid);
            map.move_robot(r#move);
        }

        assert_eq!(run(&content).unwrap(), 2028);
    }

    #[test]
    fn big_test() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "15-test-big.txt"]);
        assert_eq!(run(&content).unwrap(), 10092);
    }
}

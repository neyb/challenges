use crate::{Move, Moves};
use anyhow::*;
use challenges_common::graph::{grid, Direction};
use challenges_common::MyIterTools;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
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

type Unit = isize;
type Coord = grid::Coord<Unit>;
type Grid = grid::Grid<Item, Unit>;

struct Map {
    items: Vec<Item>,
    robot_index: usize,
    location_index: HashMap<Coord, usize>,
}

impl Map {
    fn new(items: Vec<Item>) -> Result<Self> {
        let robot_index = items
            .iter()
            .position(|item| matches!(item, Item::Robot { .. }))
            .ok_or_else(|| anyhow!("Robot not found"))?;

        let location_index = items
            .iter()
            .enumerate()
            .flat_map(|(index, item)| item.coords().iter().map(move |&coord| (coord, index)))
            .collect();

        Ok(Self {
            items,
            robot_index,
            location_index,
        })
    }

    fn boxes_gps_sum(&self) -> Unit {
        self.items
            .iter()
            .filter(|item| matches!(item, Item::Box { .. }))
            .map(|item| {
                let Coord { x, y } = item.left_coord();
                x + 100 * y
            })
            .sum()
    }

    fn movable_items<'m>(&self, item: &Item, direction: &Direction) -> Option<HashSet<usize>> {
        return movable_items_rec(self, item, direction);

        fn movable_items_rec(
            map: &Map,
            item: &Item,
            direction: &Direction,
        ) -> Option<HashSet<usize>> {
            match item {
                Item::Wall { .. } => None,
                _ => {
                    let mut result = HashSet::new();

                    result.insert(map.location_index[item.left_coord()]);

                    for coord in item.coords_at(direction) {
                        if let Some(item) = map.get(&coord) {
                            result.extend(movable_items_rec(map, item, direction)?);
                        }
                    }

                    Some(result)
                }
            }
        }
    }

    fn move_robot(&mut self, r#move: &Move) {
        if let Some(item_indexes) = self.movable_items(self.robot(), &r#move.0) {
            for &i in item_indexes.iter() {
                for coord in self.items[i].coords() {
                    self.location_index.remove(coord);
                }
            }

            for i in item_indexes {
                let item = &mut self.items[i];
                item.r#move(&r#move.0);
                for coord in self.items[i].coords() {
                    self.location_index.insert(*coord, i);
                }
            }
        }
    }

    fn robot(&self) -> &Item {
        &self.items[self.robot_index]
    }

    fn get(&self, coord: &Coord) -> Option<&Item> {
        self.location_index
            .get(coord)
            .map(|&index| &self.items[index])
    }

    fn get_mut(&mut self, coord: &Coord) -> Option<&mut Item> {
        self.location_index
            .get(coord)
            .map(|&index| &mut self.items[index])
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Item {
    Wall { coord: Coord },
    Box { coords: [Coord; 2] },
    Robot { coord: Coord },
}

impl Item {
    fn left_coord(&self) -> &Coord {
        match self {
            Item::Wall { coord } => coord,
            Item::Box { coords } => &coords[0],
            Item::Robot { coord } => coord,
        }
    }

    fn coords(&self) -> &[Coord] {
        match self {
            Item::Wall { coord } => std::slice::from_ref(coord),
            Item::Box { coords } => coords,
            Item::Robot { coord } => std::slice::from_ref(coord),
        }
    }

    fn coords_at(&self, direction: &Direction) -> Vec<Coord> {
        match self {
            Item::Wall { coord } | Item::Robot { coord } => vec![coord.at(*direction)],
            Item::Box { coords } => match direction {
                Direction::Left => vec![coords[0].at(*direction)],
                Direction::Right => vec![coords[1].at(*direction)],
                Direction::Up | Direction::Down => {
                    vec![coords[0].at(*direction), coords[1].at(*direction)]
                }
            },
        }
    }

    fn r#move(&mut self, direction: &Direction) {
        match self {
            Item::Wall { .. } => {
                panic!("Cannot move wall")
            }
            Item::Box { coords } => {
                coords[0] = coords[0].at(*direction);
                coords[1] = coords[1].at(*direction);
            }
            Item::Robot { coord } => {
                *coord = coord.at(*direction);
            }
        }
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let item: Vec<Item> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    let coord = Coord {
                        x: x as isize,
                        y: y as isize,
                    };
                    let item_type = match c {
                        '.' => return None,
                        'O' => ItemType::Box,
                        '#' => ItemType::Wall,
                        '@' => ItemType::Robot,
                        _ => return Some(Err(anyhow!("Cannot parse item: {c}"))),
                    };

                    Some(Ok(ParsedItem { coord, item_type }))
                })
            })
            .map_ok(|parsed_item| parsed_item.dedup())
            .flatten_ok()
            .try_collect()?;

        Map::new(item)
    }
}

struct ParsedItem {
    coord: Coord,
    item_type: ItemType,
}

enum ItemType {
    Wall,
    Box,
    Robot,
}

impl ParsedItem {
    fn dedup(&self) -> Vec<Item> {
        let left = Coord {
            x: self.coord.x * 2,
            y: self.coord.y,
        };
        let right = Coord {
            x: self.coord.x * 2 + 1,
            y: self.coord.y,
        };

        match self.item_type {
            ItemType::Wall => vec![Item::Wall { coord: left }, Item::Wall { coord: right }],
            ItemType::Box => vec![Item::Box {
                coords: [left, right],
            }],
            ItemType::Robot => vec![Item::Robot { coord: left }],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn big_test() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "15-test-big.txt"]);
        assert_eq!(run(&content).unwrap(), 9021);
    }
}

#![allow(dead_code)]
use challenges_common::graph::{astar, Coord, Grid, Step};
use itertools::Itertools;

fn main() {
    let map = read_map(&["aoc", "2022", "12.txt"]);

    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn read_map(location: &[&str]) -> Map {
    let mut start = None;
    let mut exit = None;

    let grid = challenges_common::get_input_lines(location)
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    'S' => {
                        start = Some(Coord { x, y });
                        b'a'
                    }
                    'E' => {
                        exit = Some(Coord { x, y });
                        b'z'
                    }
                    _ => char as u8,
                })
                .collect_vec()
        })
        .into();
    Map {
        grid,
        start: start.unwrap(),
        exit: exit.unwrap(),
    }
}

fn part1(map: &Map) -> usize {
    let found_path = astar(
        map.start,
        |from| {
            map.accessible_nodes_from(from)
                .map(|coord| Step {
                    to: coord,
                    additional_cost: 1,
                })
        },
        |coord| coord == &map.exit,
        |coord| coord.manhattan_dist_to(&map.exit),
    );

    found_path.unwrap().cost
}

fn part2(map: &Map) -> usize {
    astar(
        map.exit,
        |to| {
            map.node_accessing(to).map(|coord| Step {
                additional_cost: 1,
                to: coord,
            })
        },
        |coord| map.grid.at(coord).unwrap() == &b'a',
        |_| 0,
    )
    .unwrap()
    .cost
}

struct Map {
    grid: Grid<u8>,
    start: Coord,
    exit: Coord,
}

impl Map {
    fn edges(&self) -> Vec<(Coord, Coord)> {
        self.grid
            .coords()
            .flat_map(|from| {
                self.accessible_nodes_from(&from)
                    .map({
                        move |to_coord| (from, to_coord)
                    })
                    .collect_vec()
            })
            .collect()
    }

    fn accessible_nodes_from<'a>(&'a self, from: &Coord) -> impl Iterator<Item = Coord> + 'a {
        let &from_height = self.grid.at(from).unwrap();
        self.neightbours_with(from, move |(_, &to_height)| (from_height + 1) >= to_height)
    }

    fn node_accessing<'a>(&'a self, to: &Coord) -> impl Iterator<Item = Coord> + 'a {
        let &to_height = self.grid.at(to).unwrap();
        self.neightbours_with(to, move |(_, &from_height)| from_height + 1 >= to_height)
    }

    fn neightbours_with<'a>(
        &'a self,
        from: &Coord,
        filter: impl Fn(&(Coord, &u8)) -> bool + 'a,
    ) -> impl Iterator<Item = Coord> + 'a {
        self.grid
            .neighbours(from)
            .filter(filter)
            .map(|(to_coord, _)| to_coord)
    }
}

struct Node {
    coord: Coord,
    height: u8,
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::*;

    #[test]
    fn given_test_edges_from_00_should_be_01_and_10() {
        let map = read_map(&["aoc", "2022", "12-test.txt"]);
        let nodes = map
            .accessible_nodes_from(&Coord { x: 0, y: 0 })
            .collect::<HashSet<_>>();

        assert_eq!(
            nodes,
            HashSet::from([Coord { x: 1, y: 0 }, Coord { x: 0, y: 1 }])
        )
    }

    #[test]
    fn given_test_part1_edges() {
        let map = read_map(&["aoc", "2022", "12-test.txt"]);
        let edges = map.edges();
        assert!(edges.contains(&(Coord { x: 0, y: 0 }, Coord { x: 0, y: 1 })));
        assert!(edges.contains(&(Coord { x: 0, y: 0 }, Coord { x: 1, y: 0 })));
    }

    #[test]
    fn given_test_part1() {
        let map = read_map(&["aoc", "2022", "12-test.txt"]);
        assert_eq!(part1(&map), 31)
    }
}

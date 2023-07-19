use std::collections::HashSet;
use std::iter::successors;

use itertools::Itertools;

fn main() {
    let map = challenges_common::get_input_lines(&["aoc", "2022", "8.txt"])
        .map(|line| line.chars().collect_vec().into_iter())
        .map(|chars| chars.map(|c| c.to_digit(10).unwrap()));
    let grid = Grid::from(map);
    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
}

fn part1(grid: &Grid<Height>) -> usize {
    visible_coord(grid).len()
}

fn visible_coord(grid: &Grid<Height>) -> HashSet<Coord> {
    let line = |y: usize| (0..grid.width()).map(move |x| Coord { x, y });
    let col = |x: usize| (0..grid.height()).map(move |y| Coord { x, y });

    let mut nodes: HashSet<Coord> = HashSet::new();
    nodes.extend((0..grid.height()).flat_map(|y| keep_visible(line(y), grid)));
    nodes.extend((0..grid.height()).flat_map(|y| keep_visible(line(y).rev(), grid)));
    nodes.extend((0..grid.width()).flat_map(|x| keep_visible(col(x), grid)));
    nodes.extend((0..grid.width()).flat_map(|x| keep_visible(col(x).rev(), grid)));
    nodes
}

fn keep_visible(coords: impl Iterator<Item = Coord>, grid: &Grid<Height>) -> Vec<Coord> {
    let mut max: Option<Height> = None;

    coords
        .filter(|coord| {
            let &height = grid.at(coord).unwrap();
            match max {
                Some(max_value) if height <= max_value => false,
                _ => {
                    max = Some(height);
                    true
                }
            }
        })
        .collect()
}

fn part2(grid: &Grid<Height>) -> usize {
    grid.coords()
        .map(|coord| scenic_score(grid, &coord))
        .max()
        .unwrap()
}

fn scenic_score(grid: &Grid<Height>, coord: &Coord) -> usize {
    let start_height = grid.at(coord).unwrap();
    let view_distance = |direction: &Direction| {
        let mut can_see_more = true;
        grid.coords_from(coord, direction)
            .skip(1)
            .take_while(|(_, height)| {
                let take = can_see_more;
                can_see_more = height < start_height;
                take
            })
            .count()
    };

    view_distance(&Direction::Up)
        * view_distance(&Direction::Down)
        * view_distance(&Direction::Right)
        * view_distance(&Direction::Left)
}

type Height = u32;

struct Grid<N> {
    width: usize,
    content: Vec<N>,
}

impl<N: Copy> Grid<N> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.content.len() / self.width
    }

    fn at(&self, coord: &Coord) -> Option<&N> {
        if coord.x < self.width {
            self.content.get(coord.x + self.width * coord.y)
        } else {
            None
        }
    }

    fn coords(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.width()).flat_map(|x| (0..self.height()).map(move |y| Coord { x, y }))
    }

    fn coords_from<'d: 's, 's>(
        &'s self,
        coord: &Coord,
        direction: &'d Direction,
    ) -> impl Iterator<Item = (Coord, N)> + 's {
        let with_height = |coord: Coord| self.at(&coord).map(|height| (coord, *height));

        successors(
            with_height(coord.clone()),
            move |(coord, _)| match direction {
                Direction::Up => {
                    if coord.y != 0 {
                        with_height(Coord {
                            x: coord.x,
                            y: coord.y - 1,
                        })
                    } else {
                        None
                    }
                }
                Direction::Down => with_height(Coord {
                    x: coord.x,
                    y: coord.y + 1,
                }),
                Direction::Right => with_height(Coord {
                    x: coord.x + 1,
                    y: coord.y,
                }),
                Direction::Left => {
                    if coord.x != 0 {
                        with_height(Coord {
                            x: coord.x - 1,
                            y: coord.y,
                        })
                    } else {
                        None
                    }
                }
            },
        )
    }
}

impl<IntoIt, N> From<IntoIt> for Grid<N>
where
    IntoIt: Iterator,
    IntoIt::Item: Iterator<Item = N>,
{
    fn from(into_it: IntoIt) -> Self {
        let mut width = None;

        let content = into_it
            .into_iter()
            .flat_map(|line| match width {
                None => {
                    let line = line.collect_vec();
                    width = Some(line.len());
                    line
                }
                Some(_) => line.collect_vec(),
            })
            .collect();

        Self {
            width: width.unwrap_or(0),
            content,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use crate::*;

    #[test]
    fn test_keep_visible_1_2_3() {
        let grid = Grid::from(vec![vec![1, 2, 3].into_iter()].into_iter());

        assert_eq!(
            keep_visible(
                vec![
                    Coord { x: 0, y: 0 },
                    Coord { x: 1, y: 0 },
                    Coord { x: 2, y: 0 },
                ]
                .into_iter(),
                &grid,
            ),
            vec![
                Coord { x: 0, y: 0 },
                Coord { x: 1, y: 0 },
                Coord { x: 2, y: 0 },
            ]
        )
    }

    #[test]
    fn test_keep_visible_3_2_1() {
        let grid = Grid::from(vec![vec![1, 2, 3].into_iter()].into_iter());

        assert_eq!(
            keep_visible(
                vec![
                    Coord { x: 2, y: 0 },
                    Coord { x: 1, y: 0 },
                    Coord { x: 0, y: 0 },
                ]
                .into_iter(),
                &grid,
            ),
            vec![Coord { x: 2, y: 0 }]
        )
    }

    fn given_grid() -> Grid<u32> {
        let map = challenges_common::get_input_lines(&["aoc", "2022", "8-test.txt"])
            .map(|line| line.chars().collect_vec().into_iter())
            .map(|chars| chars.map(|c| c.to_digit(10).unwrap()));
        Grid::from(map)
    }

    #[test]
    fn given_test_part1() {
        let grid = given_grid();
        assert_eq!(part1(&grid), 21);
    }

    #[test]
    fn given_test_from_2_1() {
        let grid = given_grid();
        assert_eq!(scenic_score(&grid, &Coord { x: 2, y: 1 }), 4)
    }

    #[test]
    fn given_test_part2() {
        let grid = given_grid();
        assert_eq!(part2(&grid), 8)
    }
}

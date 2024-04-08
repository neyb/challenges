use crate::Direction::{East, North, South, West};
use anyhow::{Error, Result};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() {
    let elves = parse(&["aoc", "2022", "23.txt"]).unwrap();
    println!("part1: {}", part1(&elves));
    println!("part2: {}", part2(&elves));
}

fn part1(elves: &ElvesState) -> usize {
    let mut elves = elves.clone();
    for _ in 0..10 {
        elves.run_turn();
    }
    let (x_range, y_range) = elves.ranges();

    let area = x_range.len() * y_range.len();
    area - elves.elves.len()
}

fn part2(elves: &ElvesState) -> u16 {
    let mut elves = elves.clone();
    while elves.run_turn() {}
    elves.passed_turns
}

fn parse(path: &[&str]) -> Result<ElvesState> {
    challenges_common::get_input_content(path).parse()
}

type Unit = i16;

#[derive(Debug, PartialEq, Clone)]
struct ElvesState {
    elves: HashSet<ElfState>,
    passed_turns: u16,
}

impl ElvesState {
    fn run_turn(&mut self) -> bool {
        let mut has_move = false;
        for (desired, elves) in self.wishes() {
            if let [elf] = &elves[..] {
                has_move = true;
                let mut elf = self.elves.take(elf).unwrap();
                elf.0 = desired;
                self.elves.insert(elf);
            }
        }

        self.passed_turns += 1;

        has_move
    }

    fn wishes(&self) -> HashMap<Coord, Vec<ElfState>> {
        let mut wishes: HashMap<Coord, Vec<ElfState>> = HashMap::new();

        for elf_state in &self.elves {
            let neighbourhood = self.neighbourhood(&elf_state.0);
            if let Some(wished_direction) =
                elf_state.wished_direction(&neighbourhood, self.passed_turns)
            {
                wishes
                    .entry(elf_state.0.at(&wished_direction))
                    .or_insert_with(Vec::new)
                    .push(elf_state.clone())
            }
        }

        wishes
    }

    fn has_an_elf_in(&self, coord: &Coord) -> bool {
        self.elves.contains(&ElfState(coord.clone()))
    }

    fn neighbourhood(&self, coord: &Coord) -> Neighbourhood {
        let n = coord.at(&North);
        let s = coord.at(&South);
        let e = coord.at(&East);
        let w = coord.at(&West);
        let nw = n.at(&West);
        let ne = n.at(&East);
        let sw = s.at(&West);
        let se = s.at(&East);

        Neighbourhood {
            all: [
                self.has_an_elf_in(&nw),
                self.has_an_elf_in(&n),
                self.has_an_elf_in(&ne),
                self.has_an_elf_in(&w),
                self.has_an_elf_in(&e),
                self.has_an_elf_in(&sw),
                self.has_an_elf_in(&s),
                self.has_an_elf_in(&se),
            ],
        }
    }

    fn ranges(&self) -> (RangeInclusive<Unit>, RangeInclusive<Unit>) {
        let min_x = self.elves.iter().map(|elf| elf.0.x).min().unwrap();
        let max_x = self.elves.iter().map(|elf| elf.0.x).max().unwrap();
        let min_y = self.elves.iter().map(|elf| elf.0.y).min().unwrap();
        let max_y = self.elves.iter().map(|elf| elf.0.y).max().unwrap();
        (min_x..=max_x, min_y..=max_y)
    }
}

impl Display for ElvesState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x_range, y_range) = self.ranges();
        for y in y_range {
            for x in x_range.clone() {
                f.write_char(if self.has_an_elf_in(&Coord::new(x, y)) {
                    '#'
                } else {
                    '.'
                })?
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct ElfState(Coord);

impl ElfState {
    #[allow(unused)]
    fn new(x: Unit, y: Unit) -> Self {
        Self(Coord::new(x, y))
    }

    fn wished_direction(
        &self,
        neighbourhood: &Neighbourhood,
        passed_turns: u16,
    ) -> Option<Direction> {
        if neighbourhood.is_empty() {
            None
        } else {
            [North, South, West, East]
                .repeat(2)
                .into_iter()
                .skip(passed_turns as usize % 4)
                .take(4)
                .find(|direction| {
                    CROSS_DIRECTIONS
                        .iter()
                        .filter(|cd| cd.includes(direction))
                        .all(|cd| !neighbourhood.has(cd))
                })
        }
    }
}

struct Neighbourhood {
    all: [bool; 8],
}

impl Neighbourhood {
    fn is_empty(&self) -> bool {
        self.all.iter().all(|&b| !b)
    }

    fn has(&self, direction: &CrossDirection) -> bool {
        match direction {
            CrossDirection::NorthWest => self.all[0],
            CrossDirection::North => self.all[1],
            CrossDirection::NorthEast => self.all[2],
            CrossDirection::West => self.all[3],
            CrossDirection::East => self.all[4],
            CrossDirection::SouthWest => self.all[5],
            CrossDirection::South => self.all[6],
            CrossDirection::SouthEast => self.all[7],
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Coord {
    x: Unit,
    y: Unit,
}

impl Coord {
    fn new(x: Unit, y: Unit) -> Self {
        Self { x, y }
    }

    fn at(&self, direction: &Direction) -> Self {
        use Direction::*;

        match direction {
            North => Self {
                x: self.x,
                y: self.y - 1,
            },
            South => Self {
                x: self.x,
                y: self.y + 1,
            },
            West => Self {
                x: self.x - 1,
                y: self.y,
            },
            East => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

const CROSS_DIRECTIONS: [CrossDirection; 8] = {
    use CrossDirection::*;

    [
        NorthWest, North, NorthEast, West, East, SouthWest, South, SouthEast,
    ]
};

enum CrossDirection {
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl CrossDirection {
    fn includes(&self, direction: &Direction) -> bool {
        matches!(
            (direction, self),
            (
                North,
                CrossDirection::North | CrossDirection::NorthWest | CrossDirection::NorthEast,
            ) | (
                South,
                CrossDirection::South | CrossDirection::SouthWest | CrossDirection::SouthEast,
            ) | (
                West,
                CrossDirection::SouthWest | CrossDirection::West | CrossDirection::NorthWest,
            ) | (
                East,
                CrossDirection::East | CrossDirection::NorthEast | CrossDirection::SouthEast,
            )
        )
    }
}

impl FromStr for ElvesState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            elves: s
                .lines()
                .enumerate()
                .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
                .filter(|(_, _, c)| *c == '#')
                .map(|(x, y, _)| ElfState(Coord::new(x as Unit, y as Unit)))
                .collect(),
            passed_turns: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    macro_rules! elves {
        ($($x:expr, $y:expr);*) => {
            {
                let mut elves = HashSet::new();
                $(elves.insert(ElfState::new($x, $y));)*
                ElvesState {elves, passed_turns:0}
            }
        };
        ($($x:expr, $y:expr);* => $passed_turns:expr) => {
            {
                let mut elves = HashSet::new();
                $(elves.insert(ElfState::new($x, $y));)*
                ElvesState {elves, passed_turns:$passed_turns}
            }
        }
    }

    #[test]
    fn should_be_able_to_parse() {
        let elves = parse(&["aoc", "2022", "23-test.txt"]).unwrap();
        assert_eq!(
            elves,
            elves!(
                4, 0;
                2, 1;
                3, 1;
                4, 1;
                6, 1;
                0, 2;
                4, 2;
                6, 2;
                1, 3;
                5, 3;
                6, 3;
                0, 4;
                2, 4;
                3, 4;
                4, 4;
                0, 5;
                1, 5;
                3, 5;
                5, 5;
                6, 5;
                1, 6;
                4, 6
            )
        );
    }

    #[test]
    fn alone_elf_does_not_move() {
        let mut elves = elves!(0, 0);
        elves.run_turn();
        assert_eq!(elves, elves!(0, 0 => 1))
    }

    #[test]
    fn run_one_turn_with_that_cannot_go_bot() {
        let mut elves = elves!(0,0 ; 0,1);
        elves.run_turn();
        assert_eq!(elves, elves!(0,-1 ; 0,2 => 1))
    }

    #[test]
    fn run_one_turn_3() {
        let mut elves = elves!(0,0 ; 0,1 ; 0,2);
        elves.run_turn();
        assert_eq!(elves, elves!(0,-1 ; -1,1 ; 0,3 => 1))
    }

    #[test]
    fn run_turns() {
        let mut elves = parse(&["aoc", "2022", "23-test.txt"]).unwrap();
        elves.run_turn();
        assert_eq!(
            elves.to_string(),
            "\
.....#...
...#...#.
.#..#.#..
.....#..#
..#.#.##.
#..#.#...
#.#.#.##.
.........
..#..#...
",
            "end of first turn"
        );

        elves.run_turn();
        // end of turn 2
        assert_eq!(
            elves.to_string(),
            "\
......#....
...#.....#.
..#..#.#...
......#...#
..#..#.#...
#...#.#.#..
...........
.#.#.#.##..
...#..#....
",
            "end of turn 2"
        );

        elves.run_turn();
        elves.run_turn();
        elves.run_turn();

        assert_eq!(
            elves.to_string(),
            "\
......#....
...........
.#..#.....#
........#..
.....##...#
#.#.####...
..........#
...##..#...
.#.........
.........#.
...#..#....
"
        );

        elves.run_turn();
        elves.run_turn();
        elves.run_turn();
        elves.run_turn();
        elves.run_turn();

        assert_eq!(
            elves.to_string(),
            "\
......#.....
..........#.
.#.#..#.....
.....#......
..#.....#..#
#......##...
....##......
.#........#.
...#.#..#...
............
...#..#..#..
"
        )
    }

    #[test]
    fn given_test() {
        let elves = parse(&["aoc", "2022", "23-test.txt"]).unwrap();
        assert_eq!(part1(&elves), 110)
    }

    #[test]
    fn given_test_part2() {
        let elves = parse(&["aoc", "2022", "23-test.txt"]).unwrap();
        assert_eq!(part2(&elves), 20)
    }
}

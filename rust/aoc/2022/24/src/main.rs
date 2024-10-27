use anyhow::{Error, Result};
use challenges_common::graph::{astar, Step};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::str::FromStr;

fn main() {
    let map_with_state = parse(&["aoc", "2022", "24.txt"]).unwrap();
    println!("part1: {}", part1(&map_with_state));
    println!("part2: {}", part2(&map_with_state));
}

type Unit = u16;
type Count = u16;

fn parse(path: &[&str]) -> Result<MapWithState> {
    let content = challenges_common::get_input_content(path);
    let map = content.parse()?;
    let wind_state = content.parse()?;
    let state = State::new(&map, &wind_state);
    Ok(MapWithState { state, map })
}

fn part1(map_with_state: &MapWithState) -> Count {
    let mut cache = WindStates::new(map_with_state.state.winds.clone());
    let map_with_state = map_with_state.clone();
    let map_with_state = map_with_state.into_exit(&mut cache);
    map_with_state.state.turn
}

fn part2(map_with_state: &MapWithState) -> Count {
    let mut cache = WindStates::new(map_with_state.state.winds.clone());
    let map_with_state = map_with_state.clone();

    let map_with_state = map_with_state.into_exit(&mut cache);
    let map_with_state = map_with_state.into_entry(&mut cache);
    let map_with_state = map_with_state.into_exit(&mut cache);

    map_with_state.state.turn
}

struct WindStates {
    cache: Vec<Rc<WindState>>,
}

impl WindStates {
    fn new(init: Rc<WindState>) -> Self {
        Self { cache: vec![init] }
    }

    fn at(&mut self, turn: Count, map: &Map) -> Rc<WindState> {
        let mut last = self.cache.iter().last().unwrap().clone();
        for _ in self.cache.len()..=turn as usize {
            last = Rc::new(last.next(map));
            self.cache.push(last.clone());
        }
        self.cache.get(turn as usize).unwrap().clone()
    }
}

#[derive(Clone)]
struct MapWithState {
    map: Map,
    state: State,
}

impl MapWithState {
    fn into_exit(self, winds_states: &mut WindStates) -> Self {
        let map = &self.map;

        let state = astar(
            self.state,
            |state| {
                state
                    .nexts(map, winds_states)
                    .into_iter()
                    .map(|state| Step {
                        to: state,
                        additional_cost: 1,
                    })
            },
            |state| state.position == map.exit,
            |state| map.cardinal_distance_to_exit(&state.position),
        )
        .unwrap()
        .nodes
        .into_iter()
        .last()
        .unwrap();

        MapWithState { state, ..self }
    }

    fn into_entry(self, winds_states: &mut WindStates) -> Self {
        let map = &self.map;

        let state = astar(
            self.state,
            |state| {
                state
                    .nexts(map, winds_states)
                    .into_iter()
                    .map(|state| Step {
                        to: state,
                        additional_cost: 1,
                    })
            },
            |state| state.position == map.entry,
            |state| map.cardinal_distance_to_entry(&state.position),
        )
        .unwrap()
        .nodes
        .into_iter()
        .last()
        .unwrap();

        MapWithState { state, ..self }
    }
}

impl Display for MapWithState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.map.max_y {
            for x in 0..=self.map.max_x {
                let coord = Coord { x, y };
                let char = match self.state.winds.winds.get(&coord) {
                    Some(winds) if winds.len() > 1 => {
                        winds.len().to_string().chars().next().unwrap()
                    }
                    Some(winds) => match winds[0].direction {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    },
                    _ if self.state.position == coord => 'E',
                    _ if self.map.is_wall(&coord) => '#',
                    _ => '.',
                };

                f.write_char(char)?
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[derive(Clone)]
struct Map {
    entry: Coord,
    exit: Coord,
    max_x: Unit,
    max_y: Unit,
}

impl Map {
    fn cardinal_distance_to_entry(&self, coord: &Coord) -> Unit {
        (coord.x - self.entry.x) + (coord.y - self.entry.y)
    }

    fn cardinal_distance_to_exit(&self, coord: &Coord) -> Unit {
        (self.exit.x - coord.x) + (self.exit.y - coord.y)
    }

    fn reflect_wind_coord(&self, coord: &mut Coord) {
        if coord.x == 0 {
            coord.x = self.max_x - 1
        }
        if coord.y == 0 {
            coord.y = self.max_y - 1
        }
        if coord.x == self.max_x {
            coord.x = 1
        }
        if coord.y == self.max_y {
            coord.y = 1
        }
    }

    fn is_wall(&self, coord: &Coord) -> bool {
        &self.entry != coord
            && &self.exit != coord
            && (coord.x == 0 || coord.y == 0 || coord.x == self.max_x || coord.y == self.max_y)
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
    position: Coord,
    winds: Rc<WindState>,
    turn: Count,
}

impl State {
    fn new(map: &Map, wind_state: &WindState) -> Self {
        Self {
            position: map.entry.clone(),
            winds: Rc::new(wind_state.clone()),
            turn: 0,
        }
    }

    fn nexts(&self, map: &Map, wind_states: &mut WindStates) -> Vec<Self> {
        use Direction::*;

        let turn = self.turn + 1;
        let wind_state = wind_states.at(turn, map);

        let same_position = self.position.clone();
        vec![
            same_position.at(&Up),
            same_position.at(&Down),
            same_position.at(&Left),
            same_position.at(&Right),
            Some(same_position),
        ]
        .into_iter()
        .flatten()
        .filter(|position| !map.is_wall(position))
        .filter(|position| !wind_state.winds.contains_key(position))
        .map(|position| Self {
            turn,
            winds: wind_state.clone(),
            position,
        })
        .collect()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct WindState {
    winds: HashMap<Coord, Vec<Wind>>,
}

impl WindState {
    fn next(&self, map: &Map) -> Self {
        let winds = self
            .winds
            .iter()
            .flat_map(|(coord, winds)| winds.iter().map(move |wind| (coord, wind)))
            .filter_map(|(coord, wind)| {
                coord.at(&wind.direction).map(|mut coord| {
                    map.reflect_wind_coord(&mut coord);
                    (coord, wind)
                })
            })
            .fold(HashMap::new(), |mut map:HashMap<_,Vec<_>>, (coord, wind)| {
                map.entry(coord).or_default().push(wind.clone());
                map
            });

        Self { winds }
    }
}

impl Hash for WindState {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Coord {
    x: Unit,
    y: Unit,
}

impl Coord {
    #[allow(unused)]
    fn new(x: Unit, y: Unit) -> Self {
        Self { x, y }
    }

    fn at(&self, direction: &Direction) -> Option<Self> {
        let mut result = self.clone();

        use Direction::*;
        match direction {
            Up => result.y = result.y.checked_sub(1)?,
            Down => result.y += 1,
            Left => result.x -= 1,
            Right => result.x += 1,
        }
        Some(result)
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Wind {
    direction: Direction,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for WindState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut winds = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if let Some(wind) = {
                    use Direction::*;

                    let direction = match char {
                        'v' => Some(Down),
                        '^' => Some(Up),
                        '<' => Some(Left),
                        '>' => Some(Right),
                        _ => None,
                    };

                    direction.map(|direction| Wind { direction })
                } {
                    let coord = Coord {
                        x: x as Unit,
                        y: y as Unit,
                    };
                    winds.entry(coord).or_insert_with(Vec::new).push(wind)
                }
            }
        }

        Ok(Self { winds })
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut max_x = 0 as Unit;
        let mut max_y = 0 as Unit;

        for (y, line) in s.lines().enumerate() {
            max_y = max_y.max(y as Unit);
            for (x, _) in line.chars().enumerate() {
                max_x = max_x.max(x as Unit);
            }
        }

        let entry = Coord { x: 1, y: 0 };
        let exit = Coord {
            x: max_x - 1,
            y: max_y,
        };

        Ok(Self {
            max_x,
            max_y,
            entry,
            exit,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn init_given_test_display() {
        let map_with_state = parse(&["aoc", "2022", "24-test.txt"]).unwrap();
        assert_eq!(
            map_with_state.to_string(),
            "\
            #E######\n\
            #>>.<^<#\n\
            #.<..<<#\n\
            #>v.><>#\n\
            #<^v^^>#\n\
            ######.#\n\
            "
        )
    }

    #[test]
    fn display_after_one_turn() {
        let map_with_state = parse(&["aoc", "2022", "24-test.txt"]).unwrap();
        let mut wind_states = WindStates::new(map_with_state.state.winds.clone());
        let nexts: Vec<_> = map_with_state
            .state
            .nexts(&map_with_state.map, &mut wind_states)
            .into_iter()
            .map(|state| MapWithState {
                map: map_with_state.map.clone(),
                state,
            })
            .collect();

        assert_eq!(nexts.len(), 2);
        assert_eq!(
            nexts[0].to_string(),
            "\
            #.######\n\
            #E>3.<.#\n\
            #<..<<.#\n\
            #>2.22.#\n\
            #>v..^<#\n\
            ######.#\n\
            "
        );
        assert_eq!(
            nexts[1].to_string(),
            "\
            #E######\n\
            #.>3.<.#\n\
            #<..<<.#\n\
            #>2.22.#\n\
            #>v..^<#\n\
            ######.#\n\
            "
        );
    }

    #[test]
    fn given_test_part1() {
        let map_with_state = parse(&["aoc", "2022", "24-test.txt"]).unwrap();
        assert_eq!(part1(&map_with_state), 18)
    }

    #[test]
    fn given_test_part2() {
        let map_with_state = parse(&["aoc", "2022", "24-test.txt"]).unwrap();
        assert_eq!(part2(&map_with_state), 54)
    }

    #[test]
    #[ignore]
    fn part1_is_correct() {
        let map_with_state = parse(&["aoc", "2022", "24.txt"]).unwrap();
        assert_eq!(part1(&map_with_state), 292)
    }

    #[test]
    #[ignore]
    fn part2_is_correct() {
        let map_with_state = parse(&["aoc", "2022", "24.txt"]).unwrap();
        assert_eq!(part2(&map_with_state), 816)
    }
}

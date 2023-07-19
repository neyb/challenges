use std::collections::HashMap;

use anyhow::{anyhow, Result};
use lazy_regex::regex_captures;

use challenges_common::graph::{astar, Step};

fn main() {
    let map = parse(&["aoc", "2022", "16.txt"]).unwrap();
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn parse<'map>(path: &[&str]) -> Result<Map> {
    let mut hasher = ValveIdHasher::new();
    challenges_common::get_input_lines(path)
        .map(|line| parse_valve_data(&line, &mut hasher))
        .collect::<Result<Vec<_>>>()
        .map(move |valves_data| Map::from(valves_data, hasher))
}

mod part1;

fn part1(map: &Map) -> u32 {
    map.get_max_released::<part1::StatePart1>(30).unwrap()
}

mod part2;

fn part2(map: &Map) -> u32 {
map.get_max_released::<part2::StatePart2>(26).unwrap()
}

type ValveId = usize;

#[derive(PartialEq, Eq, Debug)]
struct ValveData {
    id: ValveId,
    flow: u32,
    tunnels_to: Vec<ValveId>,
}

#[derive(PartialEq, Debug)]
struct Map {
    hasher: ValveIdHasher,
    valves: HashMap<ValveId, ValveData>,
}

#[derive(PartialEq, Debug, Clone)]
struct Move<'map> {
    cost: u8,
    target: &'map ValveId,
}

impl Map {
    fn from(valves: Vec<ValveData>, hasher: ValveIdHasher) -> Self {
        valves.into_iter().fold(
            Map {
                hasher,
                valves: HashMap::new(),
            },
            |mut map, valve_data| {
                map.valves
                    .entry(valve_data.id)
                    .or_insert(valve_data);
                map
            },
        )
    }

    fn get(&self, id: &ValveId) -> Result<&ValveData> {
        self.valves
            .get(id)
            .ok_or_else(|| anyhow!("searching a non existing valve: {:?}", id))
    }

    fn all_moves(&self) -> HashMap<&ValveId, Vec<Move>> {
        return self
            .valves
            .keys()
            .map(|start| (start, moves_from(start, self)))
            .collect();

        fn moves_from<'a>(start: &'a ValveId, map: &'a Map) -> Vec<Move<'a>> {
            map.valves
                .iter()
                .filter_map(|(id, data)| if data.flow > 0 { Some(id) } else { None })
                .filter_map(|target| {
                    astar(
                        start,
                        |&from| {
                            map.get(from).unwrap().tunnels_to.iter().map(|target| Step {
                                to: target,
                                additional_cost: 1,
                            })
                        },
                        |&valve| valve == target,
                        |_valve| 0,
                    )
                })
                .map(|path| Move {
                    target: path.nodes.last().unwrap(),
                    cost: path.cost + 1,
                })
                .collect()
        }
    }

    fn id_of(&self, name: impl ToString) -> Result<&ValveId> {
        self.hasher
            .id_of(name.to_string())
            .ok_or_else(|| anyhow!("no such entry like {}", name.to_string()))
    }

    fn valves_to_open(&self) -> Vec<&ValveId> {
        self.valves
            .iter()
            .filter(|(_id, data)| data.flow > 0)
            .map(|(id, _data)| id)
            .collect()
    }

    fn get_max_released<'map: 'state, 'state, S: Startable<'map> + State + 'map>(
        &'map self,
        timer: u8,
    ) -> Result<u32> {
        let all_moves = self.all_moves();
        let s = S::starting_state(self)?;
        self.rec_explore(Box::new(s), timer, &all_moves)
    }

    fn rec_explore(
        & self,
        from_state: Box<dyn State+'_>,
        timer: u8,
        all_moves: & HashMap<& ValveId, Vec<Move<>>>,
    ) -> Result<u32> {
        from_state.nexts(self, timer, all_moves)?.try_fold(
            from_state.released_pressure_at(timer),
            |max, next_state| {
                next_state.and_then(|next_state| {
                    self.rec_explore(next_state, timer, all_moves)
                        .map(|curr_max| max.max(curr_max))
                })
            },
        )
    }
}

#[derive(PartialEq, Debug, Default, Clone)]
struct ValveIdHasher {
    nb_created: usize,
    id_by_name: HashMap<String, usize>,
}

impl ValveIdHasher {
    fn new() -> Self {
        Self::default()
    }

    fn id_of_or_create(&mut self, name: impl ToString) -> &ValveId {
        self.id_by_name.entry(name.to_string()).or_insert_with(|| {
            let next_val = self.nb_created;
            self.nb_created += 1;
            next_val
        })
    }

    fn id_of(&self, name: impl ToString) -> Option<&ValveId> {
        self.id_by_name.get(&name.to_string())
    }
}

fn parse_valve_data(input: &str, hasher: &mut ValveIdHasher) -> Result<ValveData> {
    let (_, name, flow_rate, tunnels) = regex_captures!(
            r#"Valve ([[:alpha:]]+) has flow rate=(\d+); tunnels? leads? to valves? ((?:[[:alpha:]]+(?:, )?)+)"#,
            input
        ).ok_or_else(|| anyhow!("{:?} does not match the pattern", input))?;

    let tunnels_to = tunnels
        .split(", ")
        .map(|name| *hasher.id_of_or_create(name))
        .collect();

    Ok(ValveData {
        id: *hasher.id_of_or_create(name),
        flow: flow_rate.parse()?,
        tunnels_to,
    })
}

trait Startable<'map>: Sized {
    fn starting_state(map: &'map Map) -> Result<Self>;
}

trait State {
    fn nexts<'a>(
        &'a self,
        map: &'a Map,
        timer: u8,
        all_moves: &'a HashMap<&'a ValveId, Vec<Move<'a>>>,
    ) -> Result<Box<dyn Iterator<Item = Result<Box<dyn State+'a>>> + 'a>>;
    fn released_pressure_at(&self, time: u8) -> u32;
}

#[cfg(test)]
mod test {
    use crate::*;

    macro_rules! map {
        ($hasher:expr, $($valve:expr)*) => {Map::from(vec![$($valve,)*], $hasher)};
    }

    macro_rules! valve {
        ($hasher:expr, $id:expr;$flow_rate:expr => $($target:expr),*) => {
            ValveData {
                id: *$hasher.id_of_or_create($id),
                flow:$flow_rate,
                tunnels_to: vec![
                    $(*$hasher.id_of_or_create($target),)*
                ]
            }
        };
    }

    #[test]
    fn creating_a_map_with_macro() {
        let mut hasher = ValveIdHasher::new();
        let map_from_macro =
            map!(hasher, valve!(&mut hasher, "AA";1 => "BB") valve!(&mut hasher,"BB";2 => "CC"));

        let mut hasher = ValveIdHasher::new();
        let map = Map::from(
            vec![
                ValveData {
                    id: *hasher.id_of_or_create("AA"),
                    flow: 1,
                    tunnels_to: vec![*hasher.id_of_or_create("BB")],
                },
                ValveData {
                    id: *hasher.id_of_or_create("BB"),
                    flow: 2,
                    tunnels_to: vec![*hasher.id_of_or_create("CC")],
                },
            ],
            hasher,
        );

        assert_eq!(map, map_from_macro)
    }

    #[test]
    fn parsing_given_input() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();

        let mut hasher = map.hasher.clone();
        assert_eq!(
            map,
            map!(
                hasher,
                valve! (&mut hasher,"AA";0 => "DD","II", "BB")
                valve! (&mut hasher, "BB";13 => "CC", "AA")
                valve! (&mut hasher, "CC";2 => "DD", "BB")
                valve! (&mut hasher, "DD";20 => "CC", "AA", "EE")
                valve! (&mut hasher, "EE";3 => "FF", "DD")
                valve! (&mut hasher, "FF";0 => "EE", "GG")
                valve! (&mut hasher, "GG";0 => "FF", "HH")
                valve! (&mut hasher, "HH";22 => "GG")
                valve! (&mut hasher, "II";0 => "AA", "JJ")
                valve! (&mut hasher, "JJ";21 => "II")
            )
        );
    }
}

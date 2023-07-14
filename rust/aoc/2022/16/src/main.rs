use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Error, Result};
use lazy_regex::regex_captures;

use challenges_common::graph::{astar, Step};

fn main() {
    let map = parse(&["aoc", "2022", "16.txt"]).unwrap();
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

fn parse<'map>(path: &[&str]) -> Result<Map> {
    challenges_common::get_input_lines(path)
        .map(|line| line.parse())
        .collect::<Result<Vec<_>>>()
        .map(Map::from)
}

fn parse_valve_id(id: &str) -> Result<u16> {
    let mut chars = id.chars();
    let c1 = chars.next().ok_or(anyhow!("no first char in id"))?;
    let c2 = chars.next().ok_or(anyhow!("no first char in id"))?;

    Ok(((c1 as u16) << 8) + c2 as u16)
}

mod part1;

fn part1(map: &Map) -> u32 {
    use part1::Part1;
    map.get_max_released(30).unwrap()
}

mod part2;

fn part2(map: &Map) -> u32 {
    use part2::Part2;
    map.get_max_released(26).unwrap()
}

type ValveId = u16;

#[derive(PartialEq, Eq, Debug)]
struct ValveData {
    id: ValveId,
    flow: u32,
    tunnels_to: Vec<ValveId>,
}

#[derive(PartialEq, Debug)]
struct Map {
    valves: HashMap<ValveId, ValveData>,
}

#[derive(PartialEq, Debug, Clone)]
struct Move<'map> {
    cost: u8,
    target: &'map ValveId,
}

impl Map {
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
                                additionnal_cost: 1,
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
}

impl FromStr for ValveData {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, id, flow_rate, tunnels) = regex_captures!(
            r#"Valve ([[:alpha:]]+) has flow rate=(\d+); tunnels? leads? to valves? ((?:[[:alpha:]]+(?:, )?)+)"#,
            s
        ).ok_or_else(|| anyhow!("{:?} does not match the pattern", s))?;

        let tunnel_to = tunnels
            .split(", ")
            .map(parse_valve_id)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            id: parse_valve_id(id)?,
            flow: flow_rate.parse()?,
            tunnels_to: tunnel_to,
        })
    }
}

impl From<Vec<ValveData>> for Map {
    fn from(valves: Vec<ValveData>) -> Self {
        valves.into_iter().fold(
            Map {
                valves: HashMap::new(),
            },
            |mut map, valve_data| {
                map.valves
                    .entry(valve_data.id.clone())
                    .or_insert(valve_data);
                map
            },
        )
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    fn s(str: &str) -> ValveId {
        parse_valve_id(str).unwrap()
    }

    macro_rules! map {
        ($($valve:expr)*) => {Map::from(vec![$($valve,)*])};
    }

    macro_rules! valve {
        ($id:expr;$flow_rate:expr => $($target:expr),*) => {
            ValveData {
                id: parse_valve_id($id).unwrap(),
                flow:$flow_rate,
                tunnels_to: vec![
                    $(parse_valve_id($target).unwrap(),)*
                ]
            }
        };
    }

    #[test]
    fn creating_a_map_with_macro() {
        let map_from_macro = map!(valve!("AA";1 => "BB") valve!("BB";2 => "CC"));

        let map = Map::from(vec![
            ValveData {
                id: s("AA"),
                flow: 1,
                tunnels_to: vec![s("BB")],
            },
            ValveData {
                id: s("BB"),
                flow: 2,
                tunnels_to: vec![s("CC")],
            },
        ]);

        assert_eq!(map, map_from_macro)
    }

    #[test]
    fn parsing_given_input() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        assert_eq!(
            map,
            map!(
                valve! ("AA";0 => "DD","II", "BB")
                valve! ("BB";13 => "CC", "AA")
                valve! ("CC";2 => "DD", "BB")
                valve! ("DD";20 => "CC", "AA", "EE")
                valve! ("EE";3 => "FF", "DD")
                valve! ("FF";0 => "EE", "GG")
                valve! ("GG";0 => "FF", "HH")
                valve! ("HH";22 => "GG")
                valve! ("II";0 => "AA", "JJ")
                valve! ("JJ";21 => "II") )
        );
    }
}

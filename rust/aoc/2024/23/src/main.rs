use anyhow::{anyhow, Error};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "23.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

type Computer = String;

struct Connections {
    connections: HashSet<Connection>,
    connections_by_computer: HashMap<Computer, Vec<Computer>>,
}

impl Connections {
    fn is_linked_to(&self, a: &Computer, b: &Computer) -> bool {
        self.connections.iter().any(|connection| {
            (&connection.0[0] == a && &connection.0[1] == b)
                || (connection.0[0] == *b && connection.0[1] == *a)
        })
    }
}

impl From<Vec<Connection>> for Connections {
    fn from(value: Vec<Connection>) -> Self {
        let connections_by_computer = value.iter().fold(
            HashMap::<Computer, Vec<Computer>>::new(),
            |mut acc, connection| {
                acc.entry(connection.0[0].clone())
                    .or_default()
                    .push(connection.0[1].clone());
                acc.entry(connection.0[1].clone())
                    .or_default()
                    .push(connection.0[0].clone());
                acc
            },
        );

        let connections: HashSet<Connection> = value.into_iter().collect();

        Self {
            connections,
            connections_by_computer,
        }
    }
}

#[derive(Debug)]
struct Connection([Computer; 2]);

impl Hash for Connection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0[0] < self.0[1] {
            self.0[0].hash(state);
            self.0[1].hash(state);
        } else {
            self.0[1].hash(state);
            self.0[0].hash(state);
        }
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1]
            || self.0[0] == other.0[1] && self.0[1] == other.0[0]
    }
}

impl Eq for Connection {}

impl FromStr for Connection {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (computer_a, computer_b) = s
            .split_once("-")
            .ok_or_else(|| anyhow!("cannot parse {s}: missing -"))?;
        anyhow::Ok(Self([computer_a.to_string(), computer_b.to_string()]))
    }
}

use crate::{Computer, Connection, Connections};
use anyhow::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

type Res = String;
pub(crate) fn run(content: &str) -> Result<Res> {
    let connections: Vec<Connection> = content.lines().map(Connection::from_str).try_collect()?;
    let connections = Connections::from(connections);
    Ok(connections.lan().iter().join(","))
}

trait Part2Connection {
    fn lan(&self) -> Vec<&Computer>;
}

impl Part2Connection for Connections {
    fn lan(&self) -> Vec<&Computer> {
        let mut visited = HashSet::new();
        let mut lan = Vec::new();

        for (computer, connected) in self.connections_by_computer.iter() {
            if !visited.contains(computer) {
                let mut group: Vec<_> = std::iter::once(computer).collect();

                for connected in connected {
                    // we consider only the first connected component
                    if group.iter().all(|c| self.is_linked_to(c, connected)) {
                        visited.insert(connected);
                        group.push(connected);
                    }
                }

                if group.len() > lan.len() {
                    lan = group;
                }
            }
        }

        lan.sort();

        lan
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "23-test.txt"]);
        assert_eq!(run(&content).unwrap(), "co,de,ka,ta");
    }
}

use crate::{Connection, Connections};
use anyhow::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let connections: Vec<Connection> = content.lines().map(Connection::from_str).try_collect()?;
    let connections = Connections::from(connections);
    Ok(connections.extract_trio())
}

trait Part1Connection {
    fn extract_trio(&self) -> usize;
}

impl Part1Connection for Connections {
    fn extract_trio(&self) -> usize {
        let sets =
            self.connections_by_computer
                .iter()
                .filter(|(a, connected)| a.starts_with("t") && connected.len() >= 2)
                .flat_map(|(a, connected)| {
                    connected.iter().enumerate().flat_map(move |(i, b)| {
                        connected.iter().skip(i + 1).map(move |c| (a, b, c))
                    })
                })
                .filter(|(_a, b, c)| self.is_linked_to(b, c))
                .map(|(a, b, c)| {
                    let mut res = [a, b, c];
                    res.sort();
                    res
                })
                .collect::<HashSet<_>>();
        sets.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "23-test.txt"]);
        assert_eq!(run(&content).unwrap(), 7);
    }
}

use challenges_common::MyIterTools;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

mod part1;
mod part2;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "08.txt"]);
    println!("part1: {}", part1::run(&content));
    println!("part2: {}", part2::run(&content));
}

struct Map {
    moves: Vec<Move>,
    nodes: HashMap<String, Node>,
}

impl Map {
    fn get(&self, node_name: &str) -> Option<&Node> {
        self.nodes.get(node_name)
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut split = s.lines().split(|line| line.is_empty());

        let moves = *split
            .next()
            .ok_or_else(|| anyhow::anyhow!("No moves found"))?
            .first()
            .ok_or_else(|| anyhow::anyhow!("empty moves"))?;

        let moves = moves
            .chars()
            .map(|c| match c {
                'L' => Ok(Move::Left),
                'R' => Ok(Move::Right),
                _ => Err(anyhow::anyhow!("Invalid move: {}", c)),
            })
            .try_collect()?;

        let nodes = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("No nodes found"))?
            .iter()
            .map(|line| line.parse::<Node>())
            .map(|node| node.map(|node| (node.id.clone(), node)))
            .try_collect()?;

        Ok(Map { moves, nodes })
    }
}

enum Move {
    Left,
    Right,
}

struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn get(&self, m: &Move) -> &str {
        match m {
            Move::Left => &self.left,
            Move::Right => &self.right,
        }
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (_, node_name, left_name, right_name) =
            lazy_regex::regex_captures!("(.+) = \\((.+), (.+)\\)", s)
                .ok_or_else(|| anyhow::anyhow!("Invalid node definition: {}", s))?;

        Ok(Node {
            id: node_name.to_owned(),
            left: left_name.to_owned(),
            right: right_name.to_owned(),
        })
    }
}

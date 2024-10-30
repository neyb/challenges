use super::*;
use anyhow::Result;

type StepCount = u32;

pub fn run(content: &str) -> StepCount {
    let map = content.parse().expect("Invalid map");
    let mut runner = Runner::new(&map);
    runner.count_step().expect("Failed to run")
}

struct Runner<'m> {
    map: &'m Map,
    current_move_index: usize,
    current_nodes: Vec<&'m Node>,
}

impl<'m> Runner<'m> {
    fn new(map: &'m Map) -> Self {
        let current_nodes = map
            .nodes
            .values()
            .filter(|node| node.id.ends_with('A'))
            .collect();
        Self {
            map,
            current_move_index: 0,
            current_nodes,
        }
    }

    fn count_step(&mut self) -> Result<StepCount> {
        let mut steps = 0;
        while !self.has_reached_destination() {
            let next_move = self.next_move()?;
            for node in self.current_nodes.iter_mut() {
                let next_node_name = node.get(next_move);
                *node = self
                    .map
                    .get(next_node_name)
                    .ok_or_else(|| anyhow::anyhow!("node not found : {}", next_node_name))?;
            }
            steps += 1;
        }
        Ok(steps)
    }

    fn has_reached_destination(&mut self) -> bool {
        self.current_nodes.iter().all(|node| node.id.ends_with('Z'))
    }

    fn next_move(&mut self) -> Result<&'m Move> {
        let i = self.current_move_index;
        self.current_move_index = (i + 1) % self.map.moves.len();
        self.map
            .moves
            .get(i)
            .ok_or_else(|| anyhow::anyhow!("move not found : {}", i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_1() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "08-test-3.txt"]);
        assert_eq!(run(&content), 6);
    }
}

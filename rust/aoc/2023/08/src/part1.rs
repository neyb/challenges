use super::*;
use anyhow::Result;

type StepCount = u16;

pub fn run(content: &str) -> StepCount {
    let map = content.parse().expect("Invalid map");
    let mut runner = Runner::new(&map);
    runner.count_step().expect("Failed to run")
}

struct Runner<'m> {
    map: &'m Map,
    current_move_index: usize,
    current_node: &'m Node,
}

impl<'m> Runner<'m> {
    fn new(map: &'m Map) -> Self {
        let current_node = map.nodes.get("AAA").expect("start node not found");
        Self {
            map,
            current_move_index: 0,
            current_node,
        }
    }

    fn count_step(&mut self) -> Result<StepCount> {
        let mut steps = 0;
        while self.current_node.id != "ZZZ" {
            let next_move =
                self.map.moves.get(self.current_move_index).ok_or_else(|| {
                    anyhow::anyhow!("move not found : {}", self.current_move_index)
                })?;
            let next_node_name = self.current_node.get(next_move);
            self.current_node = self
                .map
                .get(next_node_name)
                .ok_or_else(|| anyhow::anyhow!("node not found : {}", next_node_name))?;
            self.current_move_index = (self.current_move_index + 1) % self.map.moves.len();
            steps += 1;
        }
        Ok(steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_1() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "08-test-1.txt"]);
        assert_eq!(run(&content), 2);
    }

    #[test]
    fn given_test_2() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "08-test-2.txt"]);
        assert_eq!(run(&content), 6);
    }
}

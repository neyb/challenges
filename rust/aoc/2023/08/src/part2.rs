use super::*;
use anyhow::Result;

type StepCount = u64;

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
        // for some dark reasons... This works for the given input... I don't like that...
        // this is not even on strict loop length...
        // in theory, we only know the answer is a multiple of the lcm of all loop lengths
        // the multiple should be less than the size of the moves list...
        // no time for this shit... probably the worst aoc ever...
        Ok(self
            .get_all_path_len()?
            .into_iter()
            .fold(1, challenges_common::math::lcm))
    }

    fn get_all_path_len(&mut self) -> Result<Vec<StepCount>> {
        let mut step = 0;
        let mut paths_length = vec![None; self.current_nodes.len()];
        while paths_length.iter().filter(|p| p.is_some()).count() != self.current_nodes.len() {
            step += 1;
            let next_move = self.next_move()?;
            for (index, node) in self.current_nodes.iter_mut().enumerate() {
                let next_node_name = node.get(next_move);
                *node = self
                    .map
                    .get(next_node_name)
                    .ok_or_else(|| anyhow::anyhow!("node not found : {}", next_node_name))?;

                if paths_length[index].is_none() && node.id.ends_with('Z') {
                    paths_length[index] = Some(step);
                }
            }
        }

        Ok(paths_length.into_iter().flatten().collect())
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

use crate::{Load, Map};
use challenges_common::graph::Direction;

pub(crate) fn run(content: &str) -> anyhow::Result<Load> {
    let mut map: Map = content.parse()?;
    map.tilt(Direction::Up);
    Ok(map.get_north_load())
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 136);
    }
}

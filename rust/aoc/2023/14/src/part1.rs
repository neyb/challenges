use crate::{Load, Platform};
use challenges_common::graph::Direction;

pub(crate) fn run(content: &str) -> anyhow::Result<Load> {
    let mut platform: Platform = content.parse()?;
    platform.tilt(Direction::Up);
    Ok(platform.get_north_load())
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 136);
    }
}

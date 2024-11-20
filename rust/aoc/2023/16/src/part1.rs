use crate::{BeamHead, Contraption};
use challenges_common::graph::{Coord, Direction};

pub(crate) fn run(content: &str) -> anyhow::Result<usize> {
    let contraption: Contraption = content.parse()?;
    Ok(contraption.count_energized(BeamHead::new(Coord { x: 0, y: 0 }, Direction::Right)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "16-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 46);
    }
}

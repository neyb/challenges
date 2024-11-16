use crate::{Load, Map};

pub(crate) fn run(content: &str) -> anyhow::Result<Load> {
    let mut map: Map = content.parse()?;
    map.tilt_top()?;
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

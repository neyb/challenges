use anyhow::*;
use itertools::Itertools;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let new_content: String = content
        .split("don't")
        .enumerate()
        .flat_map(|(i, part)| {
            if i == 0 {
                vec![part]
            } else {
                part.split("do").skip(1).collect_vec()
            }
        })
        .collect();

    crate::part1::run(&new_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "03-test.txt"]);
        assert_eq!(run(&content).unwrap(), 0);
    }
}

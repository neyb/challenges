use anyhow::*;

type Res = usize;
pub(crate) fn run(_content: &str) -> Result<Res> {
    todo!("part 2 not implemented yet");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "${YEAR}", "${DAY}-test.txt"]);
        assert_eq!(run(&content).unwrap(), 0);
    }
}

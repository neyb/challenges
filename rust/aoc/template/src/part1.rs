use anyhow::*;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    todo!()
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

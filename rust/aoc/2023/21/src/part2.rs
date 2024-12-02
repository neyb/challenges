use anyhow::*;

type Res = usize;
pub(crate) fn run(_content: &String) -> Result<Res> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "21-test.txt"]);
        assert_eq!(run(&content).unwrap(), 0);
    }
}

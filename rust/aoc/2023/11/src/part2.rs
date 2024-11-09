use anyhow::Result;

type Len = usize;

pub fn run(content: &str) -> Result<Len> {
    crate::run(content, 1_000_000)
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_test_with_expansion_rate_of_10_is_1030() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "11-test.txt"]);
        assert_eq!(crate::run(&content, 10).unwrap(), 1030);
    }
}

use anyhow::Result;

type Len = usize;
pub fn run(content: &str) -> Result<Len> {
    super::run(content, 2)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn there_are_36_pairs_in_given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "11-test.txt"]);
        let universe: Universe = content.parse().unwrap();
        assert_eq!(universe.galaxies_pairs().len(), 36);
    }

    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "11-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 374);
    }
}

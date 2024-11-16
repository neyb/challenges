use crate::Load;

pub(crate) fn run(content: &String) -> anyhow::Result<Load> {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 136);
    }
}

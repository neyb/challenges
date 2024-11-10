use crate::{Len, Line};

pub(crate) fn run(content: &str) -> anyhow::Result<Len> {
    content
        .lines()
        .map(|line| line.parse::<Line>().map(|mut line| line.duplicate(5)))
        .try_fold(0, |acc, line| -> anyhow::Result<Len> {
            Ok(acc + line?.nb_arrangement())
        })
}

#[cfg(test)]
mod tests {

    #[test]
    fn given_test_first_line_has_1_arrangement() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "12-test.txt"]);
        let mut line = content
            .lines()
            .next()
            .unwrap()
            .parse::<super::Line>()
            .unwrap();
        line.duplicate(5);
        assert_eq!(line.nb_arrangement(), 1);
    }

    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "12-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 525152);
    }
}

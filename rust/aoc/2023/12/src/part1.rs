use crate::{Len, Line};

pub(crate) fn run(content: &str) -> anyhow::Result<Len> {
    content
        .lines()
        .map(|line| line.parse::<Line>())
        .try_fold(0, |acc, line| -> anyhow::Result<Len> {
            Ok(acc + line?.nb_arrangement())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_first_line_has_1_arrangement() {
        assert_eq!(given_test_nb_arrangements_on_nth_line(1), 1);
    }

    #[test]
    fn two_unknown_should_have_2_arrangements() {
        let line: Line = "?? 1".parse().unwrap();
        assert_eq!(line.nb_arrangement(), 2);
    }

    #[test]
    fn simple_line_test1() {
        let line: Line = "?#? 2".parse().unwrap();
        assert_eq!(line.nb_arrangement(), 2);
    }

    #[test]
    fn second_line_has_4_arrangements() {
        assert_eq!(given_test_nb_arrangements_on_nth_line(2), 4);
    }

    fn given_test_nb_arrangements_on_nth_line(n: usize) -> Len {
        let content = challenges_common::get_input_content(&["aoc", "2023", "12-test.txt"]);
        let line: Line = content.lines().nth(n - 1).unwrap().parse().unwrap();
        line.nb_arrangement()
    }

    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "12-test.txt"]);
        assert_eq!(run(&content).unwrap(), 21);
    }
}

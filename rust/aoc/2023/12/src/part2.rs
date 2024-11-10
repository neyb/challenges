use crate::Line;
use std::io::{Stdout, Write};

pub(crate) fn run(content: &str) -> anyhow::Result<crate::Len> {
    let mut sum = 0;
    for (n, line) in content.lines().enumerate() {
        print!("line {}... ", n + 1);
        Stdout::flush(&mut std::io::stdout())?;

        let mut line: Line = line.parse()?;
        print!("parsed... ");
        Stdout::flush(&mut std::io::stdout())?;
        line.duplicate(5);
        print!("duplicated... ");
        Stdout::flush(&mut std::io::stdout())?;
        let nb_arrangements = line.nb_arrangement();
        println!("{nb_arrangements}");
        sum += nb_arrangements;
    }
    Ok(sum)
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

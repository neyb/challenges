use crate::{Program, RegisterUnit};
use anyhow::*;
use rayon::prelude::*;

type Res = RegisterUnit;
pub(crate) fn run(content: &str) -> Result<Res> {
    let (computer, program) = crate::parse(content)?;

    (0..10)
        .into_par_iter()
        .find_first(|&i| {
            let mut computer = computer.clone();
            computer.registers[0].0 = i;
            let output = computer.execute(&program);
            let string = output.to_string();
            string.parse::<Program>().unwrap() == program
        })
        .ok_or_else(|| anyhow!("No solution found"))

    //wanna cry
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "17-test-part2.txt"]);
        assert_eq!(run(&content).unwrap(), 117440);
    }

    #[test]
    fn test_with_solution() {
        let (mut computer, program) = build(117440, 0, 0, "0,3,5,4,3,0");
        let output = computer.execute(&program);
        assert_eq!(output.to_string(), "0,3,5,4,3,0");
    }
}

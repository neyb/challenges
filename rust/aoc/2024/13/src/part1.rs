use crate::{Machines, Unit};

type Res = Unit;
pub(crate) fn run(content: &str) -> anyhow::Result<Res> {
    let machines: Machines = content.parse()?;
    anyhow::Ok(machines.min_cost())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_first_machine() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "13-test.txt"]);
        let machine = &content.parse::<Machines>().unwrap().0[0];
        assert_eq!(machine.min_cost().unwrap(), 280);
    }

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "13-test.txt"]);
        assert_eq!(run(&content).unwrap(), 480);
    }
}

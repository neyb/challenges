use crate::{Machines, Unit};
use anyhow::*;

type Res = Unit;
pub(crate) fn run(content: &str) -> Result<Res> {
    let mut machines: Machines = content.parse()?;

    for machine in &mut machines.0 {
        machine.price.x += 10000000000000;
        machine.price.y += 10000000000000;
    }

    Ok(machines.min_cost())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "13-test.txt"]);
        assert_eq!(run(&content).unwrap(), 0);
    }
}

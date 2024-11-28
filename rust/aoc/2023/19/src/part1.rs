use crate::parse_input;
use anyhow::*;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<usize> {
    let (workflows, machine_parts) = parse_input(content)?;

    let in_rule = workflows
        .get("in")
        .ok_or_else(|| anyhow!("No in workflow"))?;

    let mut res = 0;
    for machine_part in &machine_parts.0 {
        if in_rule.test(machine_part, &workflows)? {
            res += machine_part.rate() as Res;
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "19-test.txt"]);
        assert_eq!(run(&content).unwrap(), 19114);
    }
}

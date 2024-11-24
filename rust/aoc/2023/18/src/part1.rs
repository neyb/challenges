use crate::DigPlan;
use anyhow::*;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    Ok(content.parse::<DigPlan>()?.path()?.area())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "18-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 62);
    }

    #[test]
    fn given_test_path() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "18-test.txt"]);
        let dig_plan: DigPlan = content.parse().unwrap();
        let path = dig_plan.path().unwrap();

        assert_eq!(path.len, 38);
    }

    #[test]
    fn custom_test_2() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "18-test2.txt"]);
        let dig_plan: DigPlan = content.parse().unwrap();
        let path = dig_plan.path().unwrap();

        assert_eq!(path.len, 52);

        assert_eq!(path.area(), 82);
    }
}

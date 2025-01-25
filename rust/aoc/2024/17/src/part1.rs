use anyhow::*;

type Res = String;
pub(crate) fn run(content: &str) -> Result<Res> {
    let (mut computer, program) = crate::parse(content)?;
    let output = computer.execute(&program);
    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "17-test.txt"]);
        assert_eq!(run(&content).unwrap(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn first_test() {
        let (mut computer, program) = build(0, 0, 9, "2,6");
        let output = computer.execute(&program);

        assert_eq!(output.to_string(), "");
        assert_eq!(computer.registers[1].0, 1);
    }

    #[test]
    fn second_test() {
        let (mut computer, program) = build(10, 0, 0, "5,0,5,1,5,4");
        let output = computer.execute(&program);

        assert_eq!(output.to_string(), "0,1,2");
    }

    #[test]
    fn third_test() {
        let (mut computer, program) = build(2024, 0, 0, "0,1,5,4,3,0");
        // 0,1 -> A set to 1012
        // 5,4 -> out A%8 = 4
        // 3,0 -> jump to 0

        // 0,1 -> A set to 506
        // 5,4 -> out A%8 = 2
        // 3,0 -> jump to 0

        // -> A set to 253 -> out 5
        // -> A set to 126 -> out 6
        // -> A set to 63 -> out 7
        // -> A set to 31 -> out 7
        // -> A set to 15 -> out 7
        // -> A set to 7 -> out 7
        // -> A set to 3 -> out 7
        // -> A set to 1 -> out 3
        // -> A set to 0 -> out 0
        // -> exit

        let output = computer.execute(&program);

        assert_eq!(output.to_string(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(computer.registers[0].0, 0);
    }

    #[test]
    fn fourth_test() {
        let (mut computer, program) = build(0, 29, 0, "1,7");
        //     29 = 0b11101
        //      7 = 0b00111
        // 29 ^ 7 = 0b11010 = 26

        let output = computer.execute(&program);

        assert_eq!(output.to_string(), "");
        assert_eq!(computer.registers[1].0, 26);
    }

    #[test]
    fn fifth_test() {
        let (mut computer, program) = build(0, 2024, 43690, "4,0");
        let output = computer.execute(&program);

        assert_eq!(output.to_string(), "");
        assert_eq!(computer.registers[1].0, 44354);
    }
}

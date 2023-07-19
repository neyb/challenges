use std::str::FromStr;

pub use cpu::*;
use itertools::Itertools;
pub use screen::*;

fn main() {
    let instructions = challenges_common::get_input_lines(&["aoc", "2022", "10.txt"])
        .map(|line| Instruction::from_str(&line).unwrap())
        .collect_vec();

    let result = part1(&instructions);
    println!("part1: {}", result);
    println!(
        "part2:
{}",
        part2(&instructions)
    )
}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    let mut computer = Computer::new(instructions.clone());
    (20u32..=220)
        .step_by(40)
        .map(|cycle| {
            computer.run_until(cycle);
            cycle as i32 * computer.cpu.x
        })
        .sum()
}

fn part2(instructions: &Vec<Instruction>) -> Screen {
    let mut computer = Computer::new(instructions.clone());
    computer.run_until_program_is_over();
    computer.screen
}

struct Computer {
    clock: u32,
    cpu: Cpu,
    screen: Screen,
}

impl Computer {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            clock: 1,
            cpu: Cpu::new(instructions),
            screen: Screen::new(),
        }
    }

    fn tick(&mut self) {
        self.screen.tick(self.clock, self.cpu.x);
        self.cpu.tick();
        self.clock += 1;
    }

    fn run_until(&mut self, cycle: u32) {
        while self.clock < cycle {
            self.tick()
        }
    }

    fn run_until_program_is_over(&mut self) {
        while self.cpu.current_instruction().is_some() {
            self.tick()
        }
    }
}

mod cpu {
    use std::str::FromStr;

    use anyhow::{anyhow, Error, Result};
    use itertools::Itertools;

    type Value = i32;
    pub struct Cpu {
        instructions: Vec<Instruction>,
        pub x: Value,
        current_instruction_index: usize,
        current_instruction_cycle_count: u8,
    }

    impl Cpu {
        pub fn new(instructions: Vec<Instruction>) -> Self {
            Self {
                instructions,
                x: 1,
                current_instruction_index: 0,
                current_instruction_cycle_count: 0,
            }
        }

        pub fn current_instruction(&self) -> Option<&Instruction> {
            self.instructions.get(self.current_instruction_index)
        }

        pub fn tick(&mut self) {
            if let Some(current_instruction) = self.current_instruction() {
                if current_instruction.is_over(self.current_instruction_cycle_count + 1) {
                    self.x = current_instruction.run(self.x);
                    self.current_instruction_cycle_count = 0;
                    self.current_instruction_index += 1
                } else {
                    self.current_instruction_cycle_count += 1;
                }
            }
        }
    }

    #[derive(Clone)]
    pub enum Instruction {
        Noop,
        AddX(Value),
    }

    impl Instruction {
        fn is_over(&self, tick: u8) -> bool {
            match self {
                Instruction::Noop => tick >= 1,
                Instruction::AddX(_) => tick >= 2,
            }
        }

        fn run(&self, x: Value) -> Value {
            match self {
                Instruction::Noop => x,
                Instruction::AddX(to_add) => x + to_add,
            }
        }
    }

    impl FromStr for Instruction {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            use Instruction::*;

            Ok(match s.split(' ').collect_vec()[..] {
                ["noop"] => Noop,
                ["addx", value] => AddX(value.parse()?),
                _ => Err(anyhow!("cannot parse instrution from {}", s))?,
            })
        }
    }
}

mod screen {
    use std::fmt::{Display, Formatter};

    use itertools::Itertools;

    pub struct Screen {
        pub pixels: Vec<bool>,
    }

    impl Screen {
        pub fn new() -> Self {
            Self {
                pixels: vec![false; 240],
            }
        }

        pub fn tick(&mut self, clock: u32, x: i32) {
            if ((clock as i32 - 1) % 40).abs_diff(x) <= 1 {
                if let Some(pixel) = self.pixels.get_mut((clock as usize - 1) % 240) {
                    *pixel = true;
                }
            }
        }
    }

    impl Display for Screen {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for y in 0..6 {
                writeln!(
                    f,
                    "{}",
                    (0..40)
                        .map(|c| self.pixels.get(c + 40 * y).unwrap())
                        .map(|&pixel| if pixel { '#' } else { '.' })
                        .join("")
                )?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    mod computer {
        use crate::*;

        #[test]
        fn tick_increment_cycle_count() {
            let circuit = vec![Instruction::Noop];

            let mut computer = Computer::new(circuit);

            assert_eq!(computer.clock, 1);
            computer.tick();
            assert_eq!(computer.clock, 2)
        }
    }

    mod cpu {
        use crate::*;

        #[test]
        fn noop_is_consumed_after_1_tick() {
            let circuit = vec![Instruction::Noop];

            let mut cpu = Cpu::new(circuit);

            assert!(cpu.current_instruction().is_some());
            cpu.tick();
            assert!(cpu.current_instruction().is_none());
        }

        #[test]
        fn addx_is_consumed_after_2_tick() {
            let circuit = vec![Instruction::AddX(3)];

            let mut cpu = Cpu::new(circuit);

            assert!(cpu.current_instruction().is_some());
            cpu.tick();
            assert!(cpu.current_instruction().is_some());
            cpu.tick();
            assert!(cpu.current_instruction().is_none());
        }

        #[test]
        fn addx_increment_x_after_2_ticks() {
            let mut cpu = Cpu::new(vec![Instruction::AddX(3)]);

            assert_eq!(cpu.x, 1);
            cpu.tick();
            assert_eq!(cpu.x, 1);
            cpu.tick();
            assert_eq!(cpu.x, 4);
        }

        #[test]
        fn given_simple_test() {
            use Instruction::*;
            let instructions = vec![Noop, AddX(3), AddX(-5)];
            let mut cpu = Cpu::new(instructions);

            assert_eq!(cpu.x, 1);
            cpu.tick();
            assert_eq!(cpu.x, 1);
            cpu.tick();
            assert_eq!(cpu.x, 1);
            cpu.tick();
            assert_eq!(cpu.x, 4);
            cpu.tick();
            assert_eq!(cpu.x, 4);
            cpu.tick();
            assert_eq!(cpu.x, -1);
        }
    }

    mod screen {}

    #[test]
    fn part1_given_test() {
        let instructions = challenges_common::get_input_lines(&["aoc", "2022", "10-test.txt"])
            .map(|line| Instruction::from_str(&line).unwrap())
            .collect_vec();

        let mut computer = Computer::new(instructions);
        computer.run_until(20);
        assert_eq!(computer.cpu.x, 21);
        computer.run_until(60);
        assert_eq!(computer.cpu.x, 19);
        computer.run_until(100);
        assert_eq!(computer.cpu.x, 18);
        computer.run_until(140);
        assert_eq!(computer.cpu.x, 21);
        computer.run_until(180);
        assert_eq!(computer.cpu.x, 16);
        computer.run_until(220);
        assert_eq!(computer.cpu.x, 18);
    }

    #[test]
    fn part2_given_test() {
        let instructions = challenges_common::get_input_lines(&["aoc", "2022", "10-test.txt"])
            .map(|line| Instruction::from_str(&line).unwrap())
            .collect_vec();

        let mut computer = Computer::new(instructions);
        computer.run_until_program_is_over();
        assert_eq!(
            format!("{}", computer.screen),
            challenges_common::get_input_content(&["aoc", "2022", "10-test-part2-expected.txt"])
        )
    }
    #[test]
    fn part2_given_test_step_by_step() {
        let instructions = challenges_common::get_input_lines(&["aoc", "2022", "10-test.txt"])
            .map(|line| Instruction::from_str(&line).unwrap())
            .collect_vec();

        let mut computer = Computer::new(instructions);

        let expected = "##..##..##..##..##..#"
            .chars()
            .map(|c| c == '#')
            .collect_vec();

        for index in 0..expected.len() {
            computer.tick();
            assert_eq!(computer.screen.pixels[..index], expected[..index]);
        }
    }
}

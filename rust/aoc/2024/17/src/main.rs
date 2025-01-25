use anyhow::*;
use challenges_common::MyIterTools;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "17.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run());
}

mod part1;
mod part2;

fn parse(content: &str) -> Result<(Computer, Program)> {
    let (registers, program) = content
        .lines()
        .split(|line| line.is_empty())
        .map(|lines| lines.join("\n"))
        .collect_tuple()
        .ok_or_else(|| anyhow!("Invalid input"))?;

    let registers: Vec<Register> = registers
        .lines()
        .map(|line| -> Result<Register> {
            line.split(": ")
                .nth(1)
                .ok_or_else(|| anyhow!("Invalid register: {line}"))?
                .parse()
        })
        .try_collect()?;
    let registers = registers.try_into().unwrap(); // replace with ?...
    let computer = Computer { registers };

    let program = program
        .split(": ")
        .nth(1)
        .ok_or_else(|| anyhow!("Invalid program:{program}"))?
        .parse()?;
    anyhow::Ok((computer, program))
}

#[cfg(test)]
fn build(
    register_a: RegisterUnit,
    register_b: RegisterUnit,
    register_c: RegisterUnit,
    program: &str,
) -> (Computer, Program) {
    (
        Computer {
            registers: [
                Register(register_a),
                Register(register_b),
                Register(register_c),
            ],
        },
        program.parse().unwrap(),
    )
}

type RegisterUnit = u32;

#[derive(Clone, Eq, PartialEq)]
struct OpCode(u8);

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
struct Computer {
    registers: [Register; 3],
}

struct ProgramOutput(Vec<OpCode>);

impl Display for ProgramOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().join(","))
    }
}

impl Computer {
    fn execute(&mut self, program: &Program) -> ProgramOutput {
        let mut output = ProgramOutput(Vec::new());
        let mut pointer = 0;

        while pointer < program.op_codes.len() {
            let instruction = Instruction::from(&program.op_codes[pointer]);
            let operand = &program.op_codes[pointer + 1];
            match instruction.execute(operand, self) {
                InstructionResult::Jump(v) => {
                    pointer = v.0 as usize;
                }
                InstructionResult::Output(out) => {
                    output.0.push(out);
                    pointer += 2;
                }
                InstructionResult::Continue => {
                    pointer += 2;
                }
            }
        }
        output
    }

    fn read(&self, adr: usize) -> RegisterUnit {
        self.registers[adr].0
    }

    fn write(&mut self, adr: usize, value: RegisterUnit) {
        self.registers[adr] = Register(value);
    }
}

#[derive(PartialEq)]
struct Program {
    op_codes: Vec<OpCode>,
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let op_codes = s
            .split(',')
            .map(|s| -> Result<OpCode> { anyhow::Ok(OpCode(s.parse()?)) })
            .try_collect()?;

        anyhow::Ok(Self { op_codes })
    }
}

#[derive(Debug, Clone)]
struct Register(RegisterUnit);

impl FromStr for Register {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        anyhow::Ok(Self(s.parse()?))
    }
}

enum Combo {
    Literal(Literal),
    Read(usize),
}

impl Combo {
    fn resolve(&self, computer: &Computer) -> RegisterUnit {
        match self {
            Self::Literal(value) => value.0 .0 as RegisterUnit,
            Self::Read(adr) => computer.read(*adr),
        }
    }
}

struct Literal(OpCode);

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    pub(crate) fn execute(&self, operand: &OpCode, computer: &mut Computer) -> InstructionResult {
        use InstructionResult::*;
        match self {
            Instruction::Adv => {
                let num = computer.read(0);
                let denom =
                    (2 as RegisterUnit).pow(Combo::try_from(operand).unwrap().resolve(computer));
                let dv = num / denom;
                computer.write(0, dv);
                Continue
            }
            Instruction::Bxl => {
                let xor = computer.read(1) ^ operand.0 as u32;
                computer.write(1, xor);
                Continue
            }
            Instruction::Bst => {
                let combo_value = Combo::try_from(operand).unwrap().resolve(computer);
                computer.write(1, combo_value.rem_euclid(8));
                Continue
            }
            Instruction::Jnz => {
                if computer.read(0) != 0 {
                    Jump(operand.clone())
                } else {
                    Continue
                }
            }
            Instruction::Bxc => {
                let b = computer.read(1);
                let c = computer.read(2);
                computer.write(1, b ^ c);
                Continue
            }
            Instruction::Out => {
                let out = Combo::try_from(operand).unwrap().resolve(computer);
                let out = out.rem_euclid(8) as u8;

                Output(OpCode(out))
            }
            Instruction::Bdv => {
                let num = computer.read(0);
                let denom =
                    (2 as RegisterUnit).pow(Combo::try_from(operand).unwrap().resolve(computer));
                let dv = num / denom;
                computer.write(1, dv);
                Continue
            }
            Instruction::Cdv => {
                let num = computer.read(0);
                let denom =
                    (2 as RegisterUnit).pow(Combo::try_from(operand).unwrap().resolve(computer));
                let dv = num / denom;
                computer.write(2, dv);
                Continue
            }
        }
    }
}

enum InstructionResult {
    Continue,
    Jump(OpCode),
    Output(OpCode),
}

impl TryFrom<u8> for OpCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        if value < 8 {
            anyhow::Ok(Self(value))
        } else {
            Err(anyhow!("Invalid opcode"))
        }
    }
}

impl TryFrom<&OpCode> for Combo {
    type Error = Error;

    fn try_from(value: &OpCode) -> std::result::Result<Self, Self::Error> {
        match value.0 {
            0..=3 => anyhow::Ok(Self::Literal(Literal(value.clone()))),
            4..=6 => anyhow::Ok(Self::Read(value.0 as usize - 4)),
            _ => Err(anyhow!("Invalid opcode")),
        }
    }
}

impl From<&OpCode> for Instruction {
    fn from(value: &OpCode) -> Self {
        match value.0 {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid opcode {}", value.0),
        }
    }
}

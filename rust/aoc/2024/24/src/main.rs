use anyhow::{anyhow, bail, Error};
use challenges_common::MyIterTools;
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "24.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

type Name = String;
type Value = bool;

struct Device {
    values: RefCell<HashMap<String, Value>>,
    outputs: HashMap<String, Gate>,
}

impl Device {
    fn new(init_values: HashMap<String, Value>, gates: Vec<Gate>) -> Self {
        let outputs = gates
            .into_iter()
            .into_grouping_map_by(|gate| gate.output_name().clone())
            .reduce(|_gate, _, gate| panic!("Multiple gates for the same output"));

        Self {
            values: RefCell::new(init_values),
            outputs,
        }
    }

    fn get_value(&self, name: &Name) -> Option<Value> {
        let found_value = { self.values.borrow().get(name).cloned() };
        match found_value {
            Some(value) => Some(value),
            None => {
                let value = self.outputs.get(name)?.eval(self)?;
                self.values.borrow_mut().insert(name.clone(), value);
                Some(value)
            }
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Gate {
    in1: Name,
    in2: Name,
    out: Name,
    op: GateOp,
}

impl Gate {
    fn new(in1: Name, in2: Name, out: Name, op: GateOp) -> Self {
        if in1 < in2 {
            Self { in1, in2, out, op }
        } else {
            Self {
                in1: in2,
                in2: in1,
                out,
                op,
            }
        }
    }

    fn input_names(&self) -> (&Name, &Name) {
        (&self.in1, &self.in2)
    }

    fn output_name(&self) -> &Name {
        &self.out
    }

    fn eval(&self, device: &Device) -> Option<Value> {
        let a = device.get_value(&self.in1)?;
        let b = device.get_value(&self.in2)?;

        let res = match &self.op {
            GateOp::And => a && b,
            GateOp::Or => a || b,
            GateOp::Xor => a != b,
        };

        Some(res)
    }

    fn other_input(&self, in_name: &str) -> &Name {
        if self.in1 == in_name {
            &self.in2
        } else {
            &self.in1
        }
    }

    fn has_input_bit(&self) -> bool {
        self.in1.starts_with("x")
            || self.in1.starts_with("y")
            || self.in2.starts_with("x")
            || self.in2.starts_with("y")
    }

    fn has_output_bit(&self) -> bool {
        self.out.starts_with("z")
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
enum GateOp {
    And,
    Or,
    Xor,
}

impl FromStr for Device {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut lines_blocs = s.lines().split(|line| line.is_empty());
        let init_values = lines_blocs
            .next()
            .ok_or(anyhow!("No init values"))?
            .into_iter()
            .map(|line| {
                let (name, value) = line.split_once(": ").ok_or(anyhow!("Cannot parse line"))?;
                anyhow::Ok((name.to_string(), value == "1"))
            })
            .try_collect()?;

        let gates = lines_blocs
            .next()
            .ok_or(anyhow!("No gates"))?
            .into_iter()
            .map(Gate::from_str)
            .try_collect()?;

        anyhow::Ok(Self::new(init_values, gates))
    }
}

impl FromStr for Gate {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (input, output) = s.split_once(" -> ").ok_or(anyhow!("Cannot parse gate"))?;
        let (_, a, op, b) = lazy_regex::regex_captures!("(.{3}) (AND|XOR|OR) (.{3})", s)
            .ok_or_else(|| anyhow!("Cannot parse gate"))?;

        let a = a.to_string();
        let b = b.to_string();
        let output = output.to_string();

        let gate_op = match op {
            "AND" => GateOp::And,
            "OR" => GateOp::Or,
            "XOR" => GateOp::Xor,
            _ => bail!("Unknown gate: {op}"),
        };

        anyhow::Ok(Self::new(a, b, output, gate_op))
    }
}

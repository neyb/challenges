use crate::{Device, Gate, GateOp, Name};
use anyhow::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::rc::Rc;

type Res = String;
pub(crate) fn run(content: &str) -> Result<Res> {
    let device: Device = content.parse()?;
    let device: Part2Device = device.into();
    let output_errs = device.check_adders();

    Ok(output_errs.join(","))
}

struct Part2Device {
    gates_by_input: HashMap<Name, Vec<Rc<Gate>>>,
    gates: Vec<Rc<Gate>>,
}

impl From<Device> for Part2Device {
    fn from(value: Device) -> Self {
        let gates: Vec<_> = value.outputs.into_values().map(Rc::new).collect();

        let gates_by_intput = gates
            .iter()
            .flat_map(|gate| {
                vec![
                    (gate.in1.clone(), gate.clone()),
                    (gate.in2.clone(), gate.clone()),
                ]
            })
            .into_group_map();

        Self {
            gates_by_input: gates_by_intput,
            gates,
        }
    }
}

impl Part2Device {
    fn check_adders(&self) -> Vec<Name> {
        let mut inverted = Vec::new();

        for gate in &self.gates {
            match gate.op {
                GateOp::And => {
                    // an AND gate is followed by a or gate (except for the first)
                    if gate.in1 != "x00" && !self.is_followed_by(gate, |next| next.op == GateOp::Or)
                    {
                        inverted.push(gate.out.clone());
                    }
                }
                GateOp::Or => {
                    // an OR gate is a carry
                    // => should not output to a bit (except for the last)
                    if gate.has_output_bit() && gate.out != "z45" {
                        inverted.push(gate.out.clone());
                    }

                    // => should output to an OR (and XOR) gate
                    if self.is_followed_by(gate, |next| next.op == GateOp::Or) {
                        inverted.push(gate.out.clone());
                    }
                }
                GateOp::Xor => {
                    if gate.has_input_bit() {
                        // an intermediate XOR gate should output to another XOR (except for the first)
                        if !self.is_followed_by(gate, |next_gate| next_gate.op == GateOp::Xor)
                            && gate.in1 != "x00"
                        {
                            inverted.push(gate.out.clone());
                        }
                    } else {
                        // an "final" XOR should output to an output bit
                        if !gate.has_output_bit() {
                            inverted.push(gate.out.clone());
                        }
                    }
                }
            }
        }

        inverted.sort();

        inverted
    }

    fn is_followed_by(&self, gate: &Rc<Gate>, criteria: impl Fn(&Rc<Gate>) -> bool) -> bool {
        match self.gates_by_input.get(gate.output_name()) {
            None => false,
            Some(following_gates) => following_gates.iter().any(criteria),
        }
    }
}

struct CheckAdderResult {
    is_valid: bool,
    output_err: Vec<Name>,
    carry_name: Name,
}

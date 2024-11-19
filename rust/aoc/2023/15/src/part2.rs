use crate::hash;
use anyhow::{anyhow, bail, Error, Result};
use array_init::array_init;
use itertools::Itertools;
use std::str::FromStr;

type FocusPower = u32;
pub(crate) fn run(content: &str) -> Result<FocusPower> {
    let seq: Seq = content.parse()?;
    let mut boxes = Boxes::new();
    boxes.apply_seq(&seq);
    Ok(boxes.focus_power())
}

struct Boxes {
    boxes: [Box; 256],
}

impl Boxes {
    fn new() -> Self {
        Self {
            boxes: array_init(|_| Box::new()),
        }
    }

    fn apply_seq(&mut self, seq: &Seq) {
        for operation in &seq.operations {
            match operation {
                Operation::Unset { label } => {
                    self.boxes[hash(label) as usize]
                        .lenses
                        .retain(|lens| &lens.label != label);
                }
                Operation::Set(lens) => {
                    let boxx = &mut self.boxes[hash(&lens.label) as usize];
                    match boxx.lenses.iter_mut().find(|l| l.label == lens.label) {
                        None => boxx.lenses.push(lens.clone()),
                        Some(installed_lens) => installed_lens.length = lens.length,
                    };
                }
            }
        }
    }

    fn focus_power(&self) -> FocusPower {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(box_index, boxx)| {
                boxx.lenses
                    .iter()
                    .enumerate()
                    .map(move |(lens_index, lens)| {
                        (1 + box_index as FocusPower)
                            * (1 + lens_index as FocusPower)
                            * lens.length as FocusPower
                    })
            })
            .sum()
    }
}

struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    fn new() -> Self {
        Self { lenses: Vec::new() }
    }
}

struct Seq {
    operations: Vec<Operation>,
}

enum Operation {
    Unset { label: String },
    Set(Lens),
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    length: u8,
}

impl FromStr for Seq {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            operations: s.split(",").map(|part| part.parse()).try_collect()?,
        })
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, label, op, n) = lazy_regex::regex_captures!(r"^(.*)(-|=)(\d+)?$", s)
            .ok_or_else(|| anyhow!("Cannot parse operation from {}", s))?;

        match op {
            "-" => Ok(Self::Unset {
                label: label.to_owned(),
            }),
            "=" => Ok(Self::Set(Lens {
                label: label.to_owned(),
                length: n.parse()?,
            })),
            _ => bail!("Cannot parse operation from {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test() {
        assert_eq!(
            run("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7").unwrap(),
            145
        )
    }
}

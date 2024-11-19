use crate::hash;
use itertools::Itertools;
use std::convert::Infallible;
use std::str::FromStr;

type Hash = u32;
pub(crate) fn run(content: &str) -> anyhow::Result<Hash> {
    let seq: Seq = content.parse()?;
    Ok(seq.hash())
}

struct Seq {
    steps: Vec<Step>,
}

impl Seq {
    fn hash(&self) -> Hash {
        self.steps.iter().map(|step| step.hash()).sum()
    }
}

struct Step {
    content: String,
}

impl Step {
    fn hash(&self) -> Hash {
        hash(&self.content) as Hash
    }
}

impl FromStr for Step {
    type Err = Infallible;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        Ok(Self {
            content: s.to_string(),
        })
    }
}

impl FromStr for Seq {
    type Err = Infallible;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        Ok(Self {
            steps: s.split(",").map(|part| part.parse()).try_collect()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test() {
        let seq: Seq = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            .parse()
            .unwrap();
        assert_eq!(seq.hash(), 1320)
    }

    #[test]
    fn part1() {
        let seq: Seq = challenges_common::get_input_content(&["aoc", "2023", "15.txt"])
            .parse()
            .unwrap();
        assert_eq!(seq.hash(), 512950)
    }
}

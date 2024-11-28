use anyhow::{anyhow, bail, Error};
use challenges_common::MyIterTools;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "19.txt"]);
    println!("{:?}", part1::run(&content));
    println!("{:?}", part2::run(&content));
}

mod part1;
mod part2;

fn parse_input(content: &str) -> anyhow::Result<(Workflows, MachineParts)> {
    anyhow::Ok(
        match &content.lines().split(|line| line.is_empty()).collect_vec()[..] {
            [workflows, machine_parts] => {
                let workflows: Vec<Workflow> = workflows
                    .iter()
                    .map(|line| anyhow::Ok(line.parse()?))
                    .try_collect()?;
                (
                    Workflows::new(workflows),
                    MachineParts(
                        machine_parts
                            .iter()
                            .map(|line| anyhow::Ok(line.parse()?))
                            .try_collect()?,
                    ),
                )
            }
            _ => bail!("Invalid input"),
        },
    )
}

type Value = u16;
struct Workflows(HashMap<String, Workflow>);

impl Workflows {
    fn new(workflows: impl IntoIterator<Item = Workflow>) -> Self {
        Self(
            workflows
                .into_iter()
                .map(|wf| (wf.name.clone(), wf))
                .collect(),
        )
    }

    fn get(&self, name: &str) -> Option<&Workflow> {
        self.0.get(name)
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn test(&self, machine_part: &MachinePart, workflows: &Workflows) -> anyhow::Result<bool> {
        for rule in &self.rules {
            if rule.test(machine_part) {
                return rule.apply(machine_part, workflows);
            }
        }
        bail!("No rule matched")
    }
}

struct Rule {
    condition: Option<Condition>,
    action: Action,
}

impl Rule {
    fn test(&self, machine_part: &MachinePart) -> bool {
        match &self.condition {
            Some(condition) => condition.test(machine_part),
            _ => true,
        }
    }

    fn apply(&self, machine_part: &MachinePart, workflows: &Workflows) -> anyhow::Result<bool> {
        match &self.action {
            Action::Accept => anyhow::Ok(true),
            Action::Reject => anyhow::Ok(false),
            Action::Redirect { workflow_name } => workflows
                .get(workflow_name)
                .ok_or_else(|| anyhow!("Unknown workflow {workflow_name}"))?
                .test(machine_part, workflows),
        }
    }
}

struct Condition {
    field: Field,
    operator: Operator,
    value: Value,
}

impl Condition {
    fn test(&self, machine_part: &MachinePart) -> bool {
        let value = match self.field {
            Field::X => machine_part.x,
            Field::M => machine_part.m,
            Field::A => machine_part.a,
            Field::S => machine_part.s,
        };
        match self.operator {
            Operator::Gt => value > self.value,
            Operator::Lt => value < self.value,
        }
    }
}

enum Field {
    X,
    M,
    A,
    S,
}

enum Operator {
    Gt,
    Lt,
}

enum Action {
    Reject,
    Accept,
    Redirect { workflow_name: String },
}

struct MachineParts(Vec<MachinePart>);

#[derive(Clone)]
struct MachinePart {
    x: Value,
    m: Value,
    a: Value,
    s: Value,
}

impl MachinePart {
    fn rate(&self) -> Value {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Workflows {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let workflows: Vec<Workflow> = s.lines().map(|line| line.parse()).try_collect()?;
        Ok(Self::new(workflows))
    }
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (_, name, rules) = lazy_regex::regex_captures!(r"^(\w+)\{(.*)\}$", s)
            .ok_or_else(|| anyhow!("Cannot parse workflows:{s}"))?;
        let rules = rules
            .split(",")
            .map(|rule| {
                anyhow::Ok(match rule.splitn(2, ":").collect_vec().as_slice() {
                    [condition, action] => Rule {
                        condition: Some(condition.parse()?),
                        action: action.parse()?,
                    },
                    [action] => Rule {
                        condition: None,
                        action: action.parse()?,
                    },
                    _ => bail!("Invalid rule {rule}"),
                })
            })
            .try_collect()?;

        anyhow::Ok(Self {
            name: name.to_string(),
            rules,
        })
    }
}

impl FromStr for Condition {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (_, field, operator, value) = lazy_regex::regex_captures!(r"^(\w+)([<>])(\d+)$", s)
            .ok_or_else(|| anyhow!("Cannot parse condition:{s}"))?;

        anyhow::Ok(Self {
            field: field.parse()?,
            operator: operator.parse()?,
            value: value.parse()?,
        })
    }
}

impl FromStr for Field {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        anyhow::Ok(match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => bail!("unknown field {s}"),
        })
    }
}

impl FromStr for Operator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        anyhow::Ok(match s {
            ">" => Self::Gt,
            "<" => Self::Lt,
            _ => bail!("unknown operator {s}"),
        })
    }
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        anyhow::Ok(match s {
            "R" => Self::Reject,
            "A" => Self::Accept,
            _ => Self::Redirect {
                workflow_name: s.to_string(),
            },
        })
    }
}

impl FromStr for MachinePart {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (_, x, m, a, s) =
            lazy_regex::regex_captures!(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}", s)
                .ok_or_else(|| anyhow!("Cannot parse part:{s}"))?;

        anyhow::Ok(Self {
            x: x.parse()?,
            m: m.parse()?,
            a: a.parse()?,
            s: s.parse()?,
        })
    }
}

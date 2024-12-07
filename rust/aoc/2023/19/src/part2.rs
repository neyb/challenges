use crate::*;
use anyhow::*;
use challenges_common::ranges::discontinuous;

type Res = u64;
pub(crate) fn run(content: &str) -> Result<Res> {
    let (workflows, _machine_parts) = parse_input(content)?;

    let accepted = workflows.accepted_ranges()?;

    Ok(accepted
        .iter()
        .map(|machine_part_range| machine_part_range.len())
        .sum())
}

trait Part2Workflows {
    fn accepted_ranges(&self) -> Result<Vec<MachinePartRange>>;
}

impl Part2Workflows for Workflows {
    fn accepted_ranges(&self) -> Result<Vec<MachinePartRange>> {
        let in_rule = self.get("in").ok_or_else(|| anyhow!("No in workflow"))?;

        Ok(in_rule.accepted(&[MachinePartRange::full_range()], self))
    }
}

type Range = discontinuous::Range<u16>;
#[derive(Debug, PartialEq, Clone)]
struct MachinePartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl MachinePartRange {
    fn full_range() -> Self {
        Self {
            x: Range::new_inclusive(1, 4000).unwrap(),
            m: Range::new_inclusive(1, 4000).unwrap(),
            a: Range::new_inclusive(1, 4000).unwrap(),
            s: Range::new_inclusive(1, 4000).unwrap(),
        }
    }

    fn split_by(&self, condition: &Condition) -> SplitResult {
        use super::Field::*;

        let split = |range: &Range| match condition.operator {
            Operator::Gt => {
                let passed = Range::new_inclusive(condition.value + 1, range.end);
                let failed = Range::new_inclusive(range.start, condition.value);
                (passed, failed)
            }
            Operator::Lt => {
                let passed = Range::new_inclusive(range.start, condition.value - 1);
                let failed = Range::new_inclusive(condition.value, range.end);
                (passed, failed)
            }
        };

        let with_updated_range = |range: Range| {
            let mut res = self.clone();
            *match condition.field {
                X => &mut res.x,
                M => &mut res.m,
                A => &mut res.a,
                S => &mut res.s,
            } = range;
            res
        };

        let (passed, failed) = match condition.field {
            X => split(&self.x),
            M => split(&self.m),
            A => split(&self.a),
            S => split(&self.s),
        };

        SplitResult {
            passed: passed.map(with_updated_range),
            failed: failed.map(with_updated_range),
        }
    }

    fn len(&self) -> Res {
        let len = |range: &Range| (range.end - range.start + 1) as Res;
        len(&self.x) * len(&self.m) * len(&self.a) * len(&self.s)
    }
}

struct SplitResult {
    passed: Option<MachinePartRange>,
    failed: Option<MachinePartRange>,
}

trait Part2Workflow {
    fn accepted(
        &self,
        machine_part: &[MachinePartRange],
        workflows: &Workflows,
    ) -> Vec<MachinePartRange>;
}

impl Part2Workflow for Workflow {
    fn accepted(
        &self,
        machine_part: &[MachinePartRange],
        workflows: &Workflows,
    ) -> Vec<MachinePartRange> {
        let mut accepted = Vec::with_capacity(machine_part.len());
        for machine_part_range in machine_part {
            let mut rest = machine_part_range.clone();
            'rules: for rule in &self.rules {
                let (passed, failed) = match &rule.condition {
                    Some(condition) => {
                        let result = rest.split_by(condition);
                        (result.passed, result.failed)
                    }
                    None => (Some(rest.clone()), None),
                };

                if let Some(passed) = passed {
                    accepted.extend(rule.action.apply_range(&passed, workflows));
                }

                match failed {
                    Some(failed) => rest = failed,
                    None => break 'rules,
                }
            }
        }
        accepted
    }
}

trait Part2Action {
    fn apply_range(
        &self,
        machine_part_range: &MachinePartRange,
        workflows: &Workflows,
    ) -> Vec<MachinePartRange>;
}

impl Part2Action for Action {
    fn apply_range(
        &self,
        machine_part_range: &MachinePartRange,
        workflows: &Workflows,
    ) -> Vec<MachinePartRange> {
        match self {
            Action::Accept => vec![machine_part_range.clone()],
            Action::Reject => Vec::new(),
            Action::Redirect { workflow_name } => workflows
                .get(workflow_name)
                .expect("Unknown workflow")
                .accepted(&[machine_part_range.clone()], workflows),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "19-test.txt"]);
        assert_eq!(run(&content).unwrap(), 167409079868000);
    }

    #[test]
    fn simple_test_1() {
        let workflows: Workflows = "in{s<1351:A,R}".parse().unwrap();

        assert_eq!(
            workflows.accepted_ranges().unwrap(),
            vec![MachinePartRange {
                x: Range::new_inclusive(1, 4000).unwrap(),
                m: Range::new_inclusive(1, 4000).unwrap(),
                a: Range::new_inclusive(1, 4000).unwrap(),
                s: Range::new_inclusive(1, 1350).unwrap(),
            }]
        );

        let content = "\
in{s<1351:A,R}

{x=787,m=2655,a=1222,s=2876}
";
        assert_eq!(run(content).unwrap(), 1350 * 4000 * 4000 * 4000);
    }

    #[test]
    fn simple_test_2() {
        let workflows: Workflows = "in{s<1351:R,A}".parse().unwrap();

        assert_eq!(
            workflows.accepted_ranges().unwrap(),
            vec![MachinePartRange {
                x: Range::new_inclusive(1, 4000).unwrap(),
                m: Range::new_inclusive(1, 4000).unwrap(),
                a: Range::new_inclusive(1, 4000).unwrap(),
                s: Range::new_inclusive(1351, 4000).unwrap(),
            }]
        );
    }
}

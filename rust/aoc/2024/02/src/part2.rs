use crate::Line;
use anyhow::*;
use itertools::Itertools;
use std::iter::once;

type Res = usize;
pub(crate) fn run(content: &String) -> Result<Res> {
    let lines: Vec<Line> = content.lines().map(|line| line.parse()).try_collect()?;
    Ok(lines.iter().filter(|line| line.is_safe_part2()).count())
}

trait Part2Line: Sized {
    fn possible_inputs(&self) -> Vec<Self>;
    fn is_safe_part2(&self) -> bool;
}

impl Part2Line for Line {
    fn possible_inputs(&self) -> Vec<Self> {
        once(self.clone())
            .chain((0..self.numbers.len()).map(|i| {
                let mut numbers = self.numbers.clone();
                numbers.remove(i);
                Self { numbers }
            }))
            .collect()
    }

    fn is_safe_part2(&self) -> bool {
        self.possible_inputs().iter().any(|line| line.is_safe())
    }
}

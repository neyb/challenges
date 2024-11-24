use crate::{Dig, DigPlan};
use anyhow::*;

pub(crate) fn run(content: &str) -> Result<usize> {
    let mut dig_plan: DigPlan = content.parse()?;
    dig_plan.fix()?;
    Ok(dig_plan.path()?.area())
}

trait Fix {
    fn fix(&mut self) -> Result<()>;
}

impl Fix for DigPlan {
    fn fix(&mut self) -> Result<()> {
        for dig in &mut self.digs {
            dig.fix()?
        }
        Ok(())
    }
}

impl Fix for Dig {
    fn fix(&mut self) -> Result<()> {
        let color = std::mem::take(&mut self.color);
        let (dist, dir) = color.split_at(5);
        self.count = usize::from_str_radix(dist, 16)?;
        use challenges_common::graph::Direction::*;
        self.direction = match dir {
            "0" => Right,
            "1" => Down,
            "2" => Left,
            "3" => Up,
            _ => bail!("unknown direction {dir}"),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "18-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 952408144115);
    }
}

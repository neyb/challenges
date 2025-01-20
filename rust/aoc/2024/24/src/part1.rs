use crate::Device;
use anyhow::*;
use challenges_common::MyIterTools;
use itertools::Itertools;
use std::str::FromStr;

type Res = u64;
pub(crate) fn run(content: &str) -> Result<Res> {
    let device: Device = content.parse()?;
    Ok(device.get_z_output())
}

trait Part2Device {
    fn get_z_output(&self) -> Res;
}

impl Part2Device for Device {
    fn get_z_output(&self) -> Res {
        let mut res = 0;

        for i in 0.. {
            let name = format!("z{:02}", i);

            let Some(value) = self.get_value(&name) else {
                break;
            };

            if value {
                res |= 1 << i;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "24-test1.txt"]);
        assert_eq!(run(&content).unwrap(), 4);
    }

    #[test]
    fn test_run2() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "24-test2.txt"]);
        assert_eq!(run(&content).unwrap(), 2024);
    }
}

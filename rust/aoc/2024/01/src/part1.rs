use anyhow::*;
use itertools::Itertools;

type Res = i32;
pub(crate) fn run(content: &String) -> Result<u32> {
    let lines: Vec<(&str, &str)> = content
        .lines()
        .map(|line| {
            line.split_once("   ")
                .ok_or_else(|| anyhow!("cannot parse line"))
        })
        .try_collect()?;
    let mut first: Vec<Res> = lines.iter().map(|ss| ss.0.parse()).try_collect()?;
    let mut second: Vec<Res> = lines.iter().map(|ss| ss.1.parse()).try_collect()?;

    first.sort();
    second.sort();

    Ok(first.iter().zip(second).map(|(f, s)| f.abs_diff(s)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_1() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "01-test.txt"]);
        assert_eq!(run(&content).unwrap(), 11);
    }
}

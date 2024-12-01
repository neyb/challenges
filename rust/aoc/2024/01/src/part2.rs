use anyhow::*;
use itertools::Itertools;

type Res = u32;
pub(crate) fn run(content: &String) -> Result<Res> {
    let lines: Vec<(&str, &str)> = content
        .lines()
        .map(|line| {
            line.split_once("   ")
                .ok_or_else(|| anyhow!("cannot parse line"))
        })
        .try_collect()?;
    let first: Vec<Res> = lines.iter().map(|ss| ss.0.parse()).try_collect()?;
    let second: Vec<Res> = lines.iter().map(|ss| ss.1.parse()).try_collect()?;

    let mut res = 0;
    for f in &first {
        for s in &second {
            if f == s {
                res += s;
            }
        }
    }

    Ok(res)
}

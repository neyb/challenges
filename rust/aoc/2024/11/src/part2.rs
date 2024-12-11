use crate::{StoneLine, TStone};
use anyhow::*;

type Res = TStone;
pub(crate) fn run(content: &str) -> Result<Res> {
    let mut stone_line: StoneLine = content.parse()?;
    for _ in 0..75 {
        stone_line.blink();
    }

    Ok(stone_line.len())
}

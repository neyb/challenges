use crate::{Coord, Map};
use anyhow::*;

pub(crate) fn run(content: &str) -> Result<Coord> {
    let mut map: Map = content.parse()?;
    let mut taken = map.sim(1024);

    while taken.is_some() && map.path().is_some() {
        taken = map.sim(1);
    }
    Ok(taken.unwrap())
}

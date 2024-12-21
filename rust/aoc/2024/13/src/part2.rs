use crate::{Machines, Unit};
use anyhow::*;

type Res = Unit;
pub(crate) fn run(content: &str) -> Result<Res> {
    let mut machines: Machines = content.parse()?;

    for machine in &mut machines.0 {
        machine.price.x += 10000000000000;
        machine.price.y += 10000000000000;
    }

    Ok(machines.min_cost())
}

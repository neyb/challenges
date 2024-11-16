use crate::{Load, Map};
use challenges_common::cycle::detect_cycle;

pub(crate) fn run(content: &String) -> anyhow::Result<Load> {
    let map: Map = content.parse()?;
    let Some(cycle) = detect_cycle(map.clone(), |map| {
        let mut map = map.clone();
        map.spin_cycle().unwrap();
        Some(map)
    }) else {
        anyhow::bail!("no cycle detected");
    };

    println!(
        "cycle of size {:?} starts at {:?}",
        cycle.size, cycle.start_index
    );
    let identical_cycle = ((1000000000 - cycle.start_index) % cycle.size) + cycle.start_index;
    println!("identical cycle: {:?}", identical_cycle);

    let mut map = map.clone();
    for _ in 0..identical_cycle {
        map.spin_cycle().unwrap();
    }
    Ok(map.get_north_load())
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 64);
    }
}

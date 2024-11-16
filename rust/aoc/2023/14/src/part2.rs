use crate::{Load, Map};
use challenges_common::cycle::detect_cycle;

pub(crate) fn run(content: &str) -> anyhow::Result<Load> {
    let mut map: Map = content.parse()?;

    for _ in 0..get_identical_cycle(&mut map)? {
        map.spin_cycle()?;
    }

    Ok(map.get_north_load())
}

fn get_identical_cycle(map: &Map) -> anyhow::Result<usize> {
    let Some(cycle) = detect_cycle(map.clone(), |map| {
        let mut map = map.clone();
        map.spin_cycle().unwrap();
        Some(map)
    }) else {
        anyhow::bail!("no cycle detected");
    };

    println!(
        "cycle of size {} starts at {}",
        cycle.size, cycle.start_index
    );
    let identical_cycle = ((1000000000 - cycle.start_index) % cycle.size) + cycle.start_index;
    println!("identical cycle: {identical_cycle}");

    Ok(identical_cycle)
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 64);
    }
}

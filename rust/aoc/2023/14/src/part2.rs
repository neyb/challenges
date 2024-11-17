use crate::{Load, Map};
use challenges_common::cycle;

pub(crate) fn run(content: &str) -> anyhow::Result<Load> {
    let map: Map = content.parse()?;

    let map = cycle::forecast_state(
        map,
        |map| {
            let mut map = map.clone();
            map.spin_cycle();
            Some(map)
        },
        1000000000,
    );

    Ok(map.get_north_load())
}

#[cfg(test)]
mod tests {
    use crate::{Map, Place};

    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 64);
    }

    #[test]
    fn after_3_cycles() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        let mut map: Map = content.parse().unwrap();
        map.spin_cycle();
        map.spin_cycle();
        map.spin_cycle();

        let expected: Map = "\
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
            .parse()
            .unwrap();

        assert_eq!(map, expected);
    }

    #[test]
    fn load_after_3_cycles() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        let mut map: Map = content.parse().unwrap();
        map.spin_cycle();
        map.spin_cycle();
        map.spin_cycle();

        assert_eq!(map.get_north_load(), 69);
    }

    #[test]
    fn after_100_cycle_should_have_same_amount_of_rounded_rocks() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        let mut map: Map = content.parse().unwrap();
        let count_node = |map: &Map| {
            map.grid
                .nodes()
                .iter()
                .filter(|place| place == &&Place::RoundRock)
                .count()
        };
        let start_count = count_node(&map);
        for _ in 0..100 {
            map.spin_cycle();
        }

        assert_eq!(count_node(&map), start_count);
    }
}

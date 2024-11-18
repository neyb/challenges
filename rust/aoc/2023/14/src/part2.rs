use crate::{Load, Platform};
use challenges_common::cycle;

pub(crate) fn run(content: &str) -> anyhow::Result<Load> {
    let platform: Platform = content.parse()?;

    let platform = cycle::forecast_state(
        platform,
        |platform| {
            let mut platform = platform.clone();
            platform.spin_cycle();
            Some(platform)
        },
        1000000000,
    );

    Ok(platform.get_north_load())
}

#[cfg(test)]
mod tests {
    use crate::{Place, Platform};

    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 64);
    }

    #[test]
    fn after_3_cycles() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        let mut platform: Platform = content.parse().unwrap();
        platform.spin_cycle();
        platform.spin_cycle();
        platform.spin_cycle();

        let expected: Platform = "\
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

        assert_eq!(platform, expected);
    }

    #[test]
    fn load_after_3_cycles() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        let mut platform: Platform = content.parse().unwrap();
        platform.spin_cycle();
        platform.spin_cycle();
        platform.spin_cycle();

        assert_eq!(platform.get_north_load(), 69);
    }

    #[test]
    fn after_100_cycle_should_have_same_amount_of_rounded_rocks() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "14-test.txt"]);
        let mut platform: Platform = content.parse().unwrap();
        let count_node = |platform: &Platform| {
            platform
                .grid
                .nodes()
                .iter()
                .filter(|place| place == &&Place::RoundRock)
                .count()
        };
        let start_count = count_node(&platform);
        for _ in 0..100 {
            platform.spin_cycle();
        }

        assert_eq!(count_node(&platform), start_count);
    }
}

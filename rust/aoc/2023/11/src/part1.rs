use anyhow::Result;

type Len = usize;
pub fn run(content: &str) -> Result<Len> {
    super::run(content, 2)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn given_test_distance_5_to_9_is_9() {
        assert_eq!(distance_between(5, 9), 9);
    }

    #[test]
    fn given_test_distance_1_and_7_is_15() {
        assert_eq!(distance_between(1, 7), 15);
    }

    #[test]
    fn given_test_distance_3_to_6_is_17() {
        assert_eq!(distance_between(3, 6), 17);
    }

    #[test]
    fn given_test_distance_8_to_9_is_5() {
        assert_eq!(distance_between(8, 9), 5);
    }

    fn distance_between(n1: usize, n2: usize) -> usize {
        let content = challenges_common::get_input_content(&["aoc", "2023", "11-test.txt"]);
        let universe: Universe = content.parse().unwrap();
        let galaxy1 = &universe.galaxies[n1 - 1];
        let galaxy2 = &universe.galaxies[n2 - 1];
        universe.distance_between(galaxy1, galaxy2)
    }

    #[test]
    fn there_are_36_pairs_in_given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "11-test.txt"]);
        let universe: Universe = content.parse().unwrap();
        assert_eq!(universe.galaxies_pairs().len(), 36);
    }

    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "11-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 374);
    }
}

use crate::{Price, Region};
use anyhow::*;

pub(crate) fn price(region: &Region) -> Price {
    region.area() * region.sides_count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run;

    #[test]
    fn given_test_1() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test1.txt"]);
        assert_eq!(run(&content, price).unwrap(), 80);
    }
    #[test]
    fn given_test_2() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test2.txt"]);
        assert_eq!(run(&content, price).unwrap(), 436);
    }
    #[test]
    fn given_test_3() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test3.txt"]);
        assert_eq!(run(&content, price).unwrap(), 1206);
    }

    #[test]
    fn test_eshape() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test-eshape.txt"]);
        assert_eq!(run(&content, price).unwrap(), 236);
    }

    #[test]
    fn test_slash() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test-slash.txt"]);
        assert_eq!(run(&content, price).unwrap(), 368);
    }
}

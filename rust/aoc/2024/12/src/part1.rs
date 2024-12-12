use crate::{Price, Region};

pub(crate) fn price(region: &Region) -> Price {
    region.area() * region.perimeter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run;

    #[test]
    fn given_test_1() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test1.txt"]);
        assert_eq!(run(&content, price).unwrap(), 140);
    }
    #[test]
    fn given_test_2() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test2.txt"]);
        assert_eq!(run(&content, price).unwrap(), 772);
    }
    #[test]
    fn given_test_3() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test3.txt"]);
        assert_eq!(run(&content, price).unwrap(), 1930);
    }
}

use crate::Price;

pub trait Region {
    fn price(&self) -> Price;
    fn perimeter(&self) -> usize;
}

impl Region for crate::Region {
    fn price(&self) -> Price {
        self.area() * self.perimeter()
    }

    fn perimeter(&self) -> usize {
        self.cells
            .iter()
            .map(|&cell| {
                cell.neighbours(false)
                    .filter(|neighbor| !self.cells.contains(neighbor))
                    .count()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run;

    #[test]
    fn given_test_1() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test1.txt"]);
        assert_eq!(run(&content, Region::price).unwrap(), 140);
    }
    #[test]
    fn given_test_2() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test2.txt"]);
        assert_eq!(run(&content, Region::price).unwrap(), 772);
    }
    #[test]
    fn given_test_3() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "12-test3.txt"]);
        assert_eq!(run(&content, Region::price).unwrap(), 1930);
    }
}

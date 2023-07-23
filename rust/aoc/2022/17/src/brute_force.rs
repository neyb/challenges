use crate::*;

pub(crate) fn rock_tower_high(jet_pattern: &JetPattern, nb_falls: NbFallsType) -> Result<YType> {
    let mut state = State::from(jet_pattern.clone());
    state.run_falls(nb_falls)?;
    Ok(state.height)
}

#[cfg(test)]
mod test {
    use crate::brute_force::*;

    #[test]
    fn given_test_after_0_fall() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17-test.txt"]);
        let height = rock_tower_high(&jet_pattern, 0).unwrap();
        assert_eq!(height, 0)
    }

    #[test]
    fn given_test_after_1_fall() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17-test.txt"]);
        let height = rock_tower_high(&jet_pattern, 1).unwrap();
        assert_eq!(height, 1)
    }

    #[test]
    fn given_test_after_2022_falls() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17-test.txt"]);
        let height = rock_tower_high(&jet_pattern, 2022).unwrap();
        assert_eq!(height, 3068)
    }

    #[test]
    fn answer_part1() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17.txt"]);
        assert_eq!(rock_tower_high(&jet_pattern, 2022).unwrap(), 3141)
    }

    mod landscape {
        use crate::{Coord, Landscape};

        #[test]
        fn can_insert_0x_0y() {
            let mut landscape = Landscape::new();
            let coord = Coord { x: 0, y: 0 };
            landscape.insert(&coord)
        }
    }
}

use std::ops::Range;

use anyhow::Result;

use rockfall::*;

use crate::*;

mod rockfall;

pub(crate) fn rock_tower_high(jet_pattern: &JetPattern, nb_falls: NbFallsType) -> Result<YType> {
    // because there is a limited list of fall possibilities, this is some kind of "eventual periodic function"
    // so we gonna search for this pattern

    // to identiy this pattern we will rely on the fact that what define a rock fall is :
    // - jet_state (index for us)
    // - the shape of the top (to simplify we
    //     gonna only consider column height, but this is just a pertinent approximation)
    // - which next rock will fall :
    //     there is 5 rock types => this pattern must be a multiple of 5 in rock falls

    // we gonna use turtle and hare to find a "falling cycle" in rock falls

    let mut rock_falls = RockFalls::new(jet_pattern.clone());

    let detected_cycle = detect_cycle(&mut rock_falls)?;

    let mut height_of_indexes = |mut range: Range<usize>| {
        range.try_fold(0 as YType, |height, index| {
            rock_falls
                .get(index)
                .map(|fall| height + fall.height_growth as u64)
        })
    };

    if nb_falls < (detected_cycle.starting_at_fall_index + detected_cycle.cycle_fall_count) as YType
    {
        height_of_indexes(0..(nb_falls as usize))
    } else {
        let nb_cycles = (nb_falls - detected_cycle.starting_at_fall_index as NbFallsType)
            / (detected_cycle.cycle_fall_count as NbFallsType);
        let nb_not_in_cycles =
            (nb_falls - (nb_cycles * detected_cycle.cycle_fall_count as NbFallsType)) as usize;
        let start_and_end_height = height_of_indexes(0..nb_not_in_cycles)?;
        let cycles_height = (nb_cycles * detected_cycle.height_growth) as YType;
        Ok(start_and_end_height + cycles_height)
    }
}

fn detect_cycle(rock_falls: &mut RockFalls) -> Result<DetectedCycle> {
    let mut turtle_index = 0usize;
    let mut hare_index = 0usize;

    // first round
    while {
        turtle_index += 1;
        hare_index += 2;
        rock_falls.get_num(turtle_index)?.clone() != rock_falls.get_num(hare_index)?.clone()
    } {}
    let first_meet_at = turtle_index;

    // second round
    hare_index = turtle_index;
    while {
        turtle_index += 1;
        hare_index += 2;
        rock_falls.get_num(turtle_index)?.clone() != rock_falls.get_num(hare_index)?.clone()
    } {}
    let second_meet_at = turtle_index;

    let cycle_size = second_meet_at - first_meet_at;

    let starting_at_fall_index = {
        let mut index = first_meet_at - 1;
        loop {
            if rock_falls.get(index)?.clone() == rock_falls.get(index + cycle_size)?.clone() {
                if index == 0 {
                    break 0;
                } else {
                    index -= 1;
                }
            } else {
                break index;
            }
        }
    };

    let height_growth = {
        (starting_at_fall_index..starting_at_fall_index + cycle_size)
            .map(|fall_index| rock_falls.get(fall_index).map(|fall| fall.height_growth))
            .try_fold(0 as YType, |sum, height_growth| {
                height_growth.map(|height_growth| sum + (height_growth as YType))
            })?
    };

    Ok(DetectedCycle {
        cycle_fall_count: cycle_size,
        starting_at_fall_index,
        height_growth,
    })
}

struct DetectedCycle {
    starting_at_fall_index: usize,
    cycle_fall_count: usize,
    height_growth: YType,
}

#[cfg(test)]
mod test {
    use crate::smarter::rockfall::RockFalls;
    use crate::JetPattern;

    use super::*;

    #[test]
    fn detect_cycle_on_sample() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17-test.txt"]);
        let mut rock_falls = RockFalls::new(jet_pattern);
        let cycle = detect_cycle(&mut rock_falls).unwrap();
        assert_eq!(cycle.starting_at_fall_index, 27);
        assert_eq!(cycle.cycle_fall_count, 35);
        assert_eq!(cycle.height_growth, 53);
    }

    #[test]
    fn rock_tower_high_after_0_turn() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17-test.txt"]);
        assert_eq!(rock_tower_high(&jet_pattern, 0).unwrap(), 0)
    }

    #[test]
    fn rock_tower_high_after_1_turn() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17-test.txt"]);
        assert_eq!(rock_tower_high(&jet_pattern, 1).unwrap(), 1)
    }

    #[test]
    fn rock_tower_high_after_2022_turns() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17-test.txt"]);
        assert_eq!(rock_tower_high(&jet_pattern, 2022).unwrap(), 3068)
    }

    #[test]
    fn rock_tower_high_after_2022_on_real() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17.txt"]);
        assert_eq!(rock_tower_high(&jet_pattern, 2022).unwrap(), 3141)
    }

    #[test]
    fn rock_tower_high_after_1000000000000_turns() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17-test.txt"]);
        assert_eq!(
            rock_tower_high(&jet_pattern, 1000000000000).unwrap(),
            1514285714288
        )
    }
}

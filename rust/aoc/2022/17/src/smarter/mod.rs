use crate::*;
use anyhow::Result;
use rockfall::*;
use std::ops::Range;

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
    let mut i = 0;
    let Some(cycle) = challenges_common::cycle::detect_cycle(rock_falls.get(i)?, |_| {
        i += 1;
        Some(rock_falls.get(i).unwrap())
    }) else {
        return Err(anyhow!("no cycle detected"));
    };

    let height_growth = {
        (cycle.start_index..cycle.start_index + cycle.size)
            .map(|fall_index| rock_falls.get(fall_index).map(|fall| fall.height_growth))
            .try_fold(0 as YType, |sum, height_growth| {
                height_growth.map(|height_growth| sum + (height_growth as YType))
            })?
    };

    Ok(DetectedCycle {
        cycle_fall_count: cycle.size,
        starting_at_fall_index: cycle.start_index,
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
        assert_eq!(cycle.starting_at_fall_index, 28);
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

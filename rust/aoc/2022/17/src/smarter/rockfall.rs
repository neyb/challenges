use std::rc::Rc;

use anyhow::{anyhow, Result};

use crate::*;

pub struct RockFalls {
    falls: Vec<Rc<RockFall>>,
    generator: RockFallsGenerator,
}

impl RockFalls {
    pub(crate) fn new(jet_pattern: JetPattern) -> Self {
        Self {
            falls: Vec::new(),
            generator: RockFallsGenerator::new(jet_pattern),
        }
    }

    pub fn get_num(&mut self, fall_num: usize) -> Result<Rc<RockFall>> {
        self.get(fall_num - 1)
    }

    pub fn get(&mut self, fall_index: usize) -> Result<Rc<RockFall>> {
        (self.falls.len()..fall_index + 1).try_for_each(|_| {
            self.generator
                .next()
                .unwrap() // infinite iterator
                .map(|fall| self.falls.push(Rc::new(fall)))
        })?;

        self.falls
            .get(fall_index)
            .cloned()
            .ok_or_else(|| anyhow!("fall index {} should have been added", fall_index))
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RockFall {
    pub input: RockFallState,
    pub height_growth: u8,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct RockFallState {
    rock_shape: RockShape,
    jet_index: usize,
    landscape_shape: Vec<u8>, // maybe array ?
}

struct RockFallsGenerator {
    state: State,
}

impl RockFallsGenerator {
    fn new(jet_pattern: JetPattern) -> Self {
        Self {
            state: State::from(jet_pattern),
        }
    }
}

impl Iterator for RockFallsGenerator {
    type Item = Result<RockFall>;

    fn next(&mut self) -> Option<Self::Item> {
        let input = self.state.rock_fall_input();
        let height_before_fall = self.state.height;
        Some(self.state.run_until_next_rock().map(|_| RockFall {
            input,
            height_growth: (self.state.height - height_before_fall) as u8,
        }))
    }
}

trait Part2State {
    fn rock_fall_input(&self) -> RockFallState;
}

impl Part2State for State {
    fn rock_fall_input(&self) -> RockFallState {
        let landscape_shape = (0..7)
            .map(|x| {
                (0..self.height)
                    .find(|y_from_top| {
                        self.contains(&Coord {
                            x,
                            y: self.height - y_from_top - 1,
                        })
                    })
                    .map(|y_from_top| y_from_top as u8)
                    .unwrap_or(self.height as u8)
            })
            .collect();

        RockFallState {
            jet_index: self.jet_state.next_index,
            rock_shape: self.falling_rock.shape,
            landscape_shape,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::smarter::rockfall::RockFalls;
    use crate::JetPattern;
    use crate::RockShape::*;

    #[test]
    fn fall_1() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17-test.txt"]);
        let mut rock_falls = RockFalls::new(jet_pattern);
        let fall = rock_falls.get_num(1).unwrap();
        assert_eq!(fall.input.rock_shape, Minus);
        assert_eq!(fall.input.landscape_shape, vec![0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(fall.height_growth, 1);
    }

    #[test]
    fn fall_2() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17-test.txt"]);
        let mut rock_falls = RockFalls::new(jet_pattern);
        let fall = rock_falls.get_num(2).unwrap();
        assert_eq!(fall.input.rock_shape, Plus);
        assert_eq!(fall.input.landscape_shape, vec![1, 1, 0, 0, 0, 0, 1]);
        assert_eq!(fall.height_growth, 3);
    }

    #[test]
    fn fall_3() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17-test.txt"]);
        let mut rock_falls = RockFalls::new(jet_pattern);
        let fall = rock_falls.get_num(3).unwrap();
        assert_eq!(fall.input.rock_shape, Corner);
        assert_eq!(fall.input.landscape_shape, vec![4, 4, 1, 0, 1, 3, 4]);
        assert_eq!(fall.height_growth, 2);
    }

    #[test]
    fn fall_4() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17-test.txt"]);
        let mut rock_falls = RockFalls::new(jet_pattern);
        let fall = rock_falls.get_num(4).unwrap();
        assert_eq!(fall.input.rock_shape, Pipe);
        assert_eq!(fall.input.landscape_shape, vec![2, 2, 0, 2, 3, 5, 6]);
        assert_eq!(fall.height_growth, 1);
    }

    #[test]
    fn fall_10() {
        let jet_pattern = JetPattern::parse_location(&["aoc", "2022", "17-test.txt"]);
        let mut rock_falls = RockFalls::new(jet_pattern);
        let fall = rock_falls.get_num(10).unwrap();
        assert_eq!(fall.input.rock_shape, Square);
        assert_eq!(fall.input.landscape_shape, vec![13, 5, 4, 4, 0, 2, 17]);
        assert_eq!(fall.height_growth, 0);
    }
}

use crate::common::*;
use std::ops::Range;

pub fn run(content: &str) -> Position {
    todo!("implement part 2")
}

struct Almanac {
    seeds: Category,
    transformations: Vec<Transformation>,
}

struct Category {
    ranges: Vec<Range<Position>>,
}

impl Category {
    fn new(ranges: Vec<Range<Position>>) -> Self {
        Self { ranges }
    }
}

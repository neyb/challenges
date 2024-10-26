use std::iter::Step;
use std::ops::Range;

struct Ranges<P> {
    ranges: Vec<Range<P>>,
}

impl <P> Ranges<P>  where P: PartialOrd + Copy + Step {
    fn remove(&mut self, range: Range<P>) {
        self.ranges = self
            .ranges
            .iter()
            .flat_map(|current_range| {
                match (
                    current_range.contains(&range.start),
                    current_range.contains(&range.end),
                ) {
                    (true, true) => vec![
                        current_range.start..range.end,
                        range.start + 1..current_range.end,
                    ],
                    (true, false) => vec![Range {
                        start: range.end,
                        end: current_range.end,
                    }],
                    (false, true) => vec![Range {
                        start: current_range.start,
                        end: range.start,
                    }],
                    (false, false) => vec![current_range.clone()],
                }
            })
            .filter(|r| !r.contains(&range.start) && !r.contains(&range.end))
            .collect();
    }
    
}
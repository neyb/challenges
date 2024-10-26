use itertools::Either;
use std::fmt::Debug;

mod continuous;
mod discontinuous;

pub struct Ranges<R: Range> {
    ranges: Vec<R>,
}

impl<R: Range> Ranges<R> {
    fn empty() -> Self {
        Self { ranges: Vec::new() }
    }

    fn new(ranges: Vec<R>) -> Self {
        let mut result = Self { ranges };
        result.simplify();
        result
    }

    fn remove_ranges(&mut self, ranges: &Ranges<R>) -> Self {
        let mut all_removed = Self::empty();
        for range in ranges.ranges.iter() {
            let removed = self.remove_range(range);
            all_removed.ranges.extend(removed.ranges);
        }
        all_removed
    }

    fn remove_range(&mut self, range: &R) -> Self {
        let mut all_removed = Self { ranges: Vec::new() };

        let previous_ranges = std::mem::take(&mut self.ranges);
        for previous_range in previous_ranges {
            let WithoutResult { removed, remaining } = previous_range.without(range);
            if let Some(removed) = removed {
                all_removed.ranges.push(removed);
            }
            match remaining {
                Remaining::Joined(range) => self.ranges.push(range),
                Remaining::Splitted(first, second) => {
                    self.ranges.push(first);
                    self.ranges.push(second);
                }
            }
        }
        self.simplify();
        all_removed.simplify();

        all_removed
    }

    fn simplify(&mut self) {
        self.ranges.sort_by_key(|r| (r.start(), r.end()));
        let mut current = None;

        let ranges = std::mem::take(&mut self.ranges);

        for range in ranges {
            match &current {
                None => {
                    current = Some(range);
                }
                Some(current_range) => match current_range.join(&range) {
                    JoinedResult::Joined(joined) => {
                        current = Some(joined);
                    }
                    JoinedResult::Disjoint => {
                        self.ranges.push(current.replace(range).unwrap());
                    }
                },
            };
        }

        if let Some(current) = current {
            self.ranges.push(current);
        }
    }
}

trait Range: Sized {
    type Element: Ord + Copy;

    fn start(&self) -> Self::Element;
    fn end(&self) -> Self::Element;

    fn contains(&self, element: Self::Element) -> bool;

    fn overlap(&self, other: &Self) -> bool;

    fn intersection(&self, other: &Self) -> Option<Self>;
    fn join(&self, other: &Self) -> JoinedResult<Self>;
    fn without(&self, other: &Self) -> WithoutResult<Self>;
}

enum JoinedResult<R> {
    Joined(R),
    Disjoint,
}

struct WithoutResult<R> {
    remaining: Remaining<R>,
    removed: Option<R>,
}

enum Remaining<R> {
    Joined(R),
    Splitted(R, R),
}

#[cfg(test)]
mod tests {
    use super::*;

    mod discontinuous {
        use super::super::*;

        #[test]
        fn empty_ranges_should_be_empty() {
            let ranges:Ranges<discontinuous::NotEmptyRange<i32>> = Ranges::empty();
            assert_eq!(ranges.ranges.len(), 0);
        }

        #[test]
        fn can_create_ranges() {
            let range = discontinuous::NotEmptyRange::new(1, 2);
            let mut ranges = Ranges::new(vec![range]);
            assert_eq!(ranges.ranges.len(), 1);
        }
    }
}

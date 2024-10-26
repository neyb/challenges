use std::fmt::Debug;

pub mod continuous;
pub mod discontinuous;

#[derive(Debug, PartialEq)]
pub struct Ranges<R: Range> {
    pub ranges: Vec<R>,
}

impl<R: Range> Ranges<R> {
    pub fn empty() -> Self {
        Self { ranges: Vec::new() }
    }

    pub fn new(ranges: Vec<R>) -> Self {
        let mut result = Self { ranges };
        result.simplify();
        result
    }

    pub fn map(&self, mutation: impl Fn(&R) -> R) -> Self {
        Self {
            ranges: self.ranges.iter().map(|range| mutation(range)).collect(),
        }
    }

    pub fn merge(&mut self, ranges: Ranges<R>) {
        self.ranges.extend(ranges.ranges);
        self.simplify()
    }

    fn remove_ranges(&mut self, ranges: &Ranges<R>) -> Self {
        let mut all_removed = Self::empty();
        for range in ranges.ranges.iter() {
            let removed = self.remove_range(range);
            all_removed.ranges.extend(removed.ranges);
        }
        all_removed
    }

    pub fn remove_range(&mut self, range: &R) -> Self {
        let mut all_removed = Self { ranges: Vec::new() };

        let previous_ranges = std::mem::take(&mut self.ranges);
        for previous_range in previous_ranges {
            let WithoutResult { removed, remaining } = previous_range.without(range);
            if let Some(removed) = removed {
                all_removed.ranges.push(removed);
            }
            match remaining {
                Remaining::Empty => {}
                Remaining::Single(range) => self.ranges.push(range),
                Remaining::Splitted { before, after } => {
                    self.ranges.push(before);
                    self.ranges.push(after);
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
                    JoinedResult::Disjoint(_, _) => {
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

impl<R> Clone for Ranges<R>
where
    R: Range + Clone,
{
    fn clone(&self) -> Self {
        Self {
            ranges: self.ranges.clone(),
        }
    }
}

pub trait Range: Sized {
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
    Disjoint(R, R),
}

struct WithoutResult<R> {
    remaining: Remaining<R>,
    removed: Option<R>,
}

enum Remaining<R> {
    Empty,
    Single(R),
    Splitted { before: R, after: R },
}

#[cfg(test)]
mod tests {
    mod discontinuous {
        use super::super::*;

        #[test]
        fn empty_ranges_should_be_empty() {
            let ranges: Ranges<discontinuous::Range<i32>> = Ranges::empty();
            assert_eq!(ranges.ranges.len(), 0);
        }

        #[test]
        fn can_create_ranges() {
            let range = discontinuous::Range::new(1, 2).unwrap();
            let mut ranges = Ranges::new(vec![range]);
            assert_eq!(ranges.ranges.len(), 1);
        }

        #[test]
        fn simplify_should_join_1_to_2_and_2_to_4() {
            let mut ranges = Ranges::new(vec![
                discontinuous::Range::new(1, 2).unwrap(),
                discontinuous::Range::new(2, 4).unwrap(),
            ]);

            assert_eq!(
                ranges.ranges,
                vec![discontinuous::Range::new(1, 4).unwrap()]
            )
        }

        #[test]
        fn simplify_should_join_1_to_2_and_3_to_4() {
            let mut ranges = Ranges::new(vec![
                discontinuous::Range::new(1, 2).unwrap(),
                discontinuous::Range::new(3, 4).unwrap(),
            ]);

            assert_eq!(
                ranges.ranges,
                vec![discontinuous::Range::new(1, 4).unwrap()]
            )
        }
    }
}

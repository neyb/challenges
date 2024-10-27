use crate::ranges::{JoinedResult, Remaining, WithoutResult};

#[derive(Clone, PartialEq, Debug)]
pub struct Range<P> {
    pub start: P,
    pub end: P,
}

impl<P> Range<P>
where
    P: Ord + Copy + Stepable,
{
    pub fn new(start: P, end: P) -> Option<Self> {
        if start <= end {
            Some(Self { start, end })
        } else {
            None
        }
    }
}

impl<P> Range<P>
where
    P: Copy + Ord + num_traits::Num,
{
    pub fn with_length(start: P, length: P) -> Option<Self> {
        if length > P::zero() {
            Some(Self {
                start,
                end: start + length + P::one(),
            })
        } else {
            None
        }
    }
}

impl<P> super::Range for Range<P>
where
    P: Ord + Stepable + Copy,
{
    type Element = P;

    fn start(&self) -> Self::Element {
        self.start
    }

    fn end(&self) -> Self::Element {
        self.end
    }

    fn contains(&self, point: Self::Element) -> bool {
        self.start <= point && point <= self.end
    }

    fn overlap(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        Self::new(start, end)
    }

    fn join(&self, other: &Self) -> JoinedResult<Self> {
        if self.overlap(other)
            || other.end().next().unwrap() == self.start
            || self.end.next().unwrap() == other.start()
        {
            let start = self.start.min(other.start);
            let end = self.end.max(other.end);
            JoinedResult::Joined(Self::new(start, end).unwrap())
        } else {
            JoinedResult::Disjoint(self.clone(), other.clone())
        }
    }

    fn without(&self, other: &Self) -> WithoutResult<Self> {
        match self.intersection(other) {
            None => WithoutResult {
                remaining: Remaining::Single(self.clone()),
                removed: None,
            },
            Some(removed) => {
                let remaining_before = removed.start.prev().and_then(|end| Self::new(self.start, end));
                let remaining_after = removed.end.next().and_then(|start| Self::new(start, self.end));

                match (remaining_before, remaining_after) {
                    (Some(before), Some(after)) => WithoutResult {
                        remaining: Remaining::Splitted { before, after },
                        removed: Some(removed),
                    },
                    (Some(single), _) | (_, Some(single)) => WithoutResult {
                        remaining: Remaining::Single(single),
                        removed: Some(removed),
                    },
                    _ => WithoutResult {
                        remaining: Remaining::Empty,
                        removed: Some(removed),
                    },
                }
            }
        }
    }
}

impl<P> Range<P>
where
    P: Ord + Copy + Stepable + std::ops::Sub<Output=P>,
{
    fn len(&self) -> P {
        self.end - self.start
    }
}

impl<P> From<&std::ops::Range<P>> for Range<P>
where
    P: Stepable + Copy,
{
    fn from(value: &std::ops::Range<P>) -> Self {
        Self {
            start: value.start,
            end: value.end.prev().unwrap(),
        }
    }
}

pub trait Stepable: Sized {
    fn next(&self) -> Option<Self>;
    fn prev(&self) -> Option<Self>;
}

macro_rules! impl_stepable_num {
    ($($t:ty),*) => {
        $(
            impl Stepable for $t {
                fn next(&self) -> Option<Self> {
                    self.checked_add(1)
                }

                fn prev(&self) -> Option<Self> {
                    self.checked_sub(1)
                }
            }
        )*
    };
}

impl_stepable_num!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);


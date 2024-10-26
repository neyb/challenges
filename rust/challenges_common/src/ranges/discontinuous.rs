use crate::ranges::{JoinedResult, WithoutResult};

pub enum Range<P> {
    Empty,
    NotEmpty(NotEmptyRange<P>),
}

struct NotEmptyRange<P> {
    start: P,
    end: P,
}

impl<P> super::Range for NotEmptyRange<P>
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
        self.contains(other.start)
            || self.contains(other.end)
            || other.contains(self.start)
            || other.contains(self.end)
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        todo!()
    }

    fn join(&self, other: &Self) -> JoinedResult<Self> {
        todo!()
    }

    fn without(&self, other: &Self) -> WithoutResult<Self> {
        todo!()
    }
}

impl<P> NotEmptyRange<P>
where
    P: Ord + Copy + Stepable,
{
    pub fn new(start: P, end: P) -> Self {
        Self { start, end }
    }
}

impl<P> NotEmptyRange<P>
where
    P: Ord + Copy + Stepable + std::ops::Sub<Output = P>,
{
    fn len(&self) -> P {
        self.end - self.start
    }
}

pub trait Stepable {
    fn next(&self) -> Self;
    fn prev(&self) -> Self;
}

macro_rules! impl_stepable {
    ($($t:ty),*) => {
        $(
            impl Stepable for $t {
                fn next(&self) -> Self {
                    self + 1
                }

                fn prev(&self) -> Self {
                    self - 1
                }
            }
        )*
    };
}

impl_stepable!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

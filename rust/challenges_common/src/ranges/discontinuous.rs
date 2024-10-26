struct Range<P>
where
    P: PartialOrd + Stepable,
{
    start: P,
    end: P,
}

impl<P> Range<P>
where
    P: PartialOrd + Stepable,
{
    fn contains(&self, point: &P) -> bool {
        self.start <= *point && *point < self.end
    }
}

impl<P> Range<P>
where
    P: PartialOrd + Copy + Stepable + std::ops::Sub<Output = P>,
{
    fn len(&self) -> P {
        self.end - self.start
    }
}

pub trait Stepable {
    fn next(&self) -> Self;
    fn prev(&self) -> Self;
}

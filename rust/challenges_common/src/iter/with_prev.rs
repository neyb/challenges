pub struct MapWithPrev<I, F, Out>
where
    I: Iterator,
    F: FnMut((I::Item, Option<&Out>)) -> Out,
{
    iter: I,
    prev: Option<Out>,
    f: F,
}

impl<It, F, Out> MapWithPrev<It, F, Out>
where
    It: Iterator,
    F: FnMut((It::Item, Option<&Out>)) -> Out,
    Out: Clone,
{
    pub(crate) fn new(iter: It, f: F) -> Self {
        Self {
            iter,
            f,
            prev: None,
        }
    }
}

impl<It, F, Out> Iterator for MapWithPrev<It, F, Out>
where
    It: Iterator,
    F: FnMut((It::Item, Option<&Out>)) -> Out,
    Out: Clone,
{
    type Item = Out;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        let next_mapped = (self.f)((next, self.prev.as_ref()));
        self.prev = Some(next_mapped.clone());
        Some(next_mapped)
    }
}

pub struct MapWithPrevWithDefault<It, F, Out>
where
    It: Iterator,
    F: FnMut(It::Item, &Out) -> Out,
    Out: Clone,
{
    iter: It,
    prev: Out,
    f: F,
}

impl<It, F, Out> MapWithPrevWithDefault<It, F, Out>
where
    It: Iterator,
    F: FnMut(It::Item, &Out) -> Out,
    Out: Clone,
{
    pub(crate) fn new(iter: It, f: F, default: Out) -> Self {
        Self {
            iter,
            f,
            prev: default,
        }
    }
}

impl<It, F, Out> Iterator for MapWithPrevWithDefault<It, F, Out>
where
    It: Iterator,
    F: FnMut(It::Item, &Out) -> Out,
    Out: Clone,
{
    type Item = Out;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        let next_mapped = (self.f)(next, &self.prev);
        self.prev = next_mapped.clone();
        Some(next_mapped)
    }
}

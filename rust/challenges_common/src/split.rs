pub struct SplitIterator<T, It, Splitter>
where
    It: Iterator<Item = T>,
    Splitter: Fn(&T) -> bool,
{
    inner: It,
    splitter: Splitter,
}

impl<T, It, Splitter> SplitIterator<T, It, Splitter>
where
    It: Iterator<Item = T>,
    Splitter: Fn(&T) -> bool,
{
    pub fn new(inner: It, splitter: Splitter) -> Self
    where
        It: Iterator<Item = T> + Sized,
        Splitter: Fn(&T) -> bool + Sized,
    {
        Self { inner, splitter }
    }
}

impl<T, It, Splitter> Iterator for SplitIterator<T, It, Splitter>
where
    It: Iterator<Item = T>,
    Splitter: Fn(&T) -> bool,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        while let Some(item) = self.inner.next() {
            if !(self.splitter)(&item) {
                result.get_or_insert_with(Vec::new).push(item);
            } else {
                break;
            }
        }
        result
    }
}

pub struct SplitIterator<It, Splitter>
where
    It: Iterator,
    Splitter: Fn(&It::Item) -> bool,
{
    inner: It,
    splitter: Splitter,
}

impl<It, Splitter> SplitIterator<It, Splitter>
where
    It: Iterator,
    Splitter: Fn(&It::Item) -> bool,
{
    pub fn new(inner: It, splitter: Splitter) -> Self
    where
        It: Iterator + Sized,
        Splitter: Fn(&It::Item) -> bool + Sized,
    {
        Self { inner, splitter }
    }
}

impl<It, Splitter> Iterator for SplitIterator<It, Splitter>
where
    It: Iterator,
    Splitter: Fn(&It::Item) -> bool,
{
    type Item = Vec<It::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        for item in self.inner.by_ref() {
            let vec = result.get_or_insert_with(Vec::new);
            if !(self.splitter)(&item) {
                vec.push(item);
            } else {
                break;
            }
        }
        result
    }
}

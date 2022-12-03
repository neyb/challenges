mod split;

pub use split::SplitIterator;

trait MyIterTools {
    type Element;

    fn split<Splitter>(self, splitter: Splitter) -> SplitIterator<Self::Element, Self, Splitter>
    where
        Splitter: Fn(&Self::Item) -> bool,
        Self: Iterator<Item = Self::Element> + Sized;
}

impl<T> MyIterTools for dyn Iterator<Item = T> {
    type Element = T;

    fn split<Splitter>(self, splitter: Splitter) -> SplitIterator<Self::Element, Self, Splitter>
    where
        Splitter: Fn(&T) -> bool,
        Self: Iterator + Sized,
    {
        SplitIterator::new(self, splitter)
    }
}

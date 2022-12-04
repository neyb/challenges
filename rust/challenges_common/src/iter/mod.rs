mod split;

use split::SplitIterator;

pub trait MyIterTools {
    fn split<Splitter>(self, splitter: Splitter) -> SplitIterator<Self, Splitter>
    where
        Splitter: Fn(&Self::Item) -> bool,
        Self: Iterator + Sized;
}

impl<It> MyIterTools for It
where
    It: Iterator + Sized,
{
    fn split<Splitter>(self, splitter: Splitter) -> SplitIterator<Self, Splitter>
    where
        Splitter: Fn(&It::Item) -> bool,
    {
        SplitIterator::new(self, splitter)
    }
}

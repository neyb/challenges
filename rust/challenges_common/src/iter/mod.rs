use split::SplitIterator;

use crate::iter::chunk::ChunkStarting;

mod chunk;
mod split;

pub trait MyIterTools {
    fn split<Splitter>(self, splitter: Splitter) -> SplitIterator<Self, Splitter>
    where
        Splitter: Fn(&Self::Item) -> bool,
        Self: Iterator + Sized;

    fn chunks_starting_by<Splitter>(self, splitter: Splitter) -> ChunkStarting<Self, Splitter>
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

    fn chunks_starting_by<Splitter>(self, splitter: Splitter) -> ChunkStarting<Self, Splitter>
    where
        Splitter: Fn(&It::Item) -> bool,
    {
        ChunkStarting::new(self, splitter)
    }
}

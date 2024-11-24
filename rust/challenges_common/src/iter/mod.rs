use crate::iter::with_prev::{MapWithPrev, MapWithPrevWithDefault};
use split::SplitIterator;

mod chunk;
mod split;
mod with_prev;

pub trait MyIterTools: Iterator + Sized {
    fn split<Splitter>(self, splitter: Splitter) -> SplitIterator<Self, Splitter>
    where
        Splitter: Fn(&Self::Item) -> bool;

    fn chunks_starting_by<Splitter>(
        self,
        splitter: Splitter,
    ) -> chunk::ChunkStarting<Self, Splitter>
    where
        Splitter: Fn(&Self::Item) -> bool;

    fn map_with_prev<F, Out>(self, f: F) -> MapWithPrev<Self, F, Out>
    where
        F: FnMut((Self::Item, Option<&Out>)) -> Out,
        Out: Clone,
    {
        MapWithPrev::new(self, f)
    }

    fn map_with_prev_from<F, Out>(self, init: Out, f: F) -> MapWithPrevWithDefault<Self, F, Out>
    where
        F: FnMut(Self::Item, &Out) -> Out,
        Out: Clone,
    {
        MapWithPrevWithDefault::new(self, f, init)
    }
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

    fn chunks_starting_by<Splitter>(
        self,
        splitter: Splitter,
    ) -> chunk::ChunkStarting<Self, Splitter>
    where
        Splitter: Fn(&It::Item) -> bool,
    {
        chunk::ChunkStarting::new(self, splitter)
    }
}

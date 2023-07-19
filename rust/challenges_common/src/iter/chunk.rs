pub struct ChunkStarting<It, Splitter>
where
    It: Iterator,
    Splitter: Fn(&It::Item) -> bool,
{
    inner: It,
    splitter: Splitter,
    last_splitter: Option<It::Item>,
}

impl<It, Splitter> ChunkStarting<It, Splitter>
where
    It: Iterator,
    Splitter: Fn(&It::Item) -> bool,
{
    pub(crate) fn new(inner: It, splitter: Splitter) -> Self {
        Self {
            last_splitter: None,
            inner,
            splitter,
        }
    }
}

impl<It, Splitter> ChunkStarting<It, Splitter>
where
    It: Iterator,
    Splitter: Fn(&It::Item) -> bool,
{
}

impl<It, Splitter> Iterator for ChunkStarting<It, Splitter>
where
    It: Iterator,
    Splitter: Fn(&It::Item) -> bool,
{
    type Item = Vec<It::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        for item in self.inner.by_ref() {
            let vec = result.get_or_insert_with(Vec::new);
            if let Some(last_splitter) = self.last_splitter.take() {
                vec.push(last_splitter)
            }

            if vec.is_empty() || !(self.splitter)(&item) {
                vec.push(item);
            } else {
                self.last_splitter = Some(item);
                break;
            }
        }

        result.or_else(|| self.last_splitter.take().map(|empty_last| vec![empty_last]))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_output(input: Vec<&str>, expected_output: Vec<Vec<&str>>) {
        let iter = input.into_iter();

        let chunks: Vec<_> = ChunkStarting {
            inner: iter,
            splitter: |line| line.starts_with('$'),
            last_splitter: None,
        }
        .collect();

        assert_eq!(chunks, expected_output);
    }

    #[test]
    fn three_chunks() {
        test_output(
            vec![
                "$start 1", "item 1 1", "$start 2", "item 2 1", "$start 3", "item 3 1",
            ],
            vec![
                vec!["$start 1", "item 1 1"],
                vec!["$start 2", "item 2 1"],
                vec!["$start 3", "item 3 1"],
            ],
        );
    }

    #[test]
    fn three_empty_chunks() {
        test_output(
            vec!["$start 1", "$start 2", "$start 3"],
            vec![vec!["$start 1"], vec!["$start 2"], vec!["$start 3"]],
        );
    }
}

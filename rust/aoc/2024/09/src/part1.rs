use crate::{FileSystem, Res};
use anyhow::*;

pub(crate) fn run(content: &str) -> Result<Res> {
    let mut fs: FileSystem = content.parse()?;
    fs.compact();
    Ok(fs.checksum())
}

trait Part2FileSystem {
    fn compact(&mut self);
}

impl Part2FileSystem for FileSystem {
    fn compact(&mut self) {
        while let Some((empty_index, mut filled_index)) = get_regions_index_to_swap(self) {
            let id = self.regions[filled_index].id.unwrap();

            match self.regions[empty_index]
                .size
                .cmp(&self.regions[filled_index].size)
            {
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => {
                    self.split(empty_index, self.regions[filled_index].size);
                    filled_index += 1;
                }
                std::cmp::Ordering::Less => {
                    self.split(
                        filled_index,
                        self.regions[filled_index].size - self.regions[empty_index].size,
                    );
                    filled_index += 1;
                }
            };
            self.regions[empty_index].id = Some(id);
            self.regions[filled_index].id = None;
        }

        fn get_regions_index_to_swap(fs: &FileSystem) -> Option<(usize, usize)> {
            let first_empty = fs.regions.iter().position(|region| region.id.is_none());
            let last_filled = fs.regions.iter().rposition(|region| region.id.is_some());

            match (first_empty, last_filled) {
                (Some(first_empty), Some(last_filled)) if first_empty < last_filled => {
                    Some((first_empty, last_filled))
                }
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "09-test.txt"]);
        assert_eq!(run(&content).unwrap(), 1928);
    }
}

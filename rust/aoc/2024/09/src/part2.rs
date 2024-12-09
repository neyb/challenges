use crate::{FileSystem, Id};
use anyhow::*;

pub(crate) fn run(content: &str) -> Result<crate::Res> {
    let mut fs: FileSystem = content.parse()?;
    fs.compact()?;
    Ok(fs.checksum())
}

trait Part2FileSystem {
    fn compact(&mut self) -> Result<()> {
        {
            let Some(last_id) = self.last_id() else {
                return Ok(());
            };

            for id in (0..=last_id).rev() {
                let from_index = self.position_for_id(id).unwrap();
                let size = self.size_of(from_index).unwrap();
                match self.position_with_free_size(size) {
                    Some(to_index) if to_index < from_index => {
                        self.swap_file(from_index, to_index);
                    }
                    _ => {}
                }
            }

            Ok(())
        }
    }

    fn last_id(&self) -> Option<Id>;
    fn position_for_id(&self, id: Id) -> Option<usize>;
    fn size_of(&self, index: usize) -> Option<u8>;
    fn position_with_free_size(&self, size: u8) -> Option<usize>;
    fn swap_file(&mut self, from_index: usize, to_index: usize);
}

impl Part2FileSystem for FileSystem {
    fn last_id(&self) -> Option<Id> {
        self.regions
            .iter()
            .rfind(|region| region.id.is_some())
            .and_then(|region| region.id)
    }

    fn position_for_id(&self, id: Id) -> Option<usize> {
        self.regions.iter().position(|region| region.id == Some(id))
    }

    fn size_of(&self, index: usize) -> Option<u8> {
        self.regions.get(index).map(|region| region.size)
    }

    fn position_with_free_size(&self, size: u8) -> Option<usize> {
        self.regions
            .iter()
            .position(|region| region.id.is_none() && region.size >= size)
    }

    fn swap_file(&mut self, from_index: usize, to_index: usize) {
        let id = self.regions[from_index].id.unwrap();
        self.regions[from_index].id = None;
        self.split(to_index, self.regions[from_index].size);
        self.regions[to_index].id = Some(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "09-test.txt"]);
        assert_eq!(run(&content).unwrap(), 2858);
    }
}

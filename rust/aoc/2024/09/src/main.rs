use anyhow::{anyhow, Error};
use itertools::Itertools;
use std::str::FromStr;

type Res = u64;
type Id = u16;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "09.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

struct FileSystem {
    regions: Vec<Region>,
}

impl FileSystem {
    fn checksum(&self) -> Res {
        self.regions.iter().map(|region| region.checksum()).sum()
    }

    fn split(&mut self, index: usize, size: u8) {
        if let Some(other) = self.regions[index].split(size) {
            let other_index = index + 1;
            self.regions.insert(other_index, other);
        }
    }
}

struct Region {
    size: u8,
    id: Option<Id>,
    start_index: usize,
}

impl Region {
    fn split(&mut self, size: u8) -> Option<Region> {
        let orig_size = self.size;

        self.size = orig_size.min(size);

        if size < orig_size {
            Some(Region {
                size: orig_size - size,
                id: self.id,
                start_index: self.start_index + size as usize,
            })
        } else {
            None
        }
    }

    fn checksum(&self) -> Res {
        (self.start_index..self.start_index + self.size as usize)
            .filter_map(|i| self.id.map(|id| (i, id)))
            .map(|(i, id)| i as Res * id as Res)
            .sum()
    }
}

impl FromStr for FileSystem {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut is_filled = true;
        let mut id = 0;
        let mut start_index = 0;

        let regions = s
            .chars()
            .map(|char| -> anyhow::Result<Region> {
                let region_size: u8 =
                    char.to_digit(10)
                        .ok_or_else(|| anyhow!("{char} is not a digit"))? as u8;
                let region = if is_filled {
                    let region = Region {
                        size: region_size,
                        id: Some(id),
                        start_index,
                    };
                    id += 1;
                    region
                } else {
                    Region {
                        size: region_size,
                        id: None,
                        start_index,
                    }
                };
                is_filled = !is_filled;
                start_index += region_size as usize;
                anyhow::Ok(region)
            })
            .try_collect()?;

        anyhow::Ok(Self { regions })
    }
}

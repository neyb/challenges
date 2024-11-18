use std::mem;
use std::str::FromStr;

use challenges_common::graph::{Coord, Grid};

fn main() -> anyhow::Result<()> {
    let content = challenges_common::get_input_content(&["aoc", "2023", "03.txt"]);
    let map = content.parse()?;
    println!("part 1: {}", part_1::run(&map).unwrap());
    println!("part 2: {}", part_2::run(&map).unwrap());
    Ok(())
}

struct Map {
    grid: Grid<char>,
    numbers: Vec<(u32, Vec<Coord>)>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let grid: Grid<char> = s.lines().map(|line| line.chars()).into();

        let mut numbers = Vec::new();

        for y in 0..grid.height() {
            let mut num_positions = Vec::new();
            let mut num_buffer = String::new();

            for x in 0..grid.width() {
                let coord = Coord { x, y };
                match grid.at(&coord) {
                    Some(c) if c.is_ascii_digit() => {
                        num_positions.push(coord);
                        num_buffer.push(*c);
                    }
                    Some(_) if !num_buffer.is_empty() => {
                        let num = num_buffer.parse()?;
                        num_buffer = String::new();
                        let completed_num_position = mem::take(&mut num_positions);
                        numbers.push((num, completed_num_position));
                    }
                    _ => {}
                }
            }

            if !num_positions.is_empty() {
                let num = num_buffer.parse()?;
                numbers.push((num, num_positions));
            }
        }

        Ok(Self { numbers, grid })
    }
}

mod part_1 {
    use crate::Map;
    use anyhow::Result;

    pub fn run(map: &Map) -> Result<u32> {
        Ok(map.parts_numbers_sum())
    }

    trait Part1Map {
        fn parts_numbers_sum(&self) -> u32;
    }

    impl Part1Map for Map {
        fn parts_numbers_sum(&self) -> u32 {
            self.numbers
                .iter()
                .filter_map(|(number, positions)| {
                    let is_part_number = positions
                        .iter()
                        .flat_map(|position| position.neighbours(true))
                        .any(|position| matches!(self.grid.at(&position), Some(&c) if !c.is_ascii_digit() && c != '.'));

                    if is_part_number {
                        Some(number)
                    } else {
                        None
                    }
                })
                .sum()
        }
    }

    #[cfg(test)]
    mod test {
        use crate::Map;
        use itertools::Itertools;

        #[test]
        fn test() {
            let input = r"
                        467..114..
                        ...*......
                        ..35..633.
                        ......#...
                        617*......
                        .....+.58.
                        ..592.....
                        ......755.
                        ...$.*....
                        .664.598.."
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.trim())
                .join("\n");

            assert_eq!(super::run(&input.parse::<Map>().unwrap()).unwrap(), 4361)
        }
    }
}

mod part_2 {
    use crate::Map;
    use anyhow::Result;
    use itertools::Itertools;

    pub fn run(map: &Map) -> Result<u32> {
        Ok(map.gears_ratio_sum())
    }

    trait Part2Map {
        fn gears_ratio_sum(&self) -> u32;
    }

    impl Part2Map for Map {
        fn gears_ratio_sum(&self) -> u32 {
            self.grid
                .coords()
                .filter(|coord| self.grid.at(coord) == Some(&'*'))
                .filter_map(|coord| {
                    let gear_neighbours_coord = coord.neighbours(true).collect_vec();

                    let numbers = self
                        .numbers
                        .iter()
                        .filter(|(_, coords)| {
                            coords
                                .iter()
                                .any(|number_coord| gear_neighbours_coord.contains(number_coord))
                        })
                        .map(|(number, _)| number)
                        .collect_vec();

                    match numbers.as_slice() {
                        [&n1, &n2] => Some(n1 * n2),
                        _ => None,
                    }
                })
                .sum()
        }
    }
}

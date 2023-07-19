use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, Error, Result};
use lazy_regex::regex;

fn main() {
    let sensors = parse(&["aoc", "2022", "15.txt"]).unwrap();
    println!("part1 : {}", part1(&sensors, 2_000_000));
    println!("part2: {}", part2(&sensors, 4000000).unwrap())
}

type Int = i32;
type UInt = u32;

fn parse(path: &[&str]) -> Result<Vec<Sensor>> {
    challenges_common::get_input_lines(path)
        .map(|line| line.parse::<Sensor>())
        .collect()
}

fn part1(sensors: &[Sensor], row: Int) -> usize {
    let ranges = ranges_at(sensors, row);

    let nb_beacon_to_deduct = sensors
        .iter()
        .map(|sensor| &sensor.closest_beacon_position)
        .filter(|beacon| beacon.y == row)
        .filter(|beacon| ranges.contains_value(beacon.x))
        .collect::<HashSet<_>>()
        .len();

    ranges.len() - nb_beacon_to_deduct
}

fn ranges_at(sensors: &[Sensor], row: i32) -> Ranges {
    let ranges = sensors
        .iter()
        .filter_map(|sensor| sensor.excluded_beacon_positions_at_row(row));

    Ranges::from(ranges)
}

fn part2(sensors: &[Sensor], max: Int) -> Option<u64> {
    let row_range = Range { from: 0, to: max };

    for y in 0..=max {
        let ranges = ranges_at(sensors, y);
        if !ranges.contains_range(&row_range) {
            let x = ranges
                .ranges
                .into_iter()
                .find(|range| row_range.contains(range.to))
                .unwrap()
                .to
                + 1;

            return Some(x as u64 * 4_000_000 + y as u64);
        }
    }

    None
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Coord {
    x: Int,
    y: Int,
}

impl Coord {
    // fn apply(&self, vector: &Vector) -> Self {
    //     Self {
    //         x: self.x + vector.x,
    //         y: self.y + vector.y,
    //     }
    // }

    fn to(&self, to: &Self) -> Vector {
        Vector {
            x: to.x - self.x,
            y: to.y - self.y,
        }
    }
}

struct Vector {
    x: Int,
    y: Int,
}

impl Vector {
    fn manhattan(&self) -> UInt {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

struct Sensor {
    position: Coord,
    to_sensor: Vector,
    closest_beacon_position: Coord,
}

impl Sensor {
    fn excluded_beacon_positions_at_row(&self, row: Int) -> Option<Range> {
        let y_diff = row - self.position.y;
        let x_diff = self.to_sensor.manhattan() as Int - y_diff.abs();

        if x_diff >= 0 {
            Some(Range {
                from: self.position.x - x_diff,
                to: self.position.x + x_diff,
            })
        } else {
            None
        }
    }
}

impl FromStr for Sensor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let regex = regex!(
            "Sensor at x=([0-9-]+), y=([0-9-]+): closest beacon is at x=([0-9-]+), y=([0-9-]+)"
        );

        let captures = regex
            .captures(s)
            .ok_or(anyhow!(r#""{}"does not match regex"#, s))?;

        let get = |position: usize| {
            captures
                .get(position)
                .ok_or(anyhow!("no capture at {}", position))?
                .as_str()
                .parse::<Int>()
                .map_err(|err| anyhow!(err))
        };

        let position = Coord {
            x: get(1)?,
            y: get(2)?,
        };

        let closest_sensor_position = Coord {
            x: get(3)?,
            y: get(4)?,
        };

        let to_sensor = position.to(&closest_sensor_position);

        Ok(Self {
            position,
            to_sensor,
            closest_beacon_position: closest_sensor_position,
        })
    }
}

struct Ranges {
    ranges: Vec<Range>,
}

impl Ranges {
    fn from(ranges: impl IntoIterator<Item = Range>) -> Self {
        let mut result = Self {
            ranges: ranges.into_iter().collect(),
        };
        result.simplify();
        result
    }

    fn simplify(&mut self) {
        self.ranges.sort_by(|r1, r2| r1.from.cmp(&r2.from));
        let mut current = None;

        let ranges = std::mem::take(&mut self.ranges);

        for range in ranges {
            match &current {
                None => {
                    current = Some(range);
                }
                Some(current_range) => {
                    if current_range.overlap(&range) {
                        current = Some(current_range.join(&range));
                    } else {
                        self.ranges.push(current.replace(range).unwrap());
                    }
                }
            };
        }

        if let Some(current) = current {
            self.ranges.push(current);
        }
    }

    fn len(&self) -> usize {
        self.ranges
            .iter()
            .map(|r| (r.to - r.from) as usize + 1)
            .sum()
    }

    fn contains_value(&self, value: Int) -> bool {
        self.ranges.iter().any(|range| range.contains(value))
    }

    fn contains_range(&self, row_range: &Range) -> bool {
        self.ranges
            .iter()
            .any(|range| range.from <= row_range.from && row_range.to <= range.to)
    }
}

struct Range {
    from: Int,
    to: Int,
}

impl Range {
    fn overlap(&self, other: &Self) -> bool {
        !(other.to < self.from || self.to < other.from)
    }

    fn join(&self, other: &Self) -> Self {
        Self {
            from: self.from.min(other.from),
            to: self.to.max(other.to),
        }
    }

    fn contains(&self, value: i32) -> bool {
        self.from <= value && value <= self.to
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn given_test_part1() {
        let sensors = parse(&["aoc", "2022", "15-test.txt"]).unwrap();
        assert_eq!(part1(&sensors, 10), 26)
    }

    #[test]
    fn given_test_part2() {
        let sensors = parse(&["aoc", "2022", "15-test.txt"]).unwrap();
        assert_eq!(part2(&sensors, 20).unwrap(), 56000011)
    }
}

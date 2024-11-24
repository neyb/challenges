use anyhow::anyhow;
use itertools::Itertools;
use std::ops::Deref;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "06.txt"]);
    println!("part 1: {}", part1(&content));
    println!("part 2: {}", part2(&content));
}

type Time = u64;
type Distance = u64;

fn part1(input: &str) -> usize {
    let races: Races = part1_parse(input).unwrap();
    races
        .iter()
        .map(|race| race.count_acceleration_beating())
        .product()
}

fn part1_parse(input: &str) -> anyhow::Result<Races> {
    let mut lines = input.lines();

    let times: Vec<Time> = lines
        .next()
        .ok_or_else(|| anyhow!("No times line"))?
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse())
        .try_collect()?;

    let distances: Vec<Distance> = lines
        .next()
        .ok_or_else(|| anyhow!("No distances line"))?
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse())
        .try_collect()?;

    Ok(std::iter::zip(times, distances)
        .map(|(time, distance)| Race::new(time, distance))
        .collect_vec()
        .into())
}

fn part2(input: &str) -> usize {
    let races: Races = part2_parse(input).unwrap();
    races
        .iter()
        .map(|race| race.count_acceleration_beating())
        .product()
}

fn part2_parse(input: &str) -> anyhow::Result<Races> {
    let mut lines = input.lines();

    let time: Time = lines
        .next()
        .ok_or_else(|| anyhow!("No times line"))?
        .split_whitespace()
        .skip(1)
        .join("")
        .parse()?;

    let distance: Distance = lines
        .next()
        .ok_or_else(|| anyhow!("No distances line"))?
        .split_whitespace()
        .skip(1)
        .join("")
        .parse()?;

    Ok(vec![Race {
        time,
        distance_to_beat: distance,
    }]
    .into())
}

#[derive(Debug, PartialEq)]
struct Races(Vec<Race>);

impl From<Vec<Race>> for Races {
    fn from(races: Vec<Race>) -> Self {
        Races(races)
    }
}

impl Deref for Races {
    type Target = Vec<Race>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
struct Race {
    time: Time,
    distance_to_beat: Distance,
}

impl Race {
    fn new(time: Time, distance_to_beat: Distance) -> Self {
        Self {
            time,
            distance_to_beat,
        }
    }

    fn count_acceleration_beating(&self) -> usize {
        (0..=self.time)
            .map(|time| self.distance(time))
            .filter(|distance| distance > &self.distance_to_beat)
            .count()
    }

    fn distance(&self, acceleration_time: Time) -> Distance {
        let running_time = self.time - acceleration_time;
        let speed = acceleration_time;
        running_time * speed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_race_count() {
        let race = Race {
            time: 7,
            distance_to_beat: 9,
        };
        assert_eq!(race.count_acceleration_beating(), 4)
    }

    #[test]
    fn second_race_count() {
        let race = Race {
            time: 15,
            distance_to_beat: 40,
        };
        assert_eq!(race.count_acceleration_beating(), 8)
    }

    #[test]
    fn third_race_count() {
        let race = Race {
            time: 30,
            distance_to_beat: 200,
        };
        assert_eq!(race.count_acceleration_beating(), 9)
    }

    #[test]
    fn parsing_part1_given_test() {
        let input = challenges_common::get_input_content(&["aoc", "2023", "06-test.txt"]);
        assert_eq!(
            part1_parse(&input).unwrap(),
            vec![
                Race {
                    time: 7,
                    distance_to_beat: 9
                },
                Race {
                    time: 15,
                    distance_to_beat: 40
                },
                Race {
                    time: 30,
                    distance_to_beat: 200
                },
            ]
            .into()
        )
    }

    #[test]
    fn parsing_part2_given_test() {
        let input = challenges_common::get_input_content(&["aoc", "2023", "06-test.txt"]);
        let races = part2_parse(&input).unwrap();
        assert_eq!(
            races,
            vec![Race {
                time: 71530,
                distance_to_beat: 940200
            },]
            .into()
        )
    }

    #[test]
    fn given_test_part1() {
        let input = challenges_common::get_input_content(&["aoc", "2023", "06-test.txt"]);
        assert_eq!(part1(&input), 288);
    }

    #[test]
    fn given_test_part2() {
        let input = challenges_common::get_input_content(&["aoc", "2023", "06-test.txt"]);
        assert_eq!(part2(&input), 71503);
    }
}

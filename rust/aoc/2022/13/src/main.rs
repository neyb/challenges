use std::{cmp::Ordering, fmt::Debug, iter::Peekable, str::FromStr};

use anyhow::{anyhow, Context, Result};
use challenges_common::MyIterTools;
use itertools::{EitherOrBoth, Itertools};

fn main() {
    let entries: Vec<Entry> = parse(&["aoc", "2022", "13.txt"]);
    println!("part1 : {}", part1(&entries));
    println!("part2 : {}", part2(&entries));
}

fn parse(path: &[&str]) -> Vec<Entry> {
    challenges_common::get_input_lines(path)
        .split(|line| line.is_empty())
        .enumerate()
        .map(|(index, lines)| Entry {
            index: index + 1,
            left: lines[0].parse().unwrap(),
            right: lines[1].parse().unwrap(),
        })
        .collect()
}

fn part1(entries: &Vec<Entry>) -> usize {
    entries
        .iter()
        .filter(|entry| entry.is_ordered())
        .map(|entry| entry.index)
        .sum()
}

fn part2(entries: &Vec<Entry>) -> u16 {
    let divider_2 = Value::Integer(2);
    let divider_6 = Value::Integer(6);

    let (index_2, index_6) = entries
        .iter()
        .flat_map(|entry| vec![entry.left.clone(), entry.right.clone()])
        .fold((1, 2), |(smaller_than_2, smaller_than_6), value| {
            if value < divider_2 {
                (smaller_than_2 + 1, smaller_than_6 + 1)
            } else if value < divider_6 {
                (smaller_than_2, smaller_than_6 + 1)
            } else {
                (smaller_than_2, smaller_than_6)
            }
        });

    index_2 * index_6
}

#[derive(Debug)]
struct Entry {
    index: usize,
    left: Value,
    right: Value,
}

impl Entry {
    fn is_ordered(&self) -> bool {
        self.left < self.right
    }
}

#[derive(Clone)]
enum Value {
    Integer(u8),
    List(Vec<Value>),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{:?}", value),
            Self::List(values) => write!(f, "{:?}", values),
        }
    }
}

impl Value {
    fn from_chars(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<Self> {
        match chars.peek() {
            Some('[') => {
                let mut values = Vec::new();

                while {
                    match chars.next() {
                        Some('[' | ',') => {
                            if chars.peek() == Some(&']') {
                                chars.next();
                                false
                            } else {
                                true
                            }
                        }

                        Some(']') => false,
                        Some(char) => Err(anyhow!("unexpected char: {}", char))?,
                        None => Err(anyhow!("unexpected End of sequence"))?,
                    }
                } {
                    values.push(Value::from_chars(chars)?);
                }

                Ok(Value::List(values))
            }
            Some(char) if char.is_digit(10) => {
                let value_as_str: String =
                    chars.peeking_take_while(|char| char.is_digit(10)).collect();
                Ok(Value::Integer(value_as_str.parse()?))
            }
            char => Err(anyhow!("unexpected char : {:?}", char)),
        }
    }
}

impl Eq for Value {}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Ordering::*;
        use Value::*;

        match (self, other) {
            (Integer(left), Integer(right)) => left.cmp(right),
            (List(left), List(right)) => left
                .iter()
                .zip_longest(right.iter())
                .find_map(|either_or_both| match either_or_both {
                    EitherOrBoth::Both(left, right) => match left.cmp(right) {
                        Equal => None,
                        cmp @ _ => Some(cmp),
                    },
                    EitherOrBoth::Left(_) => Some(Greater),
                    EitherOrBoth::Right(_) => Some(Less),
                })
                .unwrap_or(Equal),
            (Integer(_), List(_)) => List(vec![self.clone()]).cmp(other),
            (List(_), Integer(_)) => self.cmp(&List(vec![other.clone()])),
        }
    }
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Value::from_chars(&mut s.chars().peekable()).context(format!("error parsing \"{}\"", s))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn first_given_input_is_right() {
        use Value::*;
        let entry = Entry {
            index: 1,
            left: List(vec![
                Integer(1),
                Integer(1),
                Integer(3),
                Integer(1),
                Integer(1),
            ]),
            right: List(vec![
                Integer(1),
                Integer(1),
                Integer(5),
                Integer(1),
                Integer(1),
            ]),
        };

        assert!(entry.is_ordered())
    }

    #[test]
    fn test_parsing_12_is_ok() {
        assert_eq!(Value::from_str("12").unwrap(), Value::Integer(12));
    }

    #[test]
    fn parsing_empty_array() {
        assert_eq!(Value::from_str("[]").unwrap(), Value::List(vec![]))
    }

    #[test]
    fn parsing_an_array_containing_an_empty_array() {
        let value: Value = "[1,[],3]".parse().unwrap();

        use crate::Value::*;
        assert_eq!(value, List(vec![Integer(1), List(Vec::new()), Integer(3)]))
    }

    #[test]
    fn parsing_simple_array_is_ok() {
        assert_eq!(
            Value::from_str("[1,2,33,4]").unwrap(),
            Value::List(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(33),
                Value::Integer(4),
            ])
        )
    }

    #[test]
    fn fourth_given_entry() {
        let entry = Entry {
            index: 4,
            left: "[[4,4],4,4]".parse().unwrap(),
            right: "[[4,4],4,4,4]".parse().unwrap(),
        };

        assert!(entry.is_ordered())
    }

    #[test]
    fn parsing_array_of_arrays_is_ok() {
        let value: Value = "[[1,2],[3,4]]".parse().unwrap();

        assert_eq!(
            value,
            Value::List(vec![
                Value::List(vec![Value::Integer(1), Value::Integer(2)]),
                Value::List(vec![Value::Integer(3), Value::Integer(4)]),
            ])
        )
    }

    #[test]
    fn given_test_part1_indexes() {
        let values = parse(&["aoc", "2022", "13-test.txt"]);

        let ordered_indexes = values
            .iter()
            .filter(|entry| entry.is_ordered())
            .map(|entry| entry.index)
            .collect_vec();

        assert_eq!(ordered_indexes, vec![1, 2, 4, 6]);
    }

    #[test]
    fn given_test_part1() {
        let values = parse(&["aoc", "2022", "13-test.txt"]);
        assert_eq!(part1(&values), 13)
    }

    #[test]
    fn no_lemurs_test() {
        let entry = Entry {
            index:4,
            left:Value::from_str("[[3,[1,8],0]]").unwrap(),
            right: Value::from_str("[[[3],[[6,9,9]],4,[[],8],9],[[[1,0,7],[2,1,4],[0,9,4,10,2]],[[]],1,1],[[8,10,7],[6],5],[4]]").unwrap()
        };

        assert!(entry.is_ordered())
    }
}

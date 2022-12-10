extern crate challenges_common;

use std::str::FromStr;

use anyhow::{Error, Result};

fn main() {
    let lines = challenges_common::get_input_lines(&["aoc", "2022", "2.txt"]).collect::<Vec<_>>();

    let rounds = parse1(&lines).unwrap();
    println!("sum is: {}", score_sum(&rounds));

    let rounds = parse2(&lines).unwrap();
    println!("sum 2 is: {}", score_sum(&rounds));
}

fn parse1(lines: &Vec<String>) -> Result<Vec<Round>> {
    lines
        .iter()
        .map(|line| {
            let mut splitted = line.split(" ");
            let opponent_move = splitted.next().unwrap().parse()?;
            let your_move = splitted.next().unwrap().parse()?;
            Ok(Round {
                your_move,
                opponent_move,
            })
        })
        .collect()
}

fn parse2(lines: &Vec<String>) -> Result<Vec<Round>> {
    lines
        .iter()
        .map(|line| {
            let mut splitted = line.split(" ");
            let opponent_move = splitted.next().unwrap().parse()?;
            let your_move = match (&opponent_move, splitted.next().unwrap()) {
                (Move::Paper, "Z") | (Move::Scissors, "Y") | (Move::Rock, "X") => Move::Scissors,
                (Move::Paper, "Y") | (Move::Scissors, "X") | (Move::Rock, "Z") => Move::Paper,
                (Move::Paper, "X") | (Move::Scissors, "Z") | (Move::Rock, "Y") => Move::Rock,
                (_, e) => Err(Error::msg(format!(
                    "{:?} to {} not covered",
                    &opponent_move, e
                )))?,
            };
            Ok(Round {
                your_move,
                opponent_move,
            })
        })
        .collect()
}

fn score_sum(rounds: &Vec<Round>) -> u32 {
    rounds.iter().map(Round::points).sum()
}

#[derive(Debug, PartialEq)]
struct Round {
    opponent_move: Move,
    your_move: Move,
}

impl Round {
    fn points(&self) -> u32 {
        let move_points: u32 = match self.your_move {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        let oucome_points: u32 = match (&self.your_move, &self.opponent_move) {
            _ if self.your_move == self.opponent_move => 3,
            (Move::Paper, Move::Rock)
            | (Move::Rock, Move::Scissors)
            | (Move::Scissors, Move::Paper) => 6,
            _ => 0,
        };

        move_points + oucome_points
    }
}

#[derive(PartialEq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(Error::msg("char does not match a move")),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{parse2, score_sum, Move::*, Round};

    #[test]
    fn part2_given_test() {
        let lines = vec!["A Y".to_string(), "B X".to_string(), "C Z".to_string()];
        let rounds = parse2(&lines).unwrap();
        assert_eq!(
            &rounds,
            &vec![
                Round {
                    your_move: Rock,
                    opponent_move: Rock
                },
                Round {
                    your_move: Rock,
                    opponent_move: Paper
                },
                Round {
                    your_move: Rock,
                    opponent_move: Scissors
                },
            ]
        );
        assert_eq!(score_sum(&rounds), 12)
    }
}

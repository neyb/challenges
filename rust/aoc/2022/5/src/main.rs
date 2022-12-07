use challenges_common::MyIterTools;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let (cargo, moves) = parse(challenges_common::get_input_lines(vec![
        "aoc", "2022", "5.txt",
    ]));

    println!("part1: {}", part1(&mut cargo.clone(), &moves));
    println!("part2: {}", part2(&mut cargo.clone(), &moves));
}

fn parse(lines: impl Iterator<Item = String>) -> (Cargo, Vec<Move>) {
    let (cargo, moves) = lines.split(String::is_empty).collect_tuple().unwrap();

    let cargo = Cargo::from(&cargo);
    let moves = moves.iter().map(Move::from).collect_vec();

    (cargo, moves)
}

fn part1(cargo: &mut Cargo, moves: &Vec<Move>) -> String {
    for a_move in moves {
        cargo.apply(a_move)
    }

    cargo.result()
}

fn part2(cargo: &mut Cargo, moves: &Vec<Move>) -> String {
    for a_move in moves {
        cargo.apply_9001(a_move)
    }

    cargo.result()
}

#[derive(Clone)]
struct Cargo {
    stacks: Vec<Stack>,
}

impl Cargo {
    fn apply(&mut self, a_move: &Move) {
        for _ in 0..a_move.nb_crates {
            let crate_to_move = self.get_mut(a_move.from).crates.pop().unwrap();
            self.get_mut(a_move.to).crates.push(crate_to_move);
        }
    }

    fn apply_9001(&mut self, a_move: &Move) {
        let from = &self.get(a_move.from).crates;
        let (from, to_move) = from.split_at(from.len() - a_move.nb_crates as usize);
        let from = Vec::from(from);
        let to_move = Vec::from(to_move);
        self.get_mut(a_move.from).crates = from;
        self.get_mut(a_move.to).crates.extend(to_move.into_iter());
    }

    fn get(&self, position: u8) -> &Stack {
        self.stacks.get(position as usize - 1).unwrap()
    }

    fn get_mut(&mut self, position: u8) -> &mut Stack {
        self.stacks.get_mut(position as usize - 1).unwrap()
    }

    fn result(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.crates[stack.crates.len() - 1])
            .join("")
    }
}

impl From<&Vec<String>> for Cargo {
    fn from(input: &Vec<String>) -> Self {
        let lines = input
            .iter()
            .map(|line| {
                line.chars()
                    .chunks(4)
                    .into_iter()
                    .map(|mut chunk| {
                        chunk.next().unwrap();
                        chunk.next().unwrap()
                    })
                    .collect_vec()
            })
            .rev()
            .collect_vec();

        let nb_crates = lines.get(0).unwrap().len();

        let stacks = (0..nb_crates)
            .map(|i| {
                lines
                    .iter()
                    .skip(1)
                    .map(|line| line[i])
                    .filter(|c| c != &' ')
                    .collect_vec()
            })
            .map(|crates| Stack { crates })
            .collect_vec();

        Cargo { stacks }
    }
}

#[derive(Clone)]
struct Stack {
    crates: Vec<char>,
}

struct Move {
    nb_crates: u8,
    from: u8,
    to: u8,
}

// static move_regex: Regex =
//     Regex::new(r"move (?P<times>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
impl From<&String> for Move {
    fn from(s: &String) -> Self {
        lazy_static! {
            static ref MOVE_REGEX: Regex =
                Regex::new(r"move (?P<times>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
        }

        let caps = MOVE_REGEX.captures(s).unwrap();
        Self {
            nb_crates: caps["times"].parse().unwrap(),
            from: caps["from"].parse().unwrap(),
            to: caps["to"].parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn given_test_part1() {
        let (mut cargo, moves) = parse(
            vec![
                "    [D]    ",
                "[N] [C]    ",
                "[Z] [M] [P]",
                " 1   2   3 ",
                "",
                "move 1 from 2 to 1",
                "move 3 from 1 to 3",
                "move 2 from 2 to 1",
                "move 1 from 1 to 2",
            ]
            .iter()
            .map(|l| l.to_string()),
        );

        assert_eq!("CMZ", part1(&mut cargo, &moves));
    }

    #[test]
    fn given_test_part2() {
        let (mut cargo, moves) = parse(
            vec![
                "    [D]    ",
                "[N] [C]    ",
                "[Z] [M] [P]",
                " 1   2   3 ",
                "",
                "move 1 from 2 to 1",
                "move 3 from 1 to 3",
                "move 2 from 2 to 1",
                "move 1 from 1 to 2",
            ]
            .iter()
            .map(|l| l.to_string()),
        );

        assert_eq!("MCD", part2(&mut cargo, &moves));
    }
}

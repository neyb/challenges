extern crate challenges_common;
extern crate itertools;

use std::io::{BufRead, BufReader, Read};

use itertools::Itertools;

use challenges_common::{get_input_file, MyIterTools};

fn main() {
    let input = get_input_file(&["aoc", "2022", "1.txt"]);
    let elves = parse(input);

    part1(&elves);
    part2(&elves);
}

fn part1(elves: &Vec<u32>) {
    println!("max is {}", elves.iter().max().unwrap());
}

fn part2(elves: &Vec<u32>) {
    let sum: u32 = elves.iter().sorted_by(|a, b| b.cmp(a)).take(3).sum();
    println!("max 3 sum is {}", sum)
}

fn parse<R: Read + Sized>(input: R) -> Vec<u32> {
    BufReader::new(input)
        .lines()
        .map(|line_res| line_res.unwrap())
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|s| u32::from_str_radix(s, 10).unwrap())
                .sum()
        })
        .collect()
}

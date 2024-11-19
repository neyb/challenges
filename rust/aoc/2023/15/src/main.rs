use std::num::Wrapping;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "15.txt"]);
    println!("part 1: {}", part1::run(&content).unwrap());
    println!("part 2: {}", part2::run(&content).unwrap());
}

mod part1;
mod part2;

fn hash(s: &str) -> u8 {
    s.chars()
        .map(|char| Wrapping(char as u8))
        .fold(Wrapping(0), |h, c| (h + c) * Wrapping(17))
        .0
}

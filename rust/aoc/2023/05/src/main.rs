extern crate core;

mod common;
mod part1;
mod part2;
mod ranges;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "05.txt"]);
    println!("part 1: {}", part1::run(&content));
    println!("part 2: {}", part2::run(&content));
}

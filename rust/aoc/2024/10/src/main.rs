fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "10.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

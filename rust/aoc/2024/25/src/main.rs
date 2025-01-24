fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "25.txt"]);
    println!("part1: {:?}", part1::run(&content));
}

mod part1;

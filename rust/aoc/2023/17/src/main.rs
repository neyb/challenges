fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "17.txt"]);
    println!("part1: {}", part1::run(&content).unwrap());
}

mod part1;

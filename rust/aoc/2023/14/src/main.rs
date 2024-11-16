fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "14.txt"]);
    println!("part 1: {:?}", part1::run(&content).unwrap());
    println!("part 2: {:?}", part2::run(&content).unwrap());
}

type Load = usize;

mod part1;
mod part2;

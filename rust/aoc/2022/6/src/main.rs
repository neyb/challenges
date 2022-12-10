use itertools::Itertools;

fn main() {
    let first_line = challenges_common::get_input_lines(&["aoc", "2022", "6.txt"])
        .next()
        .unwrap();

    println!("part1 {}", first_line.first_marker_position(4));
    println!("part1 {}", first_line.first_marker_position(14));
}

trait Packet {
    fn first_marker_position(&self, marker_size: usize) -> usize;
}

impl Packet for String {
    fn first_marker_position(&self, marker_size: usize) -> usize {
        self.chars()
            .collect_vec()
            .windows(marker_size)
            .position(|window| window.iter().all_unique())
            .unwrap()
            + marker_size
    }
}

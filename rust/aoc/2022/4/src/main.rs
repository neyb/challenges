use itertools::Itertools;

type Range = std::ops::Range<u16>;

fn main() {
    let lines = challenges_common::get_input_lines(vec!["aoc", "2022", "4.txt"]).unwrap();

    let pairs = lines.iter().map(parse).collect_vec();
    println!("part1: {}", part1(&pairs));
    println!("part2: {}", part2(&pairs));
}

fn parse(line: &String) -> (Range, Range) {
    let (r1, r2) = line.split(",").collect_tuple().unwrap();
    (range_of(r1), range_of(r2))
}

fn part1(pairs: &[(Range, Range)]) -> usize {
    pairs
        .iter()
        .filter(|(r1, r2)| fully_overlap(r1, r2))
        .count()
}

fn part2(pairs: &[(Range, Range)]) -> usize {
    pairs.iter().filter(|(r1, r2)| overlap(r1, r2)).count()
}

fn fully_overlap(range1: &Range, range2: &Range) -> bool {
    let range2_in_range1 = range1.start <= range2.start && range1.end >= range2.end;
    let range1_in_range2 = range1.start >= range2.start && range1.end <= range2.end;
    range1_in_range2 || range2_in_range1
}

fn overlap(range1: &Range, range2: &Range) -> bool {
    let range1_after_range2 = range1.start > range2.end;
    let range2_after_range1 = range2.start > range1.end;
    !(range1_after_range2 || range2_after_range1)
}

fn range_of(s: &str) -> Range {
    let (start, end) = s.split("-").collect_tuple().unwrap();
    Range {
        start: start.parse().unwrap(),
        end: end.parse().unwrap(),
    }
}

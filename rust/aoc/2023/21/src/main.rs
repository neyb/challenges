use challenges_common::graph::CannotParseElementFromChar;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2023", "21.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content, 26501365));
}

mod part1;
mod part2;

struct Position<U = u8> {
    position_type: PositionType,
    explore_range: Option<U>,
}

impl<U> Position<U> {
    fn should_be_explored(&self) -> bool {
        self.position_type == PositionType::Empty && self.explore_range.is_none()
    }
}

#[derive(PartialEq, Copy, Clone)]
enum PositionType {
    Start,
    Empty,
    Rock,
}

impl TryFrom<char> for Position {
    type Error = CannotParseElementFromChar;

    fn try_from(value: char) -> anyhow::Result<Self, Self::Error> {
        let position_type = match value {
            '.' => PositionType::Empty,
            '#' => PositionType::Rock,
            'S' => PositionType::Start,
            _ => Err(CannotParseElementFromChar::from(value))?,
        };

        Ok(Self {
            explore_range: Some(0).filter(|_| position_type == PositionType::Start),
            position_type,
        })
    }
}

impl TryFrom<char> for PositionType {
    type Error = CannotParseElementFromChar;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Rock),
            'S' => Ok(Self::Start),
            _ => Err(CannotParseElementFromChar::from(value)),
        }
    }
}

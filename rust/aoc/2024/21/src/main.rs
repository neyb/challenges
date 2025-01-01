use anyhow::*;
use challenges_common::graph::{grid, Direction};
use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{once, repeat_n};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::str::FromStr;

fn main() {
    let content = challenges_common::get_input_content(&["aoc", "2024", "21.txt"]);
    println!("part1: {:?}", part1::run(&content));
    println!("part2: {:?}", part2::run(&content));
}

mod part1;
mod part2;

type SharedCache<R> = Rc<RefCell<HashMap<(Coord, ButtonValue), R>>>;

#[derive(Clone)]
pub struct Pad {
    buttons: Vec<Button>,
    a_position: Coord,
    cache: SharedCache<Vec<Sequence>>,
}

impl Pad {
    pub fn num_pad() -> Self {
        Self {
            #[rustfmt::skip]
            buttons: vec![
                Button::num(7, 0, 0), Button::num(8, 1, 0), Button::num(9, 2, 0),
                Button::num(4, 0, 1), Button::num(5, 1, 1), Button::num(6, 2, 1),
                Button::num(1, 0, 2), Button::num(2, 1, 2), Button::num(3, 2, 2),
                                      Button::num(0, 1, 3), Button::a(     2, 3),
            ],
            a_position: Coord { x: 2, y: 3 },
            cache: Default::default(),
        }
    }

    pub fn dir_pad() -> Self {
        use challenges_common::graph::Direction::*;
        Self {
            #[rustfmt::skip]
            buttons: vec![               Button::dir(Up  , 1, 0), Button::a(         2, 0),
                Button::dir(Left, 0, 1), Button::dir(Down, 1, 1), Button::dir(Right, 2, 1),
            ],
            a_position: Coord { x: 2, y: 0 },
            cache: Default::default(),
        }
    }

    pub fn get_button(&self, coord: &Coord) -> Option<&Button> {
        self.buttons.iter().find(|button| &button.coord == coord)
    }

    pub fn get_button_by_value(&self, value: &ButtonValue) -> Option<&Button> {
        self.buttons.iter().find(|button| button.value == *value)
    }

    pub fn possible_move_to_tap(&self, from: &Coord, to: &ButtonValue) -> Vec<Sequence> {
        use challenges_common::graph::Direction::*;
        use std::cmp::Ordering::*;

        self.cache
            .borrow_mut()
            .entry((*from, to.clone()))
            .or_insert_with(|| {
                let to = self
                    .get_button_by_value(to)
                    .unwrap_or_else(|| panic!("{to:?} not found in pad: {:?}", self.buttons))
                    .coord;

                let vert_dir = match from.y.cmp(&to.y) {
                    Less => Some(Down),
                    Greater => Some(Up),
                    Equal => None,
                };

                let horiz_dir = match from.x.cmp(&to.x) {
                    Less => Some(Right),
                    Greater => Some(Left),
                    Equal => None,
                };

                let vertical = || {
                    let diff_y = from.y.abs_diff(to.y);
                    repeat_n(vert_dir.map(Action::Move), diff_y as usize).flatten()
                };

                let horizontal = || {
                    let diff_x = from.x.abs_diff(to.x);
                    repeat_n(horiz_dir.map(Action::Move), diff_x as usize).flatten()
                };

                let press = || once(Action::Press);

                match (vert_dir, horiz_dir) {
                    (Some(_), Some(_)) => {
                        let vertical_first =
                            once(vertical().chain(horizontal()).chain(press()).collect())
                                .take_while(|_| {
                                    self.get_button(&Coord { x: from.x, y: to.y }).is_some()
                                });

                        let horizontal_first =
                            once(horizontal().chain(vertical()).chain(press()).collect())
                                .take_while(|_| {
                                    self.get_button(&Coord { x: to.x, y: from.y }).is_some()
                                });

                        vertical_first.chain(horizontal_first).collect()
                    }
                    (Some(_), None) => {
                        vec![vertical().chain(press()).collect()]
                    }
                    (None, Some(_)) => {
                        vec![horizontal().chain(press()).collect()]
                    }
                    (None, None) => {
                        vec![Action::Press.into()]
                    }
                }
            })
            .clone()
    }
}

impl Hash for Pad {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.buttons.hash(state);
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Button {
    pub coord: Coord,
    pub value: ButtonValue,
}

impl Button {
    fn num(value: u8, x: u8, y: u8) -> Self {
        Self {
            coord: Coord { x, y },
            value: ButtonValue::Num(value),
        }
    }

    fn a(x: u8, y: u8) -> Self {
        Self {
            coord: Coord { x, y },
            value: ButtonValue::A,
        }
    }

    fn dir(direction: Direction, x: u8, y: u8) -> Self {
        Self {
            coord: Coord { x, y },
            value: ButtonValue::Direction(direction),
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum ButtonValue {
    Num(u8),
    Direction(Direction),
    A,
}

impl Display for ButtonValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonValue::Num(n) => write!(f, "{n}"),
            ButtonValue::Direction(dir) => write!(f, "{dir:?}"),
            ButtonValue::A => write!(f, "A"),
        }
    }
}

impl From<&Action> for ButtonValue {
    fn from(m: &Action) -> Self {
        match m {
            Action::Move(dir) => Self::Direction(*dir),
            Action::Press => Self::A,
        }
    }
}

impl From<&Sequence> for Vec<ButtonValue> {
    fn from(actions: &Sequence) -> Self {
        actions.iter().map(ButtonValue::from).collect()
    }
}

impl TryFrom<char> for ButtonValue {
    type Error = Error;

    fn try_from(char: char) -> Result<Self> {
        anyhow::Ok(match char {
            _ if char.is_ascii_digit() => Self::Num(char.to_digit(10).unwrap() as u8),
            'A' => Self::A,
            _ => bail!("Cannot parse ButtonValue from: {char}"),
        })
    }
}

type Coord = grid::Coord<u8>;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Code(Vec<ButtonValue>);

impl Code {
    fn num_part(&self) -> Result<usize> {
        anyhow::Ok(
            self.to_string()
                .chars()
                .filter(|c| c.is_ascii_digit())
                .join("")
                .parse()?,
        )
    }
}

impl Deref for Code {
    type Target = Vec<ButtonValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Code {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for value in &self.0 {
            write!(f, "{value}")?;
        }
        Result::Ok(())
    }
}

impl FromStr for Code {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let values = s.chars().map(ButtonValue::try_from).try_collect()?;
        anyhow::Ok(Self(values))
    }
}

impl FromIterator<ButtonValue> for Code {
    fn from_iter<I: IntoIterator<Item = ButtonValue>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Sequence(Vec<Action>);

impl Sequence {
    pub fn with_capacity(capacity: usize) -> Self {
        Sequence(Vec::with_capacity(capacity))
    }

    pub fn new() -> Self {
        Sequence(Vec::new())
    }

    pub fn extend_sequence(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Sequence {
    type Target = Vec<Action>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sequence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for action in self.iter() {
            write!(f, "{action}")?;
        }

        Result::Ok(())
    }
}

impl From<Action> for Sequence {
    fn from(value: Action) -> Self {
        let mut actions = Sequence::new();
        actions.push(value);
        actions
    }
}

impl FromIterator<Action> for Sequence {
    fn from_iter<I: IntoIterator<Item = Action>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Action {
    Move(Direction),
    Press,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Move(dir) => {
                let c = match dir {
                    Direction::Up => '^',
                    Direction::Down => 'v',
                    Direction::Left => '<',
                    Direction::Right => '>',
                };
                write!(f, "{c}")?;
            }
            Action::Press => {
                write!(f, "A")?;
            }
        };

        Result::Ok(())
    }
}

impl From<&ButtonValue> for Action {
    fn from(value: &ButtonValue) -> Self {
        match value {
            ButtonValue::Num(n) => panic!("no signal for value: {n}"),
            ButtonValue::Direction(dir) => Action::Move(*dir),
            ButtonValue::A => Action::Press,
        }
    }
}

impl TryFrom<char> for Action {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        anyhow::Ok(match value {
            'A' => Self::Press,
            '^' => Self::Move(Direction::Up),
            'v' => Self::Move(Direction::Down),
            '<' => Self::Move(Direction::Left),
            '>' => Self::Move(Direction::Right),
            _ => {
                bail!("Cannot parse Move from: {value}")
            }
        })
    }
}

#[derive(Clone)]
pub struct Robot {
    pad: Rc<Pad>,
    position: Coord,
    depth: usize,
    cache: SharedCache<usize>,
}

impl Robot {
    pub fn new(pad: Pad, depth: usize) -> Self {
        let position = pad.a_position;
        Self {
            pad: Rc::new(pad),
            position,
            depth,
            cache: Default::default(),
        }
    }

    pub fn shortest_sequence_len_to_tap(
        &mut self,
        targets: &[ButtonValue],
        system: &mut RobotsSystem,
    ) -> usize {
        if self.depth == 0 {
            targets.len()
        } else {
            targets
                .iter()
                .map(|target| self.shortest_sequence_len_to_tap_value(target, system))
                .sum()
        }
    }

    pub fn shortest_sequence_len_to_tap_value(
        &mut self,
        target: &ButtonValue,
        system: &mut RobotsSystem,
    ) -> usize {
        let result = *self
            .cache
            .borrow_mut()
            .entry((self.position, target.clone()))
            .or_insert_with(|| {
                self.possible_actions_to_tap_value(target)
                    .iter()
                    .map(|sequence| {
                        let mut robot = system.get_robot(self.depth - 1);
                        let values = sequence.iter().map(ButtonValue::from).collect_vec();
                        robot.shortest_sequence_len_to_tap(&values, system)
                    })
                    .min()
                    .unwrap()
            });
        self.move_to(target);
        result
    }

    pub fn possible_actions_to_tap_value(&self, target: &ButtonValue) -> Vec<Sequence> {
        self.pad.possible_move_to_tap(&self.position, target)
    }

    pub fn move_to(&mut self, button_value: &ButtonValue) {
        self.position = self.pad.get_button_by_value(button_value).unwrap().coord
    }
}

pub struct RobotsSystem {
    robots: Vec<Robot>,
}

impl RobotsSystem {
    pub fn new(nb_robots: usize) -> Result<Self> {
        // there is one robot who is you
        let robots: Vec<_> = repeat_n(Pad::dir_pad(), nb_robots)
            .chain(once(Pad::num_pad()))
            .enumerate()
            .map(|(depth, pad)| Robot::new(pad, depth))
            .collect();

        Ok(Self { robots })
    }

    pub fn get_robot(&self, depth: usize) -> Robot {
        self.robots[depth].clone()
    }

    fn find_a_shortest_sequence_len_for_code(&mut self, code: &Code) -> usize {
        let mut robot = self.get_robot(self.robots.len() - 1);
        robot.shortest_sequence_len_to_tap(code, self)
    }
}

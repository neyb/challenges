use anyhow::{bail, Error, Result};
use std::fmt::{Display, Formatter, Write};

fn main() {
    let snafus = parse(&["aoc", "2022", "25.txt"]);
    println!("part1: {}", part1(&snafus))
}

fn parse(path: &[&str]) -> Vec<SNAFUNumber> {
    challenges_common::get_input_lines(path)
        .map(|line| SNAFUNumber::from_string(&line).unwrap())
        .collect()
}

fn part1(snafus: &[SNAFUNumber]) -> String {
    let sum = snafus.iter().map(|snafu| snafu.value()).sum();
    SNAFUNumber::from_value(sum).to_string()
}

type Number = u64;
type SignedNumber = i64;

struct SNAFUNumber {
    digits: Vec<SNAFUDigit>,
}

impl SNAFUNumber {
    fn from_string(s: &str) -> Result<Self> {
        let digits = s
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { digits })
    }

    fn from_value(value: Number) -> Self {
        let mut value = value;
        let mut digits = Vec::new();

        while value != 0 {
            use SNAFUDigit::*;

            let digit = match value % 5 {
                0 => Zero,
                1 => {
                    value -= 1;
                    One
                }
                2 => {
                    value -= 2;
                    Two
                }
                3 => {
                    value += 2;
                    MinusTwo
                }
                4 => {
                    value += 1;
                    MinusOne
                }
                _ => panic!("mod 5 should be in 0..=4"),
            };

            digits.insert(0, digit);

            value /= 5;
        }

        Self { digits }
    }

    fn value(&self) -> Number {
        self.digits.iter().fold(0 as Number, |sum, d| {
            (sum as SignedNumber * 5 + d.value() as SignedNumber) as Number
        })
    }
}

impl Display for SNAFUNumber {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for digit in &self.digits {
            f.write_char(digit.char())?
        }
        Ok(())
    }
}

enum SNAFUDigit {
    Zero,
    One,
    Two,
    MinusTwo,
    MinusOne,
}

impl SNAFUDigit {
    fn char(&self) -> char {
        use SNAFUDigit::*;
        match self {
            Zero => '0',
            One => '1',
            Two => '2',
            MinusTwo => '=',
            MinusOne => '-',
        }
    }

    fn value(&self) -> i8 {
        use SNAFUDigit::*;
        match self {
            Zero => 0,
            One => 1,
            Two => 2,
            MinusTwo => -2,
            MinusOne => -1,
        }
    }
}

impl TryFrom<char> for SNAFUDigit {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        use SNAFUDigit::*;
        Ok(match c {
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '=' => MinusTwo,
            '-' => MinusOne,
            _ => bail!("{} is not a SNAFU digit", c),
        })
    }
}

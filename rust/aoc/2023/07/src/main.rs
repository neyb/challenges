mod part1;
mod part2;

use anyhow::Result;
use itertools::Itertools;
use std::fmt::Debug;
use std::str::FromStr;

fn main() {
    let input = challenges_common::get_input_content(&["aoc", "2023", "07.txt"]);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> Bid {
    let mut game: Game<part1::Hand> = input.parse().unwrap();
    game.all_score()
}

fn part2(input: &str) -> Bid {
    let mut game: Game<part2::Hand> = input.parse().unwrap();
    game.all_score()
}

type Bid = u32;

struct Game<H: Hand> {
    plays: Vec<Play<H>>,
}

impl<H: Hand> Game<H> {
    fn all_score(&mut self) -> Bid {
        self.plays.sort_by(|a, b| a.hand.cmp(&b.hand));

        self.plays
            .iter()
            .enumerate()
            .map(|(index, play)| play.bid * (index as Bid + 1))
            .sum()
    }
}

impl<H: Hand> FromStr for Game<H> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let plays = s.lines().map(|line| line.parse()).try_collect()?;
        Ok(Game { plays })
    }
}

#[derive(Debug)]
struct Play<H: Hand> {
    hand: H,
    bid: Bid,
}

impl<H: Hand> FromStr for Play<H> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let hand = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("no hand line ???"))?;
        let hand = parse_hand(hand)?;
        let bid = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("no bid line ???"))?
            .parse()?;
        Ok(Play { hand, bid })
    }
}

fn parse_hand<H: Hand>(s: &str) -> Result<H> {
    let cards: Vec<Card> = s.chars().map(|c| c.try_into()).try_collect()?;
    Ok(cards.into())
}

trait Hand: Ord + From<Vec<Card>> {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        use Card::*;
        Ok(match char {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Jack,
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            _ => anyhow::bail!("cannot parse card value from {}", char),
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_part1() {
        let input = challenges_common::get_input_content(&["aoc", "2023", "07-test.txt"]);
        assert_eq!(part1(&input), 6440);
    }

    #[test]
    fn given_test_part2() {
        let input = challenges_common::get_input_content(&["aoc", "2023", "07-test.txt"]);
        assert_eq!(part2(&input), 5905);
    }
}

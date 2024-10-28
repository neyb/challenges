use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = challenges_common::get_input_content(&["aoc", "2023", "07.txt"]);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> Bid {
    let mut game: Game = input.parse().unwrap();
    game.all_score()
}

fn part2(input: &str) -> Bid {
    let mut game: Game = input.parse().unwrap();
    game.update_hand_type_part2();
    game.all_score_part2()
}

type Bid = u32;

struct Game {
    plays: Vec<Play>,
}

impl Game {
    fn all_score(&mut self) -> Bid {
        self.plays.sort_by(|a, b| a.hand.cmp(&b.hand));

        self.plays
            .iter()
            .enumerate()
            .map(|(index, play)| play.bid * (index as Bid + 1))
            .sum()
    }

    fn all_score_part2(&mut self) -> Bid {
        self.plays
            .sort_by(|a, b| Part2Hand(&a.hand).cmp(&Part2Hand(&b.hand)));

        self.plays
            .iter()
            .enumerate()
            .map(|(index, play)| play.bid * (index as Bid + 1))
            .sum()
    }

    fn update_hand_type_part2(&mut self) {
        self.plays
            .iter_mut()
            .for_each(|play| play.update_hand_type_part2());
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let plays = s.lines().map(|line| line.parse()).try_collect()?;
        Ok(Game { plays })
    }
}

#[derive(Debug)]
struct Play {
    hand: Hand,
    bid: Bid,
}

impl Play {
    fn update_hand_type_part2(&mut self) {
        self.hand.hand_type = HandType::from_hand_part2(&self.hand.cards);
    }
}

impl FromStr for Play {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let hand = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("no hand line ???"))?
            .parse()?;
        let bid = split
            .next()
            .ok_or_else(|| anyhow::anyhow!("no bid line ???"))?
            .parse()?;
        Ok(Play { hand, bid })
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<Card> = s.chars().map(|c| c.try_into()).try_collect()?;
        let hand_type = HandType::from_hand(&cards);
        Ok(Hand {
            cards,
            hand_type: hand_type,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.hand_type, &self.cards).cmp(&(&other.hand_type, &other.cards))
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Part2Hand<'h>(&'h Hand);

impl <'h> PartialOrd for Part2Hand<'h> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'h> Ord for Part2Hand<'h> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_pair = (
            &self.0.hand_type,
            self.0
                .cards
                .iter()
                .map(|card| CardPart2(card))
                .collect_vec(),
        );
        let other_pair = (
            &other.0.hand_type,
            other
                .0
                .cards
                .iter()
                .map(|card| CardPart2(card))
                .collect_vec(),
        );
        self_pair.cmp(&other_pair)
    }
}

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

#[derive(PartialEq, Eq, Debug)]
struct CardPart2<'c>(&'c Card);

impl<'c> PartialOrd for CardPart2<'c> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'c> Ord for CardPart2<'c> {
    fn cmp(&self, other: &Self) -> Ordering {
        use Card::Jack;
        match (self.0, other.0) {
            (Jack, Jack) => Ordering::Equal,
            (Jack, _) => Ordering::Less,
            (_, Jack) => Ordering::Greater,
            _ => self.0.cmp(other.0),
        }
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

impl HandType {
    fn from_hand(cards: &[Card]) -> Self {
        let groups = cards.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        });
        let mut groups = groups.iter().collect_vec();

        groups.sort_by(|a, b| b.1.cmp(a.1));

        match (groups.first(), groups.get(1)) {
            (Some((_, 5)), _) => HandType::FiveOfAKind,
            (Some((_, 4)), Some((_, 1))) => HandType::FourOfAKind,
            (Some((_, 3)), Some((_, 2))) => HandType::FullHouse,
            (Some((_, 3)), _) => HandType::ThreeOfAKind,
            (Some((_, 2)), Some((_, 2))) => HandType::TwoPair,
            (Some((_, 2)), _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn from_hand_part2(cards: &[Card]) -> Self {
        let (joker_count, groups) = cards.iter().fold(
            (0, HashMap::new()),
            |(mut joker_count, mut groups), card| {
                if card == &Card::Jack {
                    joker_count += 1;
                } else {
                    *groups.entry(card).or_insert(0) += 1;
                }
                (joker_count, groups)
            },
        );

        let mut groups = groups.iter().map(|(_, &count)| count).collect_vec();
        groups.sort_by(|a, b| b.cmp(a));

        match (
            groups.first().map_or(5, |&count| count + joker_count),
            groups.get(1),
        ) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, Some(2)) => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, Some(2)) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
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

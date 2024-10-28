use crate::{Card,  HandType};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    cards: Vec<CardValuePart2>,
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.hand_type, &self.cards).cmp(&(&other.hand_type, &other.cards))
    }
}

impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Self {
        let hand_type = {
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

            let mut groups = groups.into_values().collect_vec();
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
        };

        let cards = cards.into_iter().map(CardValuePart2).collect();
        Self { cards, hand_type }
    }
}

impl super::Hand for Hand {}

#[derive(PartialEq, Eq, Debug)]
struct CardValuePart2(Card);
impl Ord for CardValuePart2 {
    fn cmp(&self, other: &Self) -> Ordering {
        use super::Card::Jack;
        match (&self.0, &other.0) {
            (Jack, Jack) => Ordering::Equal,
            (Jack, _) => Ordering::Less,
            (_, Jack) => Ordering::Greater,
            _ => self.0.cmp(&other.0),
        }
    }
}
impl PartialOrd for CardValuePart2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

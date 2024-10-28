use crate::{Card, HandType};
use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    cards: Vec<CardValuePart1>,
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
        let groups = cards.iter().fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card).or_insert(0) += 1;
            acc
        });

        let mut groups = groups.into_values().collect_vec();
        groups.sort_by_key(|&count| Reverse(count));

        let hand_type = match (groups.first(), groups.get(1)) {
            (Some(5), _) => HandType::FiveOfAKind,
            (Some(4), Some(1)) => HandType::FourOfAKind,
            (Some(3), Some(2)) => HandType::FullHouse,
            (Some(3), _) => HandType::ThreeOfAKind,
            (Some(2), Some(2)) => HandType::TwoPair,
            (Some(2), _) => HandType::OnePair,
            _ => HandType::HighCard,
        };

        let cards = cards.into_iter().map(CardValuePart1).collect();
        Self { cards, hand_type }
    }
}

impl super::Hand for Hand {}

#[derive(PartialEq, Eq, Debug)]
struct CardValuePart1(Card);
impl Ord for CardValuePart1 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for CardValuePart1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

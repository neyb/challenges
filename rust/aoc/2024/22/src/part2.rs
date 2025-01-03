use crate::{parse, Secret};
use anyhow::*;
use std::collections::HashMap;

type Res = u64;

type Price = i8;
type Change = Price;
type Seq<'p> = &'p [i8];

pub(crate) fn run(content: &str) -> Result<Res> {
    let secrets = parse(content)?;

    // this is for owning the (continuous) changes sequences to reduce allocation
    let secrets_changes_and_prices: Vec<(Vec<Change>, Vec<Price>)> = secrets
        .into_iter()
        .map(|secret| secret.into_get_changes_and_prices())
        .collect();

    let secrets_prices_by_sequences: Vec<HashMap<Seq, Price>> = secrets_changes_and_prices
        .iter()
        .map(|(changes, prices)| get_price_by_seq(changes, prices))
        .collect();

    let merged_secrets_prices_by_sequences =
        merge_secrets_prices_by_sequences(secrets_prices_by_sequences);

    Ok(merged_secrets_prices_by_sequences
        .into_values()
        .map(|price| price as Res)
        .max()
        .unwrap())
}

fn get_price_by_seq<'p>(changes: &'p [Change], prices: &'p [Price]) -> HashMap<Seq<'p>, Price> {
    let mut result = HashMap::with_capacity(2000);
    for (seq, price) in changes.windows(4).zip(prices.iter().skip(3).copied()) {
        result.entry(seq).or_insert(price);
    }
    result
}

fn merge_secrets_prices_by_sequences(
    all_prices_by_sequences: Vec<HashMap<Seq, Price>>,
) -> HashMap<Seq, Price> {
    let mut result: HashMap<Seq, Price> = HashMap::new();
    for prices_by_seq in all_prices_by_sequences {
        for (seq, price) in prices_by_seq {
            *result.entry(seq).or_insert(0) += price;
        }
    }
    result
}

trait GetPrice {
    fn get_price(&self) -> Price;
    fn into_get_changes_and_prices(self) -> (Vec<Change>, Vec<Price>);
}

impl GetPrice for Secret {
    fn get_price(&self) -> Price {
        (self.0 .0 % 10) as Price
    }

    fn into_get_changes_and_prices(mut self) -> (Vec<Change>, Vec<Price>) {
        let mut price = self.get_price();

        (0..2000)
            .map(|_| {
                let old_price = price;
                self.next();
                price = self.get_price();
                (price - old_price, price)
            })
            .unzip()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "22-test2.txt"]);
        assert_eq!(run(&content).unwrap(), 23);
    }
}

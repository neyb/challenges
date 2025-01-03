use crate::{parse, Secret};
use anyhow::*;

type Res = u64;
pub(crate) fn run(content: &str) -> Result<Res> {
    let mut secrets = parse(content)?;

    Ok(secrets
        .iter_mut()
        .map(|secret| {
            for _ in 0..2000 {
                secret.next();
            }
            i32::from(secret as &Secret) as Res
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_of_123() {
        let mut s = Secret::new(123);
        s.next();
        assert_eq!(i32::from(&s), 15887950);
    }

    #[test]
    fn test_run() {
        let content = challenges_common::get_input_content(&["aoc", "2024", "22-test.txt"]);
        assert_eq!(run(&content).unwrap(), 37327623);
    }
}

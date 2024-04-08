extern crate core;

use anyhow::*;

use challenges_common::get_input_content;

use crate::part_1::part_1;

fn main() -> Result<()> {
    let content = get_input_content(&["aoc", "2023", "01.txt"]);

    println!("part 1 : {}", part_1(&content)?);

    print!("part 2 : {}", part_2::run(&content)?);

    Ok(())
}

mod part_1 {
    use anyhow::anyhow;

    pub fn part_1(content: impl AsRef<str>) -> anyhow::Result<u32> {
        content.as_ref().lines().map(to_number_part1).sum()
    }

    fn to_number_part1(line: &str) -> anyhow::Result<u32> {
        let mut line_digits = line.chars().flat_map(|char| char.to_digit(10));
        let first = line_digits
            .next()
            .ok_or_else(|| anyhow!("no first in line {}", line))?;
        let last = line_digits.last().unwrap_or(first);
        anyhow::Ok((first.to_string() + &last.to_string()).parse()?)
    }
}

mod part_2 {
    use anyhow::Context;

    pub fn run(content: impl AsRef<str>) -> anyhow::Result<u32> {
        content.as_ref().lines().map(to_number_part2).sum()
    }

    fn to_number_part2(line: &str) -> anyhow::Result<u32> {
        let mut matches = lazy_regex::regex!(r"one|two|three|four|five|six|seven|eight|nine|\d")
            .captures_iter(line);

        let (first, []) = matches
            .next()
            .context(format!("no match for {}", line))?
            .extract();

        let last = match matches.last() {
            Some(last) => last.extract::<0>().0,
            None => first,
        };

        anyhow::Ok((part_2_to_digit(first)? + &part_2_to_digit(last)?).parse()?)
    }

    fn part_2_to_digit(s: &str) -> anyhow::Result<String> {
        anyhow::Ok(match s {
            "one" => "1".to_string(),
            "two" => "2".to_string(),
            "three" => "3".to_string(),
            "four" => "4".to_string(),
            "five" => "5".to_string(),
            "six" => "6".to_string(),
            "seven" => "7".to_string(),
            "eight" => "8".to_string(),
            "nine" => "9".to_string(),
            s => s.parse::<u32>()?.to_string(),
        })
    }

    #[cfg(test)]
    mod tests {
        use crate::part_2::run;

        #[test]
        fn given_test() {
            let result = run(r"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
                .trim());

            assert_eq!(result.unwrap(), 281)
        }

        #[test]
        fn two_to_digit() {
            assert_eq!(super::part_2_to_digit("two").unwrap(), "2")
        }

        #[test]
        fn two1nine_to_num() {
            assert_eq!(super::to_number_part2("two1nine").unwrap(), 29)
        }
    }
}

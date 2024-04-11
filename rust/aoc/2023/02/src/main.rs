use challenges_common::get_input_content;

fn main() -> anyhow::Result<()> {
    let content = get_input_content(&["aoc", "2023", "02.txt"]);

    println!("part 1: {}", part_1::run(&content)?);
    println!("part 1: {}", part_2::run(&content)?);

    Ok(())
}

mod game {
    use std::str::FromStr;

    use anyhow::anyhow;
    use itertools::Itertools;

    pub struct Game {
        pub id: u32,
        draws: Vec<Draw>,
    }

    impl Game {
        pub fn complies(&self) -> bool {
            self.draws.iter().all(|draw| draw.complies())
        }

        pub fn power(&self) -> u32 {
            let min_set = self.min_set();
            min_set.nb_red * min_set.nb_green * min_set.nb_blue
        }

        fn min_set(&self) -> MinSet {
            self.draws.iter().fold(MinSet::empty(), |mut acc, draw| {
                if acc.nb_red < draw.nb_red {
                    acc.nb_red = draw.nb_red
                }
                if acc.nb_green < draw.nb_green {
                    acc.nb_green = draw.nb_green
                }
                if acc.nb_blue < draw.nb_blue {
                    acc.nb_blue = draw.nb_blue
                }
                acc
            })
        }
    }

    impl FromStr for Game {
        type Err = anyhow::Error;

        fn from_str(line: &str) -> anyhow::Result<Self> {
            let (game_str, draws_str) = line
                .split(':')
                .collect_tuple()
                .ok_or_else(|| anyhow!("first split ko on {}", line))?;

            let game_id_str = lazy_regex::regex_captures!(r"\d+", game_str);
            let game_id: u32 = game_id_str
                .ok_or_else(|| anyhow!("game id not found in {}", game_str))?
                .parse()?;

            let draws = draws_str
                .split(';')
                .map(|draw_str| draw_str.parse::<Draw>())
                .try_collect()?;

            Ok(Game { id: game_id, draws })
        }
    }

    struct MinSet {
        pub nb_red: u32,
        pub nb_green: u32,
        pub nb_blue: u32,
    }

    impl MinSet {
        fn empty() -> MinSet {
            MinSet {
                nb_green: 0,
                nb_red: 0,
                nb_blue: 0,
            }
        }
    }

    pub struct Draw {
        pub nb_red: u32,
        pub nb_green: u32,
        pub nb_blue: u32,
    }

    impl Draw {
        fn complies(&self) -> bool {
            self.nb_red <= 12 && self.nb_green <= 13 && self.nb_blue <= 14
        }
    }

    impl Draw {
        pub fn empty() -> Self {
            Self {
                nb_red: 0,
                nb_green: 0,
                nb_blue: 0,
            }
        }
    }

    impl FromStr for Draw {
        type Err = anyhow::Error;

        fn from_str(draw_str: &str) -> anyhow::Result<Self> {
            let mut draw = Draw::empty();
            let regex = lazy_regex::lazy_regex!(r"(\d+) (blue|red|green)");
            for (_, [draw_color_count_str, color_str]) in
                regex.captures_iter(draw_str).map(|capt| capt.extract())
            {
                let draw_color_count = draw_color_count_str.parse()?;
                match color_str {
                    "blue" => draw.nb_blue = draw_color_count,
                    "red" => draw.nb_red = draw_color_count,
                    "green" => draw.nb_green = draw_color_count,
                    _ => anyhow::bail!("not a supported color: {}", color_str),
                }
            }

            Ok(draw)
        }
    }
}

mod part_2 {
    use anyhow::Result;

    use crate::game::Game;

    pub fn run(content: &impl AsRef<str>) -> Result<u32> {
        content
            .as_ref()
            .lines()
            .map(|line| -> Result<Game> { line.parse() })
            .try_fold(0, |acc, game| game.map(|game| acc + game.power()))
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn given_test() {
            let input = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .trim();

            assert_eq!(super::run(&input).unwrap(), 2286);
        }
    }
}

mod part_1 {
    use anyhow::Result;

    use crate::game::Game;

    pub fn run(content: &impl AsRef<str>) -> Result<u32> {
        content
            .as_ref()
            .lines()
            .map(|line| -> Result<Game> { line.parse() })
            .filter(|try_game| match try_game {
                Ok(game) => game.complies(),
                Err(_) => true,
            })
            .try_fold(0, |acc, game| game.map(|game| acc + game.id))
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn given_test() {
            let input = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .trim();

            assert_eq!(super::run(&input).unwrap(), 8);
        }
    }
}

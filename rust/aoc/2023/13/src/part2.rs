use crate::{Pattern, Patterns};
use challenges_common::graph::Coord;

pub(crate) fn run(content: &str) -> anyhow::Result<usize> {
    let patterns: Patterns = content.parse()?;
    Ok(patterns.summarize_smudge())
}

trait Smudged {
    fn summarize_smudge(&self) -> usize;
}

impl Smudged for Patterns {
    fn summarize_smudge(&self) -> usize {
        self.patterns
            .iter()
            .map(|pattern| pattern.summarize_smudge())
            .sum()
    }
}

impl Smudged for Pattern {
    fn summarize_smudge(&self) -> usize {
        return get_x_symmetry(self)
            .or(get_y_symmetry(self).map(|y_symm| y_symm * 100))
            .unwrap_or(0);

        fn get_x_symmetry(pattern: &Pattern) -> Option<usize> {
            'x_symm: for x_symm in 1..pattern.grid.width() {
                let mut smudged = false;
                let max_x_diff = x_symm.min(pattern.grid.width() - x_symm);
                for x_diff in 1..=max_x_diff {
                    let x_left = x_symm - x_diff;
                    let x_right = x_symm + x_diff - 1;
                    for y in 0..pattern.grid.height() {
                        if pattern.grid.at(&Coord { x: x_left, y }).unwrap().0
                            != pattern.grid.at(&Coord { x: x_right, y }).unwrap().0
                        {
                            if smudged {
                                continue 'x_symm;
                            }
                            smudged = true;
                        }
                    }
                }

                if smudged {
                    return Some(x_symm);
                }
            }
            None
        }

        fn get_y_symmetry(pattern: &Pattern) -> Option<usize> {
            'y_symm: for y_symm in 1..pattern.grid.height() {
                let mut smudged = false;
                for y_diff in 1..y_symm.min(pattern.grid.height() - y_symm) + 1 {
                    let y_left = y_symm - y_diff;
                    let y_right = y_symm + y_diff - 1;
                    for x in 0..pattern.grid.width() {
                        if pattern
                            .grid
                            .at(&Coord { x, y: y_left })
                            .map(|place| place.0)
                            != pattern
                                .grid
                                .at(&Coord { x, y: y_right })
                                .map(|place| place.0)
                        {
                            if smudged {
                                continue 'y_symm;
                            }
                            smudged = true;
                        }
                    }
                }
                if smudged {
                    return Some(y_symm);
                }
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn given_test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "13-test.txt"]);
        assert_eq!(super::run(&content).unwrap(), 400)
    }
}

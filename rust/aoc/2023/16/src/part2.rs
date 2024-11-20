use crate::{BeamHead, Contraption};
use challenges_common::graph::{Coord, Direction};

pub(crate) fn run(content: &str) -> anyhow::Result<usize> {
    let contraption: Contraption = content.parse()?;
    let beam_heads = all_possible_beam_heads(&contraption);
    Ok(beam_heads
        .iter()
        .map(|beam_head| contraption.count_energized(beam_head.clone()))
        .max()
        .unwrap())
}

fn all_possible_beam_heads(contraption: &Contraption) -> Vec<BeamHead> {
    use Direction::*;

    let grid = &contraption.grid;

    (0..grid.height())
        .flat_map(|y| {
            [
                BeamHead {
                    coord: Coord { x: 0, y },
                    direction: Right,
                },
                BeamHead {
                    coord: Coord {
                        x: grid.width() - 1,
                        y,
                    },
                    direction: Left,
                },
            ]
        })
        .chain((0..grid.width()).flat_map(|x| {
            [
                BeamHead {
                    coord: Coord { x, y: 0 },
                    direction: Down,
                },
                BeamHead {
                    coord: Coord {
                        x,
                        y: grid.height() - 1,
                    },
                    direction: Up,
                },
            ]
        }))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "16-test.txt"]);
        assert_eq!(run(&content).unwrap(), 51);
    }
}

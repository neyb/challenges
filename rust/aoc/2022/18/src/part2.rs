use std::collections::HashSet;

use challenges_common::graph::{astar, Step};

use crate::Cube;

pub(crate) fn count_exterior_sized(cubes_vec: &Vec<Cube>) -> usize {
    let mut cubes = HashSet::new();
    for cube in cubes_vec {
        cubes.insert(cube);
    }

    cubes_vec
        .iter()
        .flat_map(|&Cube { x, y, z }| {
            vec![
                Cube { x: x + 1, y, z },
                Cube { x: x - 1, y, z },
                Cube { x, y: y + 1, z },
                Cube { x, y: y - 1, z },
                Cube { x, y, z: z + 1 },
                Cube { x, y, z: z - 1 },
            ]
        })
        .filter(|neighbour_cube| !cubes.contains(neighbour_cube))
        .filter(|neighbour_cube| can_go_to_origin(neighbour_cube, &cubes))
        .count()
}

fn can_go_to_origin(neighbour_cube: &Cube, cubes: &HashSet<&Cube>) -> bool {
    let path_to_origin = astar(
        neighbour_cube.clone(),
        |&Cube { x, y, z }| {
            vec![
                Cube { x: x + 1, y, z },
                Cube { x: x - 1, y, z },
                Cube { x, y: y + 1, z },
                Cube { x, y: y - 1, z },
                Cube { x, y, z: z + 1 },
                Cube { x, y, z: z - 1 },
            ]
            .into_iter()
            .filter(|cube| !cubes.contains(cube))
            .map(|cube| Step {
                to: cube,
                additional_cost: 1,
            })
        },
        |&Cube { x, y, z }| x == 0 && y == 0 && z == 0,
        |Cube { x, y, z }| (x.abs() + y.abs() + z.abs()) as u32,
    );
    path_to_origin.is_some()
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use crate::*;

    use super::*;

    #[test]
    fn given_test() {
        let cubes = parse_cubes(&["aoc", "2022", "18-test.txt"]).unwrap();
        assert_eq!(count_exterior_sized(&cubes), 58)
    }
}

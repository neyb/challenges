use crate::Cube;
use std::collections::HashSet;

pub(crate) fn count_sized(cubes_vec: &Vec<Cube>) -> usize {
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
        .count()
}

#[cfg(test)]
mod test {
    
    use crate::parse_cubes;
    use crate::part1::count_sized;

    #[test]
    fn given_test() {
        let cubes = parse_cubes(&["aoc", "2022", "18-test.txt"]).unwrap();
        assert_eq!(count_sized(&cubes), 64)
    }
}

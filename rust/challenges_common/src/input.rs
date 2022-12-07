use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn get_input_file(location: Vec<&str>) -> File {
    let current = Path::new(".").canonicalize().unwrap();
    let target = |path: &Path| {
        location
            .iter()
            .fold(path.join("inputs"), |path, file| path.join(file))
    };

    let target = current
        .ancestors()
        .map(target)
        .find(|target| target.exists())
        .unwrap();

    File::open(target).unwrap()
}

pub fn get_input_lines(location: Vec<&str>) -> impl Iterator<Item = String> {
    let file = get_input_file(location);
    BufReader::new(file)
        .lines()
        .map(|line_res| line_res.unwrap())
}

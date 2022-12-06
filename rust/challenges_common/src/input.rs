use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn get_input_file(location: Vec<&str>) -> Option<File> {
    let current = Path::new(".").canonicalize().unwrap();
    let target = |path: &Path| {
        location
            .iter()
            .fold(path.join("inputs"), |path, file| path.join(file))
    };

    let target = current
        .ancestors()
        .map(target)
        .find(|target| target.exists());

    target.map(|target| File::open(target).unwrap())
}

pub fn get_input_lines(location: Vec<&str>) -> Option<Vec<String>> {
    get_input_file(location).map(|file| {
        BufReader::new(file)
            .lines()
            .map(|line_res| line_res.unwrap())
            .collect::<Vec<_>>()
    })
}

use std::io::Read;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn get_input_file(location: &[impl AsRef<Path>]) -> File {
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

pub fn get_input_lines(location: &[impl AsRef<Path>]) -> impl Iterator<Item = String> {
    let file = get_input_file(location);
    BufReader::new(file)
        .lines()
        .map(|line_res| line_res.unwrap())
}

pub fn get_input_content(location: &[impl AsRef<Path>]) -> std::io::Result<String> {
    let mut file = get_input_file(location);
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

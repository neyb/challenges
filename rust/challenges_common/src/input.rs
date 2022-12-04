use std::{fs::File, path::Path};

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

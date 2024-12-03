use anyhow::*;

type Res = usize;
pub(crate) fn run(content: &str) -> Result<Res> {
    let regex = lazy_regex::regex!(r"mul\((\d+),(\d+)\)");

    Ok(regex
        .captures_iter(content)
        .map(|cap| {
            let a = cap[1].parse::<usize>().unwrap();
            let b = cap[2].parse::<usize>().unwrap();
            a * b
        })
        .sum::<usize>())
}

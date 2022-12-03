extern crate itertools;

use itertools::{Itertools, fold};

fn main() {
    let content = std::fs::read_to_string("input.txt").expect("should read the file");

    let splitted = content
        .split("\n")
        .map(|s| u32::from_str_radix(s, 10))
        .map(|x| x.ok())
        .fold(Vec::<u32>::new(), |mut acc, val| {
            if let Some(val) = val {
                acc.las
            } else {

            }
            
            acc;
        })

    let x = splitted.coalesce(|x, y| match (x, y) {
        (Some(x), Some(y)) => Ok(Some(x + y)),
        _ => Err((x, y)),
    });
    for x in splitted {
        if let Some(value) = x {
            println!("{}", value)
        } else {
            println!("changement")
        }
    }
}

use std::collections::{HashSet};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::str::Chars;


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn find_unique(str: &String) -> usize{
    // example string  = mjqjpqmgbljsphdzt
    // We need to find the first time where we have 4 unique characters in a row
    // then we return the character index + 4

    for (i, c) in str.chars().enumerate() {
        let mut chars_set: HashSet<String> = HashSet::new();
        let c1 = str.get(i..(i + 4)).unwrap();

        for c in c1.chars() {
            chars_set.insert((c as char).to_string());
        }

        if chars_set.len() == 4 {
            return i + 4
        }
    }

    panic!("Shouldn't be here - no 4 unique chars found!")
}


fn main() {
    let score: usize = lines_from_file("./input.txt")
        .iter()
        .map(|a| find_unique(a))
        .sum();

    println!("{}", score)
}

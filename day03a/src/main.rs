use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use std::collections::{HashMap, HashSet};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn find_duplicated_letter(val: &String) -> String {
    // pass in a string and find the duplicated letter

    // Create a hash set
    // if letter in hash set, we return the letter
    // if not, then add the letter to the hashset

    // First split the string in half
    // Find the length of the string

    let mut letters: HashSet<String> = HashSet::new();

    let val_length: usize = val.len();
    let val_half_length: usize = val_length / 2;

    // We make the val owned and then grab a new reference to it
    let copied_val = val.to_owned();
    let first_half_slice = &copied_val[0..val_half_length];

    for letter in first_half_slice.chars() {
        letters.insert(letter.to_string());
    }

    let second_half_slice = &copied_val[val_half_length..];

    for letter in second_half_slice.chars() {
        if letters.contains(&letter.to_string()) {
            return letter.to_string();
        }
    }
    panic!("This shouldn't happen");
}

fn create_scorecard() -> HashMap<String, i32> {
    // Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.
    let mut scorecard: HashMap<String, i32> = HashMap::new();
    for (index, letter) in (b'a'..=b'z').enumerate() {
        // index starts at 0 but we need to start it from 1
        // Capital letters start with score 26 ( i.e 1 + 26 = 27)
        scorecard.insert((letter as char).to_string(), (index as i32) + 1);
        scorecard.insert(
            (letter as char).to_string().to_uppercase(),
            (index as i32) + 27,
        );
    }

    return scorecard;
}

fn calculate_score_for_letter(letter: &String, scorecard: &HashMap<String, i32>) -> i32 {
    let val = scorecard.get(letter).unwrap();
    return *val;
}

fn main() {
    println!("Starting...");
    let scorecard: HashMap<String, i32> = create_scorecard();

    let score: i32 = lines_from_file("./input.txt")
        .iter()
        .map(|a| find_duplicated_letter(a))
        .map(|a| calculate_score_for_letter(&a, &scorecard))
        .sum();

    println!("{}", score)
}

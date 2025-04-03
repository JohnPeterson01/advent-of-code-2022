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

fn create_groups<T>(arr: Vec<T>, arr_length: usize) -> Vec<Vec<T>> {
    /*
    Create a list of groups with a given length per group
     */
    let mut current_arr: Vec<T> = Vec::new();
    let mut arrays: Vec<Vec<T>> = Vec::new();
    for (index, item) in arr.into_iter().enumerate() {
        current_arr.push(item);

        if index != 0 && (index + 1) % arr_length == 0 {
            arrays.push(current_arr);
            current_arr = Vec::new();
        }
    }
    return arrays;
}

fn find_duplicated_letter_in_group(group: &Vec<String>) -> String {
    // group is an array of items e.g 3 lines from the input
    let mut letters_map = HashMap::new();
    for item in group.iter() {
        // first we create a hash set
        let mut letters = HashSet::new();

        // then we iterate over each character in the item
        // and add to the set
        for c in item.chars() {
            letters.insert(c.to_string());
        }

        // Then we iterate over the set and add each to a hashmap
        for letter in letters.into_iter() {
            if letters_map.contains_key(&letter) {
                let num = letters_map.get(&letter).unwrap();
                letters_map.insert(letter, num + 1);
            } else {
                letters_map.insert(letter, 1);
            };
        }
    }
    let group_length: usize = group.len();

    // We're looking for the one with a letter in each group i.e same as the length of the group
    for (key, value) in letters_map {
        if value == group_length {
            return key.to_string();
        }
    }
    panic!("This shouldn't occur")
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

pub fn main() {
    println!("Starting...");
    let scorecard: HashMap<String, i32> = create_scorecard();

    let lines: Vec<String> = lines_from_file("./example.txt")
        .iter()
        .map(|a| a.to_owned())
        .collect::<Vec<String>>();
    let groups = create_groups(lines, 3);

    let score: i32 = groups
        .iter()
        .map(|group| find_duplicated_letter_in_group(group))
        .map(|letter| calculate_score_for_letter(&letter, &scorecard))
        .sum();

    println!("Score: {}", score);
}

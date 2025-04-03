use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use std::collections::HashSet;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn create_cleaning_sections(line: &String) -> (HashSet<i32>, HashSet<i32>) {
    // Takes in a line of data
    // split to get to sections
    let sections = line
        .split(",")
        .map(|a| a.to_string())
        .collect::<Vec<String>>();
    let mut cleaning_sections: Vec<HashSet<i32>> = Vec::new();
    for section in sections {
        let mut cleaning_section: HashSet<i32> = HashSet::new();
        let section_start_and_end: Vec<String> =
            section.split("-").map(|a| a.to_string()).collect();
        let start = &section_start_and_end[0].parse::<i32>().unwrap();
        let end = &section_start_and_end[1].parse::<i32>().unwrap();

        for n in *start..(*end + 1) {
            // Add data to each section
            cleaning_section.insert(n);
        }

        cleaning_sections.push(cleaning_section);
    }

    let len_of_sections = cleaning_sections.len();
    if len_of_sections == 2 {
        return (
            cleaning_sections[0].to_owned(),
            cleaning_sections[1].to_owned(),
        );
    }

    panic!("Error found - we should have 2 sections in total")
}

fn are_sections_intersected(s1: HashSet<i32>, s2: HashSet<i32>) -> bool {
    return s1.intersection(&s2).count() > 0;
}

fn main() {
    let score: usize = lines_from_file("./input.txt")
        .iter()
        .map(|line| create_cleaning_sections(line))
        .map(|(s1, s2)| are_sections_intersected(s1, s2))
        .filter(|val| *val) // convert true to 1 and false to 0
        .count();

    println!("Score: {}", score);
}

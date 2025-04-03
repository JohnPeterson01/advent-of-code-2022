use std::collections::HashSet;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let lines = lines_from_file("./input.txt");

    let mut totals = Vec::new();
    let mut current_group_total = 0;

    for val_str in lines {
        if val_str == "" {
            totals.push(current_group_total);

            current_group_total = 0
        } else {
            let string = val_str.to_string();
            let val = string.parse::<i32>().unwrap();
            // Add to total
            current_group_total = current_group_total + val;
        }
    }

    totals.sort();
    let top_3_max: i32 = totals.iter().rev().take(3).sum();
    println!("{}", top_3_max)
}

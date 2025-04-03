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

    let mut total_max_value = 0;
    let mut current_group_max = 0;
    for val in lines {
        if val == "" {
            if current_group_max > total_max_value {
                total_max_value = current_group_max
            }
            current_group_max = 0
        } else {
            // Add to max
            let string = val.to_string();
            let int = string.parse::<i32>().unwrap();
            current_group_max = current_group_max + int;
            println!("{:?}", int);
        }
    }

    println!("{}", total_max_value)
}

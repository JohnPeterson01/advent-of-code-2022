use crate::CycleDetailsName::NOOP;
use std::collections::HashSet;
use std::str::Chars;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(Debug)]
enum CycleDetailsName {
    ADDX,
    NOOP,
}

struct CycleDetails {
    name: CycleDetailsName,
    cost: i32,
    weight: Option<i32>,
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn convert_line(line: &String) -> CycleDetails {
    // addx V takes two cycles to complete. After two cycles, the X register is increased by the value V. (V can be negative.)
    // noop takes one cycle to complete. It has no other effect.
    if line == "noop" {
        CycleDetails {
            name: CycleDetailsName::NOOP,
            cost: 1,
            weight: None,
        }
    } else {
        // Must have an addx
        let split_line = line.split(" ").collect::<Vec<&str>>();
        let weight = split_line.get(1).unwrap().parse::<i32>().unwrap();

        CycleDetails {
            name: CycleDetailsName::ADDX,
            cost: 2,
            weight: Some(weight),
        }
    }
}

fn apply_cycle(cycle_details: CycleDetails, cycle_count: &mut i32, sprite_value: &mut i32) {
    for _ in 0..cycle_details.cost {
        let sprite_positions = [*sprite_value - 1, *sprite_value, *sprite_value + 1];

        let mut current_crt_position = (*cycle_count - 1) % 40;

        let mut current_crt_value = *cycle_count % 40;
        if current_crt_value == 1 {
            // print new line
            println!("");
        }

        if sprite_positions.contains(&current_crt_position) {
            print!("#")
        } else {
            print!(" ")
        }

        // Start iterating
        *cycle_count += 1;
    }

    if cycle_details.weight.is_some() {
        *sprite_value += cycle_details.weight.unwrap();
    }
}

fn main() {
    let mut cycle_count = 1;
    let mut sprite_value = 1;

    lines_from_file("./input.txt")
        .iter()
        .map(|line| convert_line(line))
        .for_each(|cycle_details| {
            apply_cycle(
                cycle_details,
                &mut cycle_count,
                &mut sprite_value,
                // &milestones,
            )
        });
}

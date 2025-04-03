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

fn apply_cycle(
    cycle_details: CycleDetails,
    cycle_count: &mut i32,
    register_value: &mut i32,
    milestones: &Vec<i32>,
) -> i32 {
    if cycle_details.weight.is_some() {
        println!(
            "cycle_details -> name: {:?}, cost: {}, weight: {}",
            cycle_details.name,
            cycle_details.cost,
            cycle_details.weight.unwrap()
        );
    } else {
        println!(
            "cycle_details -> name: {:?}, cost: {}, weight: n/a",
            cycle_details.name, cycle_details.cost
        );
    }

    let mut value_to_return = 0;
    for _ in 0..cycle_details.cost {
        // Start iterating
        *cycle_count += 1;

        if milestones.contains(cycle_count) {
            value_to_return = *cycle_count * *register_value;
        }
    }

    if cycle_details.weight.is_some() {
        *register_value += cycle_details.weight.unwrap();
    }

    return value_to_return;
}

fn create_milestones() -> Vec<i32> {
    let milestones_tuple = [20, 60, 100, 140, 180, 220];
    let mut milestones = Vec::new();

    for m in milestones_tuple.iter() {
        milestones.push(*m);
    }

    return milestones;
}

fn main() {
    let mut cycle_count = 0;
    let mut register_value = 1;

    let milestones = create_milestones();

    let score: i32 = lines_from_file("./input.txt")
        .iter()
        .map(|line| convert_line(line))
        .map(|cycle_details| {
            apply_cycle(
                cycle_details,
                &mut cycle_count,
                &mut register_value,
                &milestones,
            )
        })
        .sum();

    println!("{}", score)
}

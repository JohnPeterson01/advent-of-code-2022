use std::collections::{HashMap, HashSet};
use std::str::Chars;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

extern crate core;
extern crate regex;

use regex::Regex;

enum OperationType {
    ADD,
    MULTIPLY,
    SELF_MULTIPLY,
}

struct Operation {
    operation_type: OperationType,
    value: Option<i32>,
}

struct MonkeyInput {
    monkey_id: i32,
    starting_items_worry_level: Vec<i32>,
    operation: Operation,
    test_divisor: i32,
    monkey_id_pass: i32, // throw to this monkey if pass
    monkey_id_fail: i32, // throw to this monkey if fail
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_operation(operation_raw: &String) -> Operation {
    let mut operation_value: i32 = 0;
    {
        if operation_raw.contains("old * old") {
            return Operation {
                operation_type: OperationType::SELF_MULTIPLY,
                value: None,
            };
        }

        let re = Regex::new(r"\b(\d+)\b").unwrap();
        operation_value = re
            .captures(operation_raw)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_owned()
            .parse()
            .unwrap();
    }

    let mut operation_type;
    if operation_raw.contains("*") {
        operation_type = OperationType::MULTIPLY;
    } else {
        operation_type = OperationType::ADD;
    }

    return Operation {
        operation_type,
        value: Some(operation_value),
    };
}

fn map_chunks_to_monkey_input(raw_input: Vec<String>) -> MonkeyInput {
    let mut regex = Regex::new(r"Monkey\s*(\d+):").unwrap();
    let captures = regex.captures(raw_input.get(0).unwrap()).unwrap();
    let monkey_id: i32 = captures
        .get(1)
        .map(|m| m.as_str())
        .unwrap()
        .parse()
        .unwrap();

    // Finding the starting items
    let starting_items_raw = raw_input.get(1).unwrap();
    regex = Regex::new(r"\d+").unwrap();
    let starting_items_worry_level: Vec<i32> = regex
        .find_iter(starting_items_raw)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect();

    // Finding the operation
    let operation_raw = raw_input.get(2).unwrap();
    let operation = get_operation(operation_raw);

    // Trying out the block scoping rules in rust:
    // notice how you can use the same variable name
    let mut test_divisor: i32 = 0;
    let test_divisor_raw = raw_input.get(3).unwrap();
    {
        let re = Regex::new(r"\b(\d+)\b").unwrap();
        test_divisor = re
            .captures(test_divisor_raw)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_owned()
            .parse()
            .unwrap();
    }

    // monkey_id_pass
    let mut monkey_id_pass: i32 = 0;
    let monkey_id_pass_raw = raw_input.get(4).unwrap();
    {
        let re = Regex::new(r"\b(\d+)\b").unwrap();
        monkey_id_pass = re
            .captures(monkey_id_pass_raw)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_owned()
            .parse()
            .unwrap();
    }

    // monkey_id_fail
    let mut monkey_id_fail: i32 = 0;
    let monkey_id_fail_raw = raw_input.get(5).unwrap();
    {
        let re = Regex::new(r"\b(\d+)\b").unwrap();
        monkey_id_fail = re
            .captures(monkey_id_fail_raw)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_owned()
            .parse()
            .unwrap();
    }

    return MonkeyInput {
        monkey_id,
        starting_items_worry_level,
        operation,
        test_divisor,
        monkey_id_pass,
        monkey_id_fail,
    };
}

fn split_lines_into_chunks(lines: Vec<String>, chunk_size: i32) -> Vec<Vec<String>> {
    let mut chunk: Vec<String> = Vec::new();
    let mut chunks: Vec<Vec<String>> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        // Check if we need a new chunk
        if index != 0 && (index as i32 % chunk_size) == 0 {
            chunks.push(chunk);
            chunk = Vec::new();
        }

        chunk.push(line.to_owned())
    }
    // Captures the last chunk that doesn't get pushed in the for loop
    chunks.push(chunk);

    return chunks;
}

fn get_starting_items(monkey_inputs: &Vec<MonkeyInput>) -> HashMap<i32, Vec<i128>> {
    let mut data = HashMap::new();
    for mi in monkey_inputs.into_iter() {
        let monkey = mi.to_owned();
        let monkey_id = &monkey.monkey_id;
        let starting_items = &monkey.starting_items_worry_level;

        let new_starting_items = starting_items
            .iter()
            .map(|a| a.to_owned() as i128)
            .collect::<Vec<i128>>();

        data.insert(monkey_id.to_owned(), new_starting_items);
    }

    return data;
}

fn process(
    monkey_inputs: Vec<MonkeyInput>,
    starting_items_data: &mut HashMap<i32, Vec<i128>>,
) -> i32 {
    // Run the iterations 20 times over
    let iteration_count = 20;

    // Records how many times each monkey inspects items
    // monkey_id -> inspection count
    let mut monkey_inspection_counts: HashMap<i32, i32> = HashMap::new();

    // Convert to something more functional (i.e a nested map
    for i in 0..iteration_count {
        println!("iteration count: -----> {}", i);
        for mi in monkey_inputs.iter() {
            let monkey_id = &mi.monkey_id;
            println!("monkey_id: {}", monkey_id);
            let starting_items = starting_items_data.get(&monkey_id).unwrap().to_owned();
            let operation = &mi.operation;

            for starting_worry_level in starting_items.into_iter() {
                let mut current_worry_level: i128 = starting_worry_level.to_owned() as i128;
                println!("current_worry_level: {}", current_worry_level);
                match operation.operation_type {
                    OperationType::ADD => {
                        current_worry_level += operation.value.unwrap() as i128;
                    }
                    OperationType::MULTIPLY => {
                        current_worry_level *= operation.value.unwrap() as i128;
                    }
                    OperationType::SELF_MULTIPLY => current_worry_level *= current_worry_level,
                    _ => panic!("unknown operation type"),
                }

                // Monkey gets bored with item. Worry level is divided by 3
                let new_worry_level = current_worry_level as f64 / 3.0;
                current_worry_level = new_worry_level.floor() as i128;

                // Run divisor test
                let remainder = current_worry_level % mi.test_divisor as i128;

                if remainder == (0 as i128) {
                    // monkey_id_pass
                    let monkey_id_pass = mi.monkey_id_pass;

                    let mut new_starting_items =
                        starting_items_data.get(&monkey_id_pass).unwrap().to_owned();
                    new_starting_items.push(current_worry_level.to_owned() as i128);
                    starting_items_data.insert(monkey_id_pass, new_starting_items);
                } else {
                    // monkey_id_fail
                    let monkey_id_fail = mi.monkey_id_fail;

                    let mut new_starting_items =
                        starting_items_data.get(&monkey_id_fail).unwrap().to_owned();
                    new_starting_items.push(current_worry_level.to_owned() as i128);
                    starting_items_data.insert(monkey_id_fail, new_starting_items);
                }

                // Adding the scores to the inspection count
                let found_existing_score = monkey_inspection_counts.contains_key(&monkey_id);
                if found_existing_score {
                    let existing_score = monkey_inspection_counts.get(&monkey_id).unwrap();
                    monkey_inspection_counts
                        .insert(monkey_id.to_owned(), (existing_score.to_owned() + 1));
                } else {
                    monkey_inspection_counts.insert(monkey_id.to_owned(), 1);
                }
            }

            // Reset the monkey starting items
            starting_items_data.insert(monkey_id.to_owned(), Vec::new());
        }

        // print which monkey has which item:
        println!("$$$$$$$$$$$$$ End of round: $$$$$$$$$$$$$");
        for (monkey_id, monkey_item) in monkey_inspection_counts.iter() {
            println!("monkey id: {}", monkey_id);
            println!("{},", monkey_item);
        }

        // print which monkey has which item:
        for (monkey_id, monkey_items) in starting_items_data.iter() {
            println!("monkey id: {}", monkey_id);
            for monkey_item in monkey_items.iter() {
                print!(" {},", monkey_item);
            }
            println!("");
        }
    }

    let mut totals = monkey_inspection_counts
        .into_iter()
        .map(|data| data.1)
        .collect::<Vec<i32>>();
    totals.sort_unstable_by(|a, b| b.cmp(a));

    return totals.iter().take(2).fold(1, |acc, x| acc * x);
}

fn main() {
    let individual_input_length = 6;
    let lines: Vec<String> = lines_from_file("./input.txt")
        .iter()
        .map(|line| line.to_owned())
        .filter(|line| line.to_owned() != "")
        .collect::<Vec<String>>();
    let monkey_inputs: Vec<MonkeyInput> = split_lines_into_chunks(lines, individual_input_length)
        .iter()
        .map(|chunk| map_chunks_to_monkey_input(chunk.to_owned()))
        .collect();

    let mut starting_items: HashMap<i32, Vec<i128>> = get_starting_items(&monkey_inputs);

    // process monkey inputs
    let score = process(monkey_inputs, &mut starting_items);

    println!("Score: {:}", score)
}

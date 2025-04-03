use std::borrow::Borrow;
use std::collections::HashMap;
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

fn contains_block(line: &String) -> bool {
    return line.contains('[');
}

fn contains_move(line: &String) -> bool {
    return line.contains("move");
}

fn create_stacks(lines: &Vec<String>) -> HashMap<isize, Vec<String>> {
    // e.g line = '[Z] [M] [P]'
    // So we can create a map -> Vec

    // First we create the list map
    let mut stacks_list_map: HashMap<isize, Vec<String>> = HashMap::new();
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if i % 4 == 1 && (c as char).to_string() != " " {
                let map_index = (i - 1) / 4;
                // We've found a value so we should add it to one of our stack lists
                let mut existing_arr: Vec<String> = Vec::new();
                if stacks_list_map.contains_key(&(map_index as isize)) {
                    existing_arr = stacks_list_map
                        .get(&(map_index as isize))
                        .unwrap()
                        .to_owned();
                }
                existing_arr.push((c as char).to_string());
                stacks_list_map.insert((map_index as isize), existing_arr.to_owned());
            }
        }
    }

    // then we just reverse the vecs and push onto stacks
    let mut stacks_map: HashMap<isize, Vec<String>> = HashMap::new();

    for i in 0..stacks_list_map.len() {
        let mut existing_arr = stacks_list_map.get(&(i as isize)).unwrap().to_owned();
        existing_arr.reverse();

        let mut stack: Vec<String> = Vec::new();
        for item in existing_arr {
            stack.push(item)
        }

        stacks_map.insert((i as isize), stack);
    }

    return stacks_map;
}

fn move_stacks(
    count: isize,
    start: isize,
    end: isize,
    stacks: &mut HashMap<isize, Vec<String>>,
) -> &mut HashMap<isize, Vec<String>> {
    // We need the indexes
    let i_start = start - 1;
    let i_end = end - 1;

    if start == end {
        return stacks;
    }

    let mut starting_stack = stacks.get(&i_start).unwrap().to_owned();
    let mut end_stack = stacks.get(&i_end).unwrap().to_owned();

    for i in 0..count {
        let item_to_move = starting_stack.pop().unwrap();
        end_stack.push(item_to_move);
    }

    stacks.insert(i_start, starting_stack);
    stacks.insert(i_end, end_stack);

    return stacks;
}

fn get_move_defintion(line: &String) -> (isize, isize, isize) {
    let split: Vec<String> = line.split(" ").map(|a| a.to_string()).collect();

    let move_count: isize = Some(split.get(1))
        .unwrap()
        .expect("couldn't get value from line")
        .parse()
        .unwrap();
    let start: isize = Some(split.get(3))
        .unwrap()
        .expect("couldn't get value from line")
        .parse()
        .unwrap();
    let stop: isize = Some(split.get(5))
        .unwrap()
        .expect("couldn't get value from line")
        .parse()
        .unwrap();

    return (move_count, start, stop);
}

fn main() {
    // This gets the initial blocks
    let blocks: Vec<String> = lines_from_file("./input.txt")
        .iter()
        .filter(|line| contains_block(line))
        .map(|a| a.to_owned())
        .collect::<Vec<String>>();

    let moves: Vec<String> = lines_from_file("./input.txt")
        .iter()
        .filter(|line| contains_move(line))
        .map(|a| a.to_owned())
        .collect::<Vec<String>>();

    let mut stacks = create_stacks(&blocks);

    let stack_map = moves
        .iter()
        .map(|line| get_move_defintion(line))
        .fold(&mut stacks, |stacks, (a, b, c)| {
            move_stacks(a, b, c, stacks)
        });

    // Print each of the top of the stacks
    for i in 0..stack_map.len() {
        let mut value = stack_map.get(&(i as isize)).unwrap().to_owned();
        let value_str = value.pop().unwrap();
        print!("{}", value_str)
    }
}

use std::collections::{HashMap, HashSet, VecDeque};
use std::str::Chars;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn create_row(line: &String) -> Vec<String> {
    return line.chars().map(|a| a.to_string()).collect();
}

fn create_grid() -> Vec<Vec<String>> {
    let grid = lines_from_file("./input.txt")
        .iter()
        .map(|line| create_row(line))
        .collect();

    return grid;
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_next_moves(current_coords: (usize, usize), grid: &Vec<Vec<String>>) -> Vec<(usize, usize)> {
    // Gets the next available moves

    let len_rows = grid.len() as i32;
    let len_columns = grid[0].len() as i32;

    let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut moves: Vec<(usize, usize)> = Vec::new();
    for (xx, yy) in neighbors.iter() {
        let new_row_index = current_coords.0 as i32 + xx.to_owned() as i32;
        let new_col_index = current_coords.1 as i32 + yy.to_owned() as i32;

        if new_row_index >= 0
            && new_row_index < len_rows
            && new_col_index >= 0
            && new_col_index < len_columns
        {
            moves.push((new_row_index as usize, new_col_index as usize));
        }
    }

    return moves;
}

fn create_scorecard() -> HashMap<String, i32> {
    // Lowercase item types a through z have priorities 1 through 26.
    let mut scorecard: HashMap<String, i32> = HashMap::new();
    for (index, letter) in (b'a'..=b'z').enumerate() {
        // index starts at 0 but we need to start it from 1
        scorecard.insert((letter as char).to_string(), (index as i32) + 1);
    }

    return scorecard;
}

fn can_move(starting_character: String, ending_character: String) -> bool {
    if &starting_character == "E" {
        return false;
    }

    let scorecard = create_scorecard();
    let mut starting_score;
    if &starting_character == "S" {
        starting_score = 0;
    } else {
        starting_score = scorecard.get(&starting_character).unwrap().to_owned();
    }

    let mut ending_score;
    if &ending_character == "E" {
        ending_score = 26;
    } else {
        ending_score = scorecard.get(&ending_character).unwrap().to_owned();
    }

    return starting_score + 1 >= ending_score.to_owned();
}

fn find_shortest_path(
    starting_coords: (usize, usize),
    ending_coords: (usize, usize),
    grid: &Vec<Vec<String>>,
) -> i32 {
    // Finds the shortest path between two points performing a BFS
    let mut places_been: HashSet<(usize, usize)> = HashSet::new();
    let mut queue_of_work: VecDeque<(usize, usize)> = VecDeque::new();
    queue_of_work.push_back(starting_coords);

    // This stores the cost for each move
    let mut shortest_path_map: HashMap<(usize, usize), i32> = HashMap::new();
    shortest_path_map.insert(starting_coords, 0);

    // Create a HashMap to store the level of each node
    let mut levels = HashMap::new();
    levels.insert(starting_coords, 0);

    let mut cost = 0;

    while let Some(current_coords) = queue_of_work.pop_front() {
        let mut next_queue_of_work: VecDeque<(usize, usize)> = VecDeque::new();
        println!(
            "current_coords: ({}, {})",
            current_coords.0, current_coords.1
        );

        // Potentially don't need this
        if places_been.contains(&(current_coords.0, current_coords.1)) {
            continue;
        } else {
            places_been.insert((current_coords.0.to_owned(), current_coords.1.to_owned()));
        }

        let next_moves = get_next_moves(
            (current_coords.0.to_owned(), current_coords.1.to_owned()),
            &grid,
        );
        for place in next_moves.iter() {
            println!("next_move: ({}, {})", place.0, place.1);
        }

        let current_level = levels[&current_coords];
        for (xx, yy) in next_moves.iter() {
            if places_been.contains(&(xx.to_owned(), yy.to_owned())) {
                continue;
            }

            // Work out if we can move to the new grid character
            let new_character = &grid[xx.to_owned()][yy.to_owned()];

            if new_character == "E" {
                cost = current_level + 1;
            }

            let current_character = &grid[current_coords.0.to_owned()][current_coords.1.to_owned()];
            if can_move(current_character.to_string(), new_character.to_string()) {
                next_queue_of_work.push_back((xx.to_owned(), yy.to_owned()));

                // Work out the cost of moving here i.e 1 + current cost
                shortest_path_map.insert((xx.to_owned(), yy.to_owned()), current_level + 1);

                println!("Inserting level: ({}, {}), {}", xx, yy, current_level + 1);
                levels.insert((xx.to_owned(), yy.to_owned()), current_level + 1);
            }
        }

        queue_of_work.extend(next_queue_of_work);
    }

    // printing the shortest path square
    for (i, row) in grid.iter().enumerate() {
        println!("");
        for (j, character) in row.iter().enumerate() {
            if shortest_path_map.contains_key(&(i, j)) {
                print!("{}, ", shortest_path_map.get(&(i, j)).unwrap())
            } else {
                print!(".")
            }
        }
    }

    return shortest_path_map
        .get(&ending_coords.to_owned())
        .unwrap()
        .to_owned();
}

fn main() {
    // Create grid from input
    let grid: Vec<Vec<String>> = create_grid();

    // Get the starting and ending coords
    let starting_letter = "S";
    let ending_letter = "E";
    let mut starting_coords = (0, 0);
    let mut ending_coords = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, character) in row.iter().enumerate() {
            if character == starting_letter {
                starting_coords = (i, j);
                continue;
            }

            if character == ending_letter {
                ending_coords = (i, j);
                continue;
            }
        }
    }

    let score = find_shortest_path(starting_coords, ending_coords, &grid);

    println!("Score: {}", score)
}

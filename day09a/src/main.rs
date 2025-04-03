use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::str::Chars;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

struct H_Move {
    direction: (i32, i32),
    count: i32,
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn should_move_t(h_coords: &mut (i32, i32), t_coords: &mut (i32, i32)) -> bool {
    // This func looks at all the potential moves around t and sees if h within moving distance
    // if h is within one move then we don't need to move t
    let t_moves = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (0, 0),
    ];

    for (xx, yy) in t_moves.iter() {
        let new_t_xx = t_coords.0 + xx;
        let new_t_yy = t_coords.1 + yy;

        if h_coords.0 == new_t_xx && h_coords.1 == new_t_yy {
            return false;
        }
    }
    return true;
}

fn move_and_record(
    starting_coords: &mut (i32, i32),
    move_direction: (i32, i32),
    positions: &mut HashSet<(i32, i32)>,
) -> (i32, i32) {
    // move t and record the move in the positions set
    // returns back the new coords
    let new_coords = (
        starting_coords.0 + move_direction.0,
        starting_coords.1 + move_direction.1,
    );
    positions.insert(new_coords);

    return new_coords;
}

fn move_h_and_t(
    h_move: H_Move,
    h_coords: &mut (i32, i32),
    t_coords: &mut (i32, i32),
    t_positions: &mut HashSet<(i32, i32)>,
) {

    // Move H
    for _ in 0..h_move.count {
        // move h
        let xx = h_move.direction.0;
        let yy = h_move.direction.1;

        let new_xx = h_coords.0 + xx;
        let new_yy = h_coords.1 + yy;

        h_coords.0 = new_xx;
        h_coords.1 = new_yy;
    }

    // Then we run a check to say "should I move T"?
    // i.e is H coord within current T moves [*..*]
    if !should_move_t(h_coords.borrow_mut(), t_coords.borrow_mut()) {
        // If we don't need to move t then we can just return straight away
        return;
    }

    // First we look to move t diagonally
    let h_row = h_coords.0;
    let h_col = h_coords.1;
    let t_row = t_coords.0;
    let t_col = t_coords.1;

    let mut diagonal_t_move = (0, 0);
    if h_col > t_col && t_row > h_row {
        // Need to move t diagonal right
        diagonal_t_move = (-1, 1);
    } else if h_col > t_col && h_row > t_row {
        // Need to move t down right
        diagonal_t_move = (1, 1);
    } else if t_col > h_col && h_row > t_row {
        // Need to move t down left
        diagonal_t_move = (1, -1);
    } else if t_col > h_col && t_row > h_row {
        // Need to move t up left
        diagonal_t_move = (-1, -1);
    }

    if diagonal_t_move.0 != 0 && diagonal_t_move.1 != 0 {
        // We need to move
        let new_t_coords = move_and_record(t_coords, diagonal_t_move, t_positions);
        t_coords.0 = new_t_coords.0;
        t_coords.1 = new_t_coords.1;
        if !should_move_t(h_coords.borrow_mut(), t_coords.borrow_mut()) {
            // If we don't need to move t then we can just return straight away
            return;
        }
    }

    // if either the row or column match then we know there's no diagonal move to make
    if h_coords.0 == t_coords.0 || h_coords.1 == t_coords.1 {
        // same row or same column
        for _ in 0..h_move.count {
            let new_t_coords = move_and_record(t_coords, h_move.direction, t_positions);
            t_coords.0 = new_t_coords.0;
            t_coords.1 = new_t_coords.1;

            if !should_move_t(h_coords.borrow_mut(), t_coords.borrow_mut()) {
                // If we don't need to move t then we can just return straight away
                return;
            }
        }
    }
}

fn convert_line_to_move(line: &String) -> H_Move {
    let split_line = line.split(" ").collect::<Vec<&str>>();
    let direction_letter = split_line.get(0).unwrap().to_owned();

    // We now need to convert the direction letter
    let mut direction = (0, 0);
    match direction_letter {
        "R" => {
            direction = (0, 1);
        }
        "L" => {
            direction = (0, -1);
        }
        "U" => {
            direction = (-1, 0);
        }
        "D" => {
            direction = (1, 0);
        }
        _ => panic!("Unknown direction"),
    }

    let count = split_line.get(1).unwrap().parse::<i32>().unwrap();

    return H_Move { direction, count };
}

fn main() {
    let mut h_coordinates = (0, 0);
    let mut t_coordinates = (0, 0);

    // Stores all the positions that t has travelled to
    let mut t_positions: HashSet<(i32, i32)> = HashSet::new();

    // Insert one for the starting position
    t_positions.insert((0, 0));

    lines_from_file("./example.txt")
        .iter()
        .map(|line| convert_line_to_move(line))
        .for_each(|h_move| {
            move_h_and_t(
                h_move,
                &mut h_coordinates,
                &mut t_coordinates,
                &mut t_positions,
            )
        });

    println!("{}", t_positions.len())
}

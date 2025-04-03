use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_char(str: &str, position: usize) -> String {
    str.chars().nth(position).unwrap().to_string()
}

fn get_player_moves(line: String) -> (String, String) {
    let player_1_move = get_char(&line, 0);
    let player_2_move = get_char(&line, 2);
    return (player_1_move, player_2_move);
}

fn get_move_as_int(move_str: &str) -> i32 {
    // Improvement would be to make this a constant i.e use phf;
    let moves: HashMap<String, i32> = HashMap::<_, _>::from_iter(IntoIter::new([
        ("A".to_string(), 1),
        ("B".to_string(), 2),
        ("C".to_string(), 3),
        ("X".to_string(), 1),
        ("Y".to_string(), 2),
        ("Z".to_string(), 3),
    ]));
    return moves.get(move_str).copied().unwrap();
}

fn calculate_score(player_1_move: String, player_2_move: String) -> i32 {
    // Rules of the game
    // 1 for Rock,
    // 2 for Paper,
    // and 3 for Scissors

    // (0 if you lost, 3 if the round was a draw, and 6 if you won)

    // We'll do it from the POV of player 1 winning the game

    // Move to constant outside...
    let player_1_move_int = get_move_as_int(&player_1_move);
    let player_2_move_int = get_move_as_int(&player_2_move);

    let rules: HashMap<i32, i32> =
        HashMap::<_, _>::from_iter(IntoIter::new([(1, 3), (2, 1), (3, 2)]));
    let losing_player_2_move = rules.get(&player_1_move_int).unwrap();
    if &player_2_move_int == losing_player_2_move {
        // Player 1 wins
        return 6 + player_1_move_int;
    }

    if player_1_move_int == player_2_move_int {
        // Draw
        return 3 + player_1_move_int;
    }
    // Loss
    return 0 + player_1_move_int;
}

fn main() {
    // Calculate the score you would get if you were to follow the strategy guide
    println!("Calculating rock paper scissors score...");

    let mut scores: Vec<i32> = Vec::new();
    let lines = lines_from_file("./input.txt");
    for line in lines {
        let (player_1_move, player_2_move): (String, String) = get_player_moves(line);
        let score: i32 = calculate_score(player_2_move, player_1_move);
        scores.push(score);
    }

    let scores_sum: i32 = scores.iter().sum();
    println!("{}", scores_sum)
}

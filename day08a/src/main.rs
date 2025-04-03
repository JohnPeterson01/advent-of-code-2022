use std::collections::HashSet;
use std::str::Chars;
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

fn create_row(line: &String) -> Vec<i32> {
    return line
        .chars()
        .map(|a| a.to_string().parse::<i32>().unwrap())
        .collect();
}

fn create_grid() -> Vec<Vec<i32>> {
    let grid = lines_from_file("./example.txt")
        .iter()
        .map(|line| create_row(line))
        .collect();

    return grid;
}

fn check_if_edge(row_index: i32, col_index: i32, grid: &Vec<Vec<i32>>) -> bool {
    let len_rows = grid.len();
    let len_columns = grid[0].len();

    return row_index == 0
        || row_index == ((len_rows as i32) - 1)
        || col_index == 0
        || col_index == ((len_columns as i32) - 1);
}

fn check_if_visible(
    row_index: i32,
    col_index: i32,
    grid: &Vec<Vec<i32>>,
    tree_height: &i32,
    neighbour_direction: Option<(&i32, &i32)>,
) -> bool {
    // First we check fo see if it's an edge value
    if check_if_edge(row_index, col_index, grid) {
        // if it is then we can assert that the tree is visible from the outside
        return true;
    }

    let neighbors = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    for (xx, yy) in neighbors.iter() {
        // neighbour_direction is a specified direction
        // we can skip all directions if one is pre specified
        if neighbour_direction.is_some() && neighbour_direction.unwrap() != (xx, yy) {
            continue;
        }

        // We want to keep iterating in the up,down,left,right direction until we reach
        // the end of the line
        let new_row_index = row_index + xx;
        let new_col_index = col_index + yy;
        let new_tree_height = grid[new_row_index as usize][new_col_index as usize];

        // If it's an edge and the new tree height is less then we can assert that the tree
        // is visible from the outside
        if &new_tree_height < tree_height && check_if_edge(new_row_index, new_col_index, grid) {
            return true;
        }

        // if the new tree height is more than or equalled to the current height, then we can continue
        // iterating over the neighbours
        if &new_tree_height >= tree_height {
            continue;
        }

        // if the new tree height is less but it is not an edge then we need to keep checking in that line of trees (row or column)
        if &new_tree_height < tree_height && !check_if_edge(new_row_index, new_col_index, grid) {
            let visible = check_if_visible(
                new_row_index as i32,
                new_col_index as i32,
                &grid,
                &tree_height,
                Some((xx, yy)),
            );
            if visible {
                return true;
            } else {
                continue;
            }
        }
    }

    return false;
}

fn main() {
    // Create a grid [[row1],[row2]] where coordinates are (row, column)
    let grid: Vec<Vec<i32>> = create_grid();

    let score: usize = grid
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            return row
                .iter()
                .enumerate()
                .map(|(col_index, tree_height)| {
                    check_if_visible(
                        row_index as i32,
                        col_index as i32,
                        &grid,
                        tree_height,
                        None,
                    )
                })
                .collect::<Vec<bool>>();
        })
        .flatten()
        .filter(|a| a.to_owned())
        .count();

    println!("Score: {}", score);
}

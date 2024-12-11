use std::collections::HashMap;
use std::io::BufRead;
use std::sync::Mutex;
use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
};

fn init_directions() -> HashMap<char, (i8, i8)> {
    let mut map = HashMap::new();
    map.insert('^', (-1, 0));
    map.insert('>', (0, 1));
    map.insert('v', (1, 0));
    map.insert('<', (0, -1));
    map
}

fn init_next_direction() -> HashMap<char, char> {
    let mut map = HashMap::new();
    map.insert('^', '>');
    map.insert('>', 'v');
    map.insert('v', '<');
    map.insert('<', '^');
    map
}

pub fn file_to_buffer(filename: String) -> BufReader<Cursor<String>> {
    let file = File::open(filename);
    let mut content = String::new();
    file.unwrap().read_to_string(&mut content).unwrap();

    let cursor = Cursor::new(content);
    let reader = BufReader::new(cursor);
    reader
}

pub fn buf_reader_to_matrix(reader: BufReader<Cursor<String>>) -> Vec<Vec<char>> {
    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    return matrix;
}

pub fn find_special_symbols(
    matrix: &Vec<Vec<char>>,
    symbols: Vec<char>,
) -> HashMap<char, Vec<(usize, usize)>> {
    println!(
        "Finding symbols {:?} in matrix of size: {}, {}",
        symbols,
        matrix[0].len(),
        matrix.len()
    );
    let total_rows = matrix[0].len();
    let total_columns = matrix.len();

    let mut symbols_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for row in 0..total_rows {
        for column in 0..total_columns {
            if symbols.contains(&matrix[row][column]) {
                let symbol = matrix[row][column];
                symbols_positions
                    .entry(symbol)
                    .or_insert_with(Vec::new)
                    .push((row, column));
            }
        }
    }
    symbols_positions
}

pub fn calculate_movements(
    matrix: &Vec<Vec<char>>,
    mut path_matrix: Vec<Vec<i32>>,
    guard_initial_position: (usize, usize),
    guard_initial_direction: char,
) -> i32 {
    let directions = init_directions();
    let next_direction = init_next_direction();
    // mark initial position in path_matrix.
    path_matrix[guard_initial_position.0][guard_initial_position.1] = 1;
    // start walking.
    let mut position = (
        guard_initial_position.0 as i128,
        guard_initial_position.1 as i128,
    );
    let mut direction = guard_initial_direction;
    let mut direction_value = directions.get(&direction).unwrap();
    loop {
        let next_proposed_position = (
            position.0 as i128 + direction_value.0 as i128,
            position.1 as i128 + direction_value.1 as i128,
        );
        if !check_boundaries(
            matrix,
            next_proposed_position.0 as usize,
            next_proposed_position.1 as usize,
        ) {
            println!("Guard wants to leave the map (out of bounds)!");
            break;
        }
        print!("Guard see if can move to {:?} ->", next_proposed_position);
        if matrix[next_proposed_position.0 as usize][next_proposed_position.1 as usize] == '#' {
            direction = *next_direction.get(&direction).unwrap();
            direction_value = directions.get(&direction).unwrap();
            println!(
                "There is an obstacle at {:?}, changing direction to {}",
                next_proposed_position, direction
            );
        } else {
            position = next_proposed_position;
            path_matrix[position.0 as usize][position.1 as usize] = 1;
            println!("Guard advanced to {:?}", position)
        }
    }
    sum_matrix(path_matrix)
}

fn check_boundaries(matrix: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    x >= 0 && y >= 0 && y < matrix.len() && x < matrix[y].len()
}

fn sum_matrix(matrix: Vec<Vec<i32>>) -> i32 {
    matrix.iter().map(|row| row.iter().sum::<i32>()).sum()
}

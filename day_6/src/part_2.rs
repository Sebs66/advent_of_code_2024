use crate::utils;
use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use utils::{buf_reader_to_matrix, find_special_symbols, Direction, GuardCoords};

pub fn part_2(reader: BufReader<Cursor<String>>) -> i16 {
    let matrix = buf_reader_to_matrix(reader);
    let mut path_matrix = vec![vec![0 as i16; matrix[0].len()]; matrix.len()];

    let mut visited_positions: HashMap<GuardCoords, Vec<Direction>> = HashMap::new();

    let guard_symbol = '^';

    let symbols_positions = find_special_symbols(&matrix, vec![guard_symbol, '#']);
    println!("{:?}", symbols_positions);
    let guard_first_position = symbols_positions.get(&guard_symbol).unwrap()[0];
    let obstacles = symbols_positions.get(&'#').unwrap();

    println!(
        "Guard starts at {:?}, in direction {}",
        guard_first_position,
        Direction::North
    );

    let mut direction = Direction::North;
    for row in matrix {
        for column in row {}
    }
}

use crate::utils;
use std::io::{BufReader, Cursor};
use utils::{
    buf_reader_to_matrix, find_next_obstacle, find_special_symbols, sum_matrix, Direction,
};

pub fn part_1(reader: BufReader<Cursor<String>>) -> i16 {
    let matrix = buf_reader_to_matrix(reader);
    let mut path_matrix = vec![vec![0 as i16; matrix[0].len()]; matrix.len()];

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
    let mut guard_position = guard_first_position;
    loop {
        let (is_obstacle, guard_new_position, obstacle_position) =
            find_next_obstacle(guard_position, &direction, obstacles, &mut path_matrix);
        if !is_obstacle {
            break;
        }
        direction = direction.turn_rigth();
        guard_position = guard_new_position;

        println!(
            "is_obstacle: {}, guard at {:?} , obstacle: {:?}",
            is_obstacle, guard_position, obstacle_position
        );
    }
    sum_matrix(path_matrix)
}

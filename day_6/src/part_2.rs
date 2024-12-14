use crate::utils;
use std::collections::HashMap;
use std::io::{BufReader, Cursor};
use std::ptr::null;
use utils::{
    buf_reader_to_matrix, find_next_obstacle, find_special_symbols, Direction, GuardCoords,
    ObstaclesCoords,
};

pub fn part_2(reader: BufReader<Cursor<String>>) -> i32 {
    let mut matrix = buf_reader_to_matrix(reader);
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

    let mut loops_count = 0;

    for row in 0..matrix.len() {
        for column in 0..matrix[0].len() {
            let symbol = matrix[row][column];
            if symbol != '.' {
                continue;
            }
            let mut new_matrix = matrix.clone();
            new_matrix[row][column] = '#';

            let symbols_positions = find_special_symbols(&new_matrix, vec![guard_symbol, '#']);
            let obstacles = symbols_positions.get(&'#').unwrap();

            if walking_loop(guard_first_position, obstacles, &mut path_matrix) {
                loops_count += 1;
            }
        }
    }
    return loops_count;
}

fn walking_loop(
    guard_first_position: GuardCoords,
    obstacles: &ObstaclesCoords,
    mut path_matrix: &mut Vec<Vec<i16>>,
) -> bool {
    let mut direction = Direction::North;
    let mut guard_position = guard_first_position;

    let mut visited_positions: HashMap<GuardCoords, Vec<Direction>> = HashMap::new();

    loop {
        let (is_obstacle, guard_new_position, obstacle_position) =
            find_next_obstacle(guard_position, &direction, obstacles, &mut path_matrix);
        if !is_obstacle {
            break;
        }

        if let Some(directions) = visited_positions.get(&guard_position) {
            if directions.contains(&direction) {
                println!(
                    "Already visited {:?} with direction {}. We found a loop!",
                    guard_position, direction
                );
                return true;
            }
        }

        visited_positions
            .entry(guard_position)
            .and_modify(|directions| directions.push(direction))
            .or_insert(vec![direction.clone()]);

        direction = direction.turn_rigth();
        guard_position = guard_new_position;

        // println!(
        //     "is_obstacle: {}, guard at {:?} , obstacle: {:?}",
        //     is_obstacle, guard_position, obstacle_position
        // );
    }
    return false;
}

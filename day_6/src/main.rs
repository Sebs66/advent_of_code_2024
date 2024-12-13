mod utils;
use std::env::{self};
use std::io::{BufReader, Cursor};
use utils::{buf_reader_to_matrix, calculate_movements, file_to_buffer, find_special_symbols};
fn main() {
    let reader = file_to_buffer(String::from("example"));
    let result = part_1(reader);
    println!("{}", result);
    println!("\n\n\n\n");
}

fn part_1(reader: BufReader<Cursor<String>>) -> i32 {
    let matrix = buf_reader_to_matrix(reader);
    let path_matrix = vec![vec![0; matrix[0].len()]; matrix.len()];
    let guard_symbol = '^';

    let symbols_positions = find_special_symbols(&matrix, vec![guard_symbol, '#']);
    println!("{:?}", symbols_positions);
    let guard_first_position = symbols_positions.get(&guard_symbol).unwrap()[0];

    calculate_movements(
        &matrix,
        path_matrix,
        guard_first_position,
        '^',
        symbols_positions.get(&'#').unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use crate::{file_to_buffer, part_1};

    #[test]
    fn test_part1() {
        let reader = file_to_buffer(String::from("example"));
        let total_distinct_positions = part_1(reader);
        println!("total_distinct_positions: {}", total_distinct_positions);
        assert_eq!(total_distinct_positions, 41)
    }
}

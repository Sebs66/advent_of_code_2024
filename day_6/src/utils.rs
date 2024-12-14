use std::collections::HashMap;
use std::env;
use std::fmt;
use std::io::BufRead;
use std::sync::Mutex;
use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
};

type Coords = (i32, i32);
type ObstacleCoords = Coords;
pub type GuardCoords = Coords;
type DirectionCoords = Coords;
type ObstaclesCoords = Vec<ObstacleCoords>;

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn_rigth(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn coords(&self) -> DirectionCoords {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let direction_str = match self {
            Direction::North => "^",
            Direction::South => "v",
            Direction::East => ">",
            Direction::West => "<",
        };
        write!(f, "{}", direction_str)
    }
}
pub fn file_to_buffer(filename: String) -> BufReader<Cursor<String>> {
    let current_dir = env::current_dir().unwrap();
    let folder = current_dir.parent().unwrap(); // Propagates the error.

    let file_path = folder.join(filename);
    let file = File::open(file_path);
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
) -> HashMap<char, Vec<(i32, i32)>> {
    println!(
        "Finding symbols {:?} in matrix of size: {}, {}",
        symbols,
        matrix[0].len(),
        matrix.len()
    );
    let total_rows = matrix[0].len();
    let total_columns = matrix.len();

    let mut symbols_positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for row in 0..total_rows {
        for column in 0..total_columns {
            if symbols.contains(&matrix[row][column]) {
                let symbol = matrix[row][column];
                symbols_positions
                    .entry(symbol)
                    .or_insert_with(Vec::new)
                    .push((row as i32, column as i32).try_into().unwrap());
            }
        }
    }
    symbols_positions
}

fn check_boundaries(matrix: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    x >= 0 && y >= 0 && y < matrix.len() && x < matrix[y].len()
}

fn is_safe(position: GuardCoords, boundaries: &Coords) -> bool {
    position.0 >= 0 && position.1 >= 0 && position.0 <= boundaries.0 && position.1 <= boundaries.1
}

pub fn sum_matrix(matrix: Vec<Vec<i16>>) -> i16 {
    matrix.iter().map(|row| row.iter().sum::<i16>()).sum()
}

///
/// Finds the next obstacle in the path of the guard in that direction,
/// If no obstacle is encounter, returns false and the coord of the boundary.
pub fn find_next_obstacle(
    guard_position: GuardCoords,
    direction: &Direction,
    obstacles: &ObstaclesCoords,
    path_matrix: &mut Vec<Vec<i16>>,
) -> (bool, GuardCoords, ObstacleCoords) {
    let boundaries = &obstacles
        .iter() // Use `iter` instead of `into_iter` to avoid consuming
        .fold((i32::MIN, i32::MIN), |acc, &(y, x)| {
            (acc.0.max(x), acc.1.max(y))
        });

    let mut position = guard_position;
    let mut last_position = position;
    let direction_coords = direction.coords();
    let mut is_obstacle_found = false;
    while is_safe(position, boundaries) {
        path_matrix[position.0 as usize][position.1 as usize] = 1;
        position = (
            position.0 + direction_coords.0,
            position.1 + direction_coords.1,
        );
        if obstacles.contains(&position) {
            is_obstacle_found = true;
            break;
        }
        last_position = position;
    }
    (is_obstacle_found, last_position, position)
}

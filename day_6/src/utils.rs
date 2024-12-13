use std::collections::HashMap;
use std::env;
use std::io::BufRead;
use std::sync::Mutex;
use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
};

type ObstacleCoords = (i128, i128);
type Direction = char;
type GuardCoords = (i128, i128);

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
    obstacles_positions: &Vec<(usize, usize)>,
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
    let mut candidate_obstacles: Vec<(i128, i128)> = vec![];
    candidate_obstacles = check_next_obstacles(
        &obstacles_positions,
        direction,
        position,
        candidate_obstacles,
    );
    println!("Candidate obstacles: {:?}", candidate_obstacles);

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
            println!("Guard wants to leave the map (End of simulation)!");
            break;
        }
        // print!("Guard see if can move to {:?} ->", next_proposed_position);
        if matrix[next_proposed_position.0 as usize][next_proposed_position.1 as usize] == '#' {
            direction = *next_direction.get(&direction).unwrap();
            direction_value = directions.get(&direction).unwrap();
            println!(
                "At {:?}, There is an obstacle at {:?}, changing direction to '{}' {:?} ",
                position, next_proposed_position, direction, direction_value
            );
        } else {
            position = next_proposed_position;
            path_matrix[position.0 as usize][position.1 as usize] = 1;
            // println!("Guard advanced to {:?}", position)
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

fn check_next_obstacles(
    obstacles_positions: &Vec<(usize, usize)>,
    next_direction: char,
    position: (i128, i128),
    candidate_obstacles: Vec<GuardCoords>,
) -> Vec<GuardCoords> {
    println!(
        "Starting at: {:?}, we check for obstacles in the {} direction",
        position, next_direction
    );
    let (mut is_an_obstacle, _, mut new_direction, mut new_position, mut candidate_obstacles) =
        find_next_obstacles(
            obstacles_positions,
            position,
            next_direction,
            candidate_obstacles,
        );
    while is_an_obstacle {
        (
            is_an_obstacle,
            _,
            new_direction,
            new_position,
            candidate_obstacles,
        ) = find_next_obstacles(
            obstacles_positions,
            new_position,
            new_direction,
            candidate_obstacles,
        );
    }
    return candidate_obstacles;
}

fn find_next_obstacles(
    obstacles_positions: &Vec<(usize, usize)>,
    position: GuardCoords,
    direction: Direction,
    mut candidate_obstacles: Vec<ObstacleCoords>,
) -> (
    bool,
    ObstacleCoords,
    Direction,
    GuardCoords,
    Vec<GuardCoords>,
) {
    // println!(
    //     "find_next_obstacle: Looking for obstacles starting at {:?} in direction {}",
    //     position, direction
    // );
    // First to see if any of the obstacles are in the same direction. so in the same row/column as the 0 dir value.
    // And if the second dir value is 1, > than , if its -1 < than.
    // Examples.
    // Position (row:1, col: 4) and direction (row:0,col:1)
    // Look for obstacles with row=1 and col>4.
    // Position (row:1, col:9) and direction (row:1, col: 0)
    // Look for obstacles with row>1 and col=9

    candidate_obstacles = evaluate_candidate_position_for_obstacle(
        position,
        direction,
        obstacles_positions,
        candidate_obstacles,
    );
    let (is_obstacle, obstacle_coords, next_direction, new_position) =
        evaluate_next_obstacle(position, direction, obstacles_positions);

    return (
        is_obstacle,
        obstacle_coords,
        next_direction,
        new_position,
        candidate_obstacles,
    );
}

fn evaluate_next_obstacle(
    position: (i128, i128),
    direction: char,
    obstacles_positions: &Vec<(usize, usize)>,
) -> (bool, ObstacleCoords, Direction, GuardCoords) {
    let directions = init_directions();
    let direction_values = directions.get(&direction).unwrap();
    let coordinate_to_match = if direction_values.0 == 0 { 0 } else { 1 };

    for &(obstacle_row, obstacle_col) in obstacles_positions {
        let obstacle_position = (obstacle_row as i128, obstacle_col as i128);
        let mut evaluation: bool = false;

        if coordinate_to_match == 0 && obstacle_position.0 == position.0 {
            // println!(
            //     "find_next_obstacle: Obstacle at {:?} matched first coordinate",
            //     obstacle_position
            // );
            evaluation = match direction_values.1 {
                1 => position.1 < obstacle_position.1,
                -1 => position.1 > obstacle_position.1,
                _ => false,
            };
        } else if coordinate_to_match == 1 && obstacle_position.1 == position.1 {
            // println!(
            //     "find_next_obstacle: Obstacle at {:?} matched second coordinate",
            //     obstacle_position
            // );
            evaluation = match direction_values.0 {
                1 => position.0 < obstacle_position.0,
                -1 => position.0 > obstacle_position.0,
                _ => false,
            };
        }
        if evaluation == true {
            let position_before_obstacle = (
                obstacle_position.0 - direction_values.0 as i128,
                obstacle_position.1 - direction_values.1 as i128,
            );
            let next_directions = init_next_direction();
            let next_direction = *next_directions.get(&direction).unwrap();
            println!(
                "find_next_obstacle: {:?} --{} obstacle at {:?}.           Changing direction to {}",
                position_before_obstacle, direction, obstacle_position,next_direction
            );
            return (
                true,
                obstacle_position,
                next_direction,
                position_before_obstacle,
            );
        }
    }
    return (false, (-1, -1), '0', (-1, -1));
}

fn evaluate_candidate_position_for_obstacle(
    initial_guard_position: (i128, i128),
    direction: char,
    obstacles_positions: &Vec<(usize, usize)>,
    mut candidate_obstacles: Vec<ObstacleCoords>,
) -> Vec<ObstacleCoords> {
    let directions_mapper = init_directions();

    let (_, obstacle_coords, new_direction, _) =
        evaluate_next_obstacle(initial_guard_position, direction, obstacles_positions);

    if obstacle_coords.0 == -1 || obstacle_coords.1 == -1 {
        return candidate_obstacles;
    }
    println!(
        "evaluate_candidate_position_for_obstacle: searching between {:?} and {:?} in {} direction",
        initial_guard_position, obstacle_coords, direction
    );
    let direction_increment = *directions_mapper.get(&direction).unwrap();
    let direction_increment = (direction_increment.0 as i128, direction_increment.1 as i128);

    let mut candidate_coords = initial_guard_position;
    while candidate_coords != obstacle_coords {
        candidate_coords.0 += direction_increment.0;
        candidate_coords.1 += direction_increment.1;
        let (is_obstacle, obstacle_2_coords, _, _) =
            evaluate_next_obstacle(candidate_coords, new_direction, obstacles_positions);

        if is_obstacle {
            candidate_coords.0 += direction_increment.0;
            candidate_coords.1 += direction_increment.1;
            if !(candidate_coords.0 == obstacle_coords.0 && candidate_coords.1 == obstacle_coords.1)
            {
                println!(
                    "With a candidate obstacle at {:?}, we encounter an obtacle at {:?}!",
                    candidate_coords, obstacle_2_coords
                );
                candidate_obstacles.push(candidate_coords);
            }
        }
    }
    return candidate_obstacles;
}

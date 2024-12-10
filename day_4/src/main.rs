use core::fmt;
use std::arch::x86_64::_mm256_permutevar8x32_epi32;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead, BufReader, Cursor, Read};

#[derive(Debug, Hash, Eq, PartialEq)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
#[derive(Debug)]
struct Direction {
    x: i32,
    y: i32,
}

// Define the HashMap outside of main
fn get_directions() -> HashMap<Directions, Direction> {
    let mut directions = HashMap::new();

    directions.insert(Directions::Up, Direction { x: 0, y: -1 });
    directions.insert(Directions::Down, Direction { x: 0, y: 1 });
    directions.insert(Directions::Left, Direction { x: -1, y: 0 });
    directions.insert(Directions::Right, Direction { x: 1, y: 0 });
    directions.insert(Directions::UpLeft, Direction { x: -1, y: -1 });
    directions.insert(Directions::UpRight, Direction { x: 1, y: -1 });
    directions.insert(Directions::DownLeft, Direction { x: -1, y: 1 });
    directions.insert(Directions::DownRight, Direction { x: 1, y: 1 });

    directions
}

fn get_directions_v2() -> HashMap<Directions, Direction> {
    let mut directions = HashMap::new();

    directions.insert(Directions::UpLeft, Direction { x: -1, y: -1 });
    directions.insert(Directions::UpRight, Direction { x: 1, y: -1 });
    directions.insert(Directions::DownLeft, Direction { x: -1, y: 1 });
    directions.insert(Directions::DownRight, Direction { x: 1, y: 1 });

    directions
}

impl fmt::Display for Directions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let buffer = read_file(String::from("input"));
    let matrix = buffer_to_matrix(buffer.unwrap());
    let count = part_1(matrix);
    println!("count: {}", count);
    let buffer = read_file(String::from("input"));
    let matrix = buffer_to_matrix(buffer.unwrap());
    let count = part_2(matrix);
    println!("count: {}", count);
}

fn read_file(filename: String) -> Result<BufReader<Cursor<String>>, io::Error> {
    let current_dir = env::current_dir()?; // Propagates the error.
    let file_path = current_dir.join(filename);

    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let cursor = Cursor::new(content);

    let reader = BufReader::new(cursor);
    Ok(reader)
}
fn buffer_to_matrix(reader: BufReader<Cursor<String>>) -> Vec<Vec<char>> {
    let matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    matrix
}
fn part_1(matrix: Vec<Vec<char>>) -> i32 {
    let mut xmass_count = 0;
    for (line_index, line) in matrix.iter().enumerate() {
        for (column_index, char) in line.iter().enumerate() {
            if char != &'X' {
                continue;
            }
            // println!(
            //     "char at line: {} column: {} is : {}",
            //     line_index, column_index, char
            // );
            let directions = get_directions();
            let chars_num = 4;
            let mut words: Vec<String> = vec![];
            for (dir_name, direction) in directions {
                let word =
                    retrieve_chars_at_dir(&matrix, line_index, column_index, &direction, chars_num);
                words.push(word);
            }
            // println!("{:?}", words);
            for word in words {
                if word == "XMAS" {
                    xmass_count += 1;
                }
            }
        }
    }
    xmass_count
}

fn part_2(matrix: Vec<Vec<char>>) -> i32 {
    let mut xmass_count = 0;
    for (line_index, line) in matrix.iter().enumerate() {
        for (column_index, char) in line.iter().enumerate() {
            if char != &'A' {
                continue;
            }
            // println!(
            //     "char at line: {} column: {} is : {}",
            //     line_index, column_index, char
            // );
            if retrieve_words_x_mas(&matrix, line_index, column_index) {
                xmass_count += 1;
            }
            // println!("{:?}", words);
        }
    }
    xmass_count
}

fn retrieve_chars_at_dir(
    matrix: &Vec<Vec<char>>,
    y_coord: usize,
    x_coord: usize,
    dir: &Direction,
    chars_num: i32,
) -> String {
    let mut string = String::new();
    string.push(matrix[y_coord][x_coord]);

    // println!(
    //     "Searching for xmass words starting at coordinate ({},{}) at dir {:?}",
    //     x_coord, y_coord, dir
    // );
    for index in 1..(chars_num) {
        let x = x_coord as i32 + dir.x * index;
        let y = y_coord as i32 + dir.y * index;
        if x >= 0 && y >= 0 {
            let x = x as usize;
            let y = y as usize;
            if y < matrix.len() && x < matrix[y].len() {
                let char_ = matrix[y][x];
                string.push(char_);
            } else {
                break;
            }
        } else {
            break;
        }
    }
    // println!("{}", string);
    return string;
}

fn check_boundaries(matrix: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
    x >= 0 && y >= 0 && y < matrix.len() && x < matrix[y].len()
}

fn retrieve_words_x_mas(matrix: &Vec<Vec<char>>, y_coord: usize, x_coord: usize) -> bool {
    let mut directions_1 = HashMap::new();
    let mut directions_2 = HashMap::new();

    directions_1.insert(Directions::UpLeft, Direction { x: -1, y: -1 });
    directions_1.insert(Directions::DownRight, Direction { x: 1, y: 1 });

    directions_2.insert(Directions::UpRight, Direction { x: 1, y: -1 });
    directions_2.insert(Directions::DownLeft, Direction { x: -1, y: 1 });

    fn retrieve_x_mas_by_dirs(
        directions: HashMap<Directions, Direction>,
        matrix: &Vec<Vec<char>>,
        y_coord: usize,
        x_coord: usize,
    ) -> Vec<String> {
        let mut words_dir: Vec<String> = vec![];
        for dir in directions.values() {
            let mut word = String::new();
            let x = x_coord as i32;
            let y = y_coord as i32;
            let start_x = x + dir.x * -1;
            let start_y = y + dir.y * -1;
            if !check_boundaries(matrix, start_y as usize, start_x as usize) {
                continue;
            }
            let char = matrix[start_y as usize][start_x as usize];
            word.push(char);

            for index in 0..2 {
                let x = x_coord as i32 + dir.x * index;
                let y = y_coord as i32 + dir.y * index;
                if check_boundaries(matrix, y as usize, x as usize) {
                    let char = matrix[y as usize][x as usize];
                    word.push(char);
                }
            }
            if word == "MAS" {
                words_dir.push(word);
            }
        }
        words_dir
    }
    let words_1 = retrieve_x_mas_by_dirs(directions_1, matrix, y_coord, x_coord);
    let words_2 = retrieve_x_mas_by_dirs(directions_2, matrix, y_coord, x_coord);

    if words_1.len() == 1 && words_1[0] == "MAS" && words_2.len() == 1 && words_2[0] == "MAS" {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use crate::{buffer_to_matrix, part_1, part_2};

    #[test]
    fn test_part1() {
        let test_input = String::from(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        let cursor = Cursor::new(test_input); // In-memory cursor over the input string
        let reader: BufReader<Cursor<String>> = BufReader::new(cursor);
        let matrix = buffer_to_matrix(reader);
        let xmass_count = part_1(matrix);
        println!("count: {}", xmass_count)
    }

    #[test]
    fn test_part2() {
        let test_input = String::from(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        let cursor = Cursor::new(test_input); // In-memory cursor over the input string
        let reader: BufReader<Cursor<String>> = BufReader::new(cursor);
        let matrix = buffer_to_matrix(reader);
        let xmass_count = part_2(matrix);
        println!("count: {}", xmass_count)
    }
}

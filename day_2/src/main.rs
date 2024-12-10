mod alternative;
use core::num;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Cursor, Read};

fn main() -> io::Result<()> {
    // let reader = read_file(String::from("input"))?;
    // let safe_lines = part_1(reader);
    // let reader = read_file(String::from("input"))?;
    // let safe_lines_2 = part_1_two(reader);
    // println!("safe_lines: {}", safe_lines?);
    // println!("safe_lines: {}", safe_lines_2?);
    let reader = read_file(String::from("input"))?;
    // let safe_lines_part2 = part_2(reader);
    let safe_lines_part2 = alternative::part_2(reader);
    println!("safe_lines part2: {}", safe_lines_part2?);

    Ok(())
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

fn part_1(reader: BufReader<Cursor<String>>) -> io::Result<i32> {
    let mut correct_lines = 0;
    for line in reader.lines() {
        let mut state = 0;
        let line = line?;
        let parsed_vector: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let first_number = parsed_vector[0];
        let second_number = parsed_vector[1];
        let dif_first_second = (second_number - first_number).abs();
        if dif_first_second > 3 || dif_first_second < 1 {
            continue; // line is not safe.
        }
        let direction = if second_number > first_number { 1 } else { -1 }; // 1 -> increasing, -1 -> decreasing.
        let mut previous_number = &second_number;
        for number in &parsed_vector[2..] {
            if (previous_number - number).abs() > 3 {
                state = -1; // line is not safe.
                break;
            }
            let local_direction = if number > previous_number { 1 } else { -1 };
            if local_direction != direction {
                state = -1;
                break;
            }
            if previous_number - number == 0 {
                state = -1;
                break;
            }
            previous_number = number;
            // print!("{}, ", number);
        }
        if state == 0 {
            correct_lines += 1;
        }
        // println!("");
    }
    Ok(correct_lines)
}

fn part_1_two(reader: BufReader<Cursor<String>>) -> io::Result<i32> {
    let mut correct_lines = 0;
    for line in reader.lines() {
        let mut state = 0;
        let line = line?;
        let parsed_vector: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut direction = 0;
        let mut prev_number: Option<i32> = None;
        for number in &parsed_vector {
            if prev_number.is_none() {
                prev_number = Some(*number);
                continue;
            }
            let dif_numbers = (prev_number.unwrap() - *number).abs();
            if dif_numbers > 3 || dif_numbers < 1 {
                state = -1; // line is not safe.
                break;
            }

            let local_direction = if *number > prev_number.unwrap() {
                1
            } else {
                -1
            };
            if direction == 0 {
                direction = local_direction;
            }
            if local_direction != direction {
                state = -1;
                break;
            }
            prev_number = Some(*number);
        }
        if state == 0 {
            correct_lines += 1;
        }
    }
    Ok(correct_lines)
}

fn calculate_direction(number: i32, prev_number: i32) -> i32 {
    let dir = (number - prev_number).cmp(&0);
    let dir = match dir {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
    };
    dir
}
fn part_2(reader: BufReader<Cursor<String>>) -> io::Result<i32> {
    let mut correct_lines = 0;
    let mut line_index = 1;
    for line in reader.lines() {
        let line = line?;
        let parsed_vector: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut prev_number = parsed_vector[0];
        let mut prev_dir = -999;
        let mut prev_prev_number = 0;
        let mut constraints_violations = 0;
        for index in 1..parsed_vector.len() {
            let number = parsed_vector[index];
            let local_dir = calculate_direction(number, prev_number);
            let prev_prev_dir = calculate_direction(number, prev_prev_number);
            let prev_distance = (number - prev_number).abs();
            if prev_dir == -999 {
                prev_dir = local_dir
            }
            if prev_distance > 3 {
                constraints_violations += 1;
                if index + 1 < parsed_vector.len() {
                    let prev_next_distance = (prev_number - parsed_vector[index + 1]).abs();
                    let prev_prev_distance = (number - prev_prev_number).abs();
                    let next_number = parsed_vector[index + 1];
                    let next_distance = (number - next_number).abs();
                    if prev_prev_distance <= 3 {
                        prev_number = number;
                        prev_dir = local_dir;
                        if prev_prev_distance > 3 {
                            constraints_violations += 1;
                            break;
                        }
                    } else if prev_next_distance <= 3 {
                        prev_number = prev_prev_number;
                        prev_dir = prev_prev_dir
                    } else if next_distance < 3 {
                        prev_number = number;
                        prev_dir = local_dir;
                    } else if prev_prev_distance > 3 && prev_next_distance > 3 {
                        constraints_violations += 1;
                        break;
                    } else {
                        constraints_violations += 1;
                        break;
                    }
                }
                continue;
            } else if local_dir == 0 {
                constraints_violations += 1;
                continue;
            } else if prev_dir != local_dir {
                constraints_violations += 1;
                continue;
            }
            prev_dir = local_dir;
            prev_prev_number = prev_number;
            prev_number = number;
        }
        if constraints_violations <= 1 {
            correct_lines += 1;
            println!("{}: correct line {}", line_index, line);
        } else {
            println!("{}: bad line {}", line_index, line);
        }
        line_index += 1;
    }
    println!("{}", correct_lines);
    Ok(correct_lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_part1() {
        let test_input = String::from(
            "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9",
        );
        // Convert the string to a BufReader
        let cursor = Cursor::new(test_input); // In-memory cursor over the input string
        let reader = BufReader::new(cursor);
        let result = part_1(reader).unwrap();
        assert_eq!(result, 2);
    }
    #[test]
    fn test_part1_two() {
        let reader = read_file(String::from("input")).unwrap();
        let result = part_1_two(reader).unwrap();
        assert_eq!(result, 202);
    }

    #[test]
    fn test_part2() {
        let test_input = String::from(
            "7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9",
        );
        // Convert the string to a BufReader
        let cursor = Cursor::new(test_input); // In-memory cursor over the input string
        let reader = BufReader::new(cursor);
        let result = part_2(reader).unwrap();
        assert_eq!(result, 4)
    }
    #[test]
    fn test_part2_fail() {
        let test_input = String::from(
            "80 82 81 82 83 85 88
            48 51 54 55 58 57 55
            61 64 65 66 70 73 76",
        );
        // Convert the string to a BufReader
        let cursor = Cursor::new(test_input); // In-memory cursor over the input string
        let reader = BufReader::new(cursor);
        let result = part_2(reader).unwrap();
        assert_eq!(result, 0)
    }
    #[test]
    fn test_part2_fail_2() {
        let test_input = String::from("61 64 65 66 70 73 76");
        // Convert the string to a BufReader
        let cursor = Cursor::new(test_input); // In-memory cursor over the input string
        let reader = BufReader::new(cursor);
        let result = part_2(reader).unwrap();
        assert_eq!(result, 0)
    }
    #[test]
    fn test_part2_success() {
        let test_input = String::from(
            "60 58 62 65 66 69 71
        16 22 23 26 29 32 35 37",
        );
        // Convert the string to a BufReader
        let cursor = Cursor::new(test_input); // In-memory cursor over the input string
        let reader = BufReader::new(cursor);
        let result = part_2(reader).unwrap();
        assert_eq!(result, 2)
    }
    #[test]
    fn test_part2_file() {
        let reader = read_file(String::from("input")).unwrap();
        let result = part_2(reader).unwrap();
        assert_eq!(result, 4)
    }
}

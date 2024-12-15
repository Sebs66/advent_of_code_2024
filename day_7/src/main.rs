use itertools::Itertools;
use std::env::{self};
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read};

fn main() {
    part_1(String::from("input"));
    part_2(String::from("input"));
}

fn part_1(filename: String) {
    let reader = file_to_buffer(filename);
    let parsed_lines = parse_lines(reader);
    let result = check_lines(parsed_lines, vec!["+", "*"]);
    println!("{}", result);
}

fn part_2(filename: String) {
    let reader = file_to_buffer(filename);
    let parsed_lines = parse_lines(reader);
    let result = check_lines(parsed_lines, vec!["+", "*", "||"]);
    println!("{}", result);
}

fn compute_combinations<'a>(vector: &'a Vec<&str>, iterations: usize) -> Vec<Vec<&'a str>> {
    std::iter::repeat(vector.clone())
        .take(iterations)
        .multi_cartesian_product()
        .collect()
}

fn parse_lines(reader: BufReader<Cursor<String>>) -> Vec<Vec<i64>> {
    let mut parsed_lines: Vec<Vec<i64>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let parsed_numbers: Vec<i64> = line
            .replace(':', " ")
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        parsed_lines.push(parsed_numbers);
    }
    parsed_lines
}

fn file_to_buffer(filename: String) -> BufReader<Cursor<String>> {
    let current_dir = env::current_dir().unwrap();
    println!("Dir: {:?}", current_dir);

    let file_path = current_dir.join(filename);
    let file: Result<File, std::io::Error> = File::open(file_path);
    let mut content = String::new();
    file.unwrap().read_to_string(&mut content).unwrap();

    let cursor = Cursor::new(content);
    let reader = BufReader::new(cursor);
    reader
}

fn check_lines(parsed_equations: Vec<Vec<i64>>, operators: Vec<&str>) -> i64 {
    let mut valid_lines_accumulator = 0;
    for equation in parsed_equations {
        let result = equation[0];
        let numbers = &equation[1..];
        let aritmetic_combinations = compute_combinations(&operators, numbers.len() - 1);
        for combination in aritmetic_combinations {
            let aritmetic_result = numbers.iter().enumerate().fold(0, |acc, (index, &num)| {
                if index == 0 {
                    return num;
                }
                let operation = combination[index - 1];
                match operation {
                    "+" => acc + num,
                    "*" => acc * num,
                    "||" => concatenate(acc, num),
                    _ => panic!("Unknown operator"),
                }
            });
            if aritmetic_result == result {
                valid_lines_accumulator += result;
                break;
            }
        }
    }
    return valid_lines_accumulator;
}

fn concatenate(partial_result: i64, number: i64) -> i64 {
    let concatenation = partial_result.to_string() + &number.to_string();
    concatenation.parse::<i64>().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{check_lines, file_to_buffer, parse_lines};

    #[test]
    fn test_part1() {
        let reader = file_to_buffer(String::from("example"));
        let parsed_lines = parse_lines(reader);
        let result = check_lines(parsed_lines, vec!["+", "*"]);
        assert_eq!(3749, result)
    }
    #[test]
    fn test_part2() {
        let reader = file_to_buffer(String::from("example"));
        let parsed_lines = parse_lines(reader);
        let result = check_lines(parsed_lines, vec!["+", "*", "||"]);
        assert_eq!(11387, result)
    }
}

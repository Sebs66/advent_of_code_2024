use std::fs;

use regex::Regex;

fn main() {
    let file_path = "input";
    let content = fs::read_to_string(file_path).unwrap();
    let result = part1(content);
    println!("result: {}", result);

    let content = fs::read_to_string(file_path).unwrap();
    let result = part2(content);
    println!("result: {}", result);
}

fn part1(corrupted_memory: String) -> i32 {
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(&pattern).unwrap();

    let mut computation = 0;
    for numbers in re.captures_iter(&corrupted_memory) {
        let number1 = &numbers[1].parse::<i32>().unwrap(); // First capture group
        let number2 = &numbers[2].parse::<i32>().unwrap(); // Second capture group

        computation += number1 * number2;
    }
    return computation;
}
fn part2(corrupted_memory: String) -> i32 {
    let pattern = r"(mul\(\d{1,3},\d{1,3}\))|(don't\(\))|(do\(\))";
    let re = Regex::new(&pattern).unwrap();

    let mut computation = 0;
    let mut enabled = true;
    for capture in re.captures_iter(&corrupted_memory) {
        if let Some(mul_match) = capture.get(1) {
            let mul_pattern = r"(\d{1,3}),(\d{1,3})";
            let re_mul = Regex::new(mul_pattern).unwrap();

            if let Some(caps) = re_mul.captures(mul_match.as_str()) {
                let number_1: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
                let number_2: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
                // println!("mul({}, {})", number_1, number_2);
                if enabled {
                    computation += number_1 * number_2
                }
            }
        } else if let Some(dont) = capture.get(2) {
            enabled = false;
        } else if let Some(do_) = capture.get(3) {
            enabled = true
        }
        // println!("{:?}", capture);
    }
    return computation;
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_aoc_example() {
        let corrupted_memory =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        let result = part1(corrupted_memory);

        assert_eq!(result, 161)
    }

    #[test]
    fn test_aoc_example_part2() {
        let corrupted_memory =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        let result = part2(corrupted_memory);

        assert_eq!(result, 48)
    }
}

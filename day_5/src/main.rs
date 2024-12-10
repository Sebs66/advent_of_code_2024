use core::num;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Cursor, Read};
use std::ops::Index;
fn main() {
    let reader = read_file(String::from("input"));
    let (result_1, result_2) = part_1(reader.unwrap());
    println!("{}", result_1);
    println!("{}", result_2);
    // part_1();
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

fn part_1(reader: BufReader<Cursor<String>>) -> (i32, i32) {
    let (pages_rules, pages_to_update) = parse_input(reader);
    let mut sum_of_middle_number_valid_lines = 0;
    let mut sum_of_middle_number_ex_invalid_lines = 0;
    let pages_rules_copy = pages_rules.clone();

    for line in pages_to_update {
        let mut updated_pages: HashMap<i32, bool> = HashMap::new();
        let mut valid = true;
        for &page in &line {
            // println!("{}, rules : {:?}", page, line);
            // let required_pages = pages_rules[&page].clone();
            if let Some(required_pages) = pages_rules.get(&page) {
                for &required_page in required_pages {
                    if line.contains(&required_page)
                        && !updated_pages.get(&required_page).copied().unwrap_or(false)
                    {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    break;
                }
                updated_pages.insert(page, true);
            } else {
                updated_pages.insert(page, true);
            }
        }

        if valid {
            let middle_value = line[(line.len() - 1) / 2];
            println!("Valid line: {:?}, middle value: {}", line, middle_value);
            sum_of_middle_number_valid_lines += middle_value
        }
        if !valid {
            let middle_value_ex_invalid = part_2(line, pages_rules_copy.clone());
            sum_of_middle_number_ex_invalid_lines += middle_value_ex_invalid;
        }
    }
    (
        sum_of_middle_number_valid_lines,
        sum_of_middle_number_ex_invalid_lines,
    )
}

fn part_2(line: Vec<i32>, page_rules: HashMap<i32, Vec<i32>>) -> i32 {
    // for bad lines.
    println!("invalid line: {:?}", line);
    let line_copy = line.clone();

    let mut valid_line: Vec<i32> = vec![];
    loop {
        for index in 0..line_copy.len() {
            let number = line_copy[index];
            if valid_line.contains(&number) {
                continue;
            }
            if let Some(pre_requisites) = page_rules.get(&number) {
                // println!("{:?}", pre_requisites);
                let line_without_number: Vec<i32> =
                    line.iter().filter(|&&x| x != number).cloned().collect();
                let pre_requisites_without_already_present_numbers: Vec<&i32> = pre_requisites
                    .iter()
                    .filter(|x| !valid_line.contains(x))
                    .collect();
                if line_without_number
                    .iter()
                    .any(|x| pre_requisites_without_already_present_numbers.contains(&x))
                {
                    continue;
                } else {
                    valid_line.push(number);
                }
            } else {
                valid_line.push(number);
            }
        }
        if valid_line.len() == line_copy.len() {
            break;
        }
    }
    let middle_value = valid_line[(line.len()) / 2];
    let middle_index = line.len() / 2;
    println!(
        "middle index {}, middle value {}",
        middle_index, middle_value
    );
    println!(
        "ex_invalid line: {:?}, middel value: {}",
        valid_line, middle_value
    );
    middle_value
}

fn parse_input(reader: BufReader<Cursor<String>>) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut limiter = false;

    let mut pages_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut pages_to_update: Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            limiter = true;
            continue;
        }
        if !limiter {
            let pages: Vec<&str> = line.split('|').collect();
            let page_requirement = pages.get(0).unwrap().parse::<i32>().unwrap();
            let page = pages.get(1).unwrap().parse::<i32>().unwrap();
            pages_rules
                .entry(page)
                .or_insert_with(Vec::new)
                .push(page_requirement);
        } else {
            pages_to_update.push(
                line.split(",")
                    .map(|number| number.parse::<i32>().unwrap())
                    .collect(),
            );
        }
    }
    (pages_rules, pages_to_update)
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    use std::io::{BufReader, Cursor};
    #[test]

    fn test_part1() {
        let test_input = String::from(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );

        let cursor = Cursor::new(test_input); // In-memory cursor over the input string
        let reader: BufReader<Cursor<String>> = BufReader::new(cursor);
        let (sum_1, sum_2) = part_1(reader);
        println!("{}", sum_1);
        println!("{}", sum_2);
        assert_eq!(sum_1, 143);
        assert_eq!(sum_2, 123);
    }
}

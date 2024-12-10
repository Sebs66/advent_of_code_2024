use std::io::{self, BufRead, BufReader, Cursor};

pub fn part_2(reader: BufReader<Cursor<String>>) -> io::Result<i32> {
    let mut correct_lines = 0;
    for line in reader.lines() {
        let line = line?;
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        for i in 0..report.len() {
            let inc = report
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, val)| val)
                .collect::<Vec<i32>>()
                .windows(2)
                .all(is_safe_increasing);

            let dec = report
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, val)| val)
                .collect::<Vec<i32>>()
                .windows(2)
                .all(is_safe_decreasing);

            if inc || dec {
                correct_lines += 1;
                break;
            }
        }
    }
    println!("{}", correct_lines);
    Ok(correct_lines)
}

fn is_safe_increasing(pair: &[i32]) -> bool {
    pair[1] > pair[0] && (pair[1] - pair[0] > 0) && (pair[1] - pair[0]) < 4
}
fn is_safe_decreasing(pair: &[i32]) -> bool {
    pair[0] > pair[1] && (pair[0] - pair[1] > 0) && (pair[0] - pair[1]) < 4
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

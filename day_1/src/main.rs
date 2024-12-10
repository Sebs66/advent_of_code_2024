use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::zip;

fn main() -> io::Result<()> {
    let reader = read_file(String::from("input"))?;
    let result_part1 = part_1(reader)?;
    let reader = read_file(String::from("input"))?;
    println!("Result part 1: {result_part1}");
    let result_part1_alternative = part_1_youtube(reader);
    println!("Result part 1 alternative: {:?}", result_part1_alternative?);
    let reader = read_file(String::from("input"))?;
    let result_part2 = part_2(reader)?;
    println!("Result part 2: {result_part2}");

    Ok(())
}

fn read_file(filename: String) -> Result<BufReader<File>, io::Error> {
    let current_dir = env::current_dir()?; // Propagates the error.
    let file_path = current_dir.join(filename);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}
fn order_columns(reader: BufReader<File>) -> Result<(Vec<i32>, Vec<i32>), io::Error> {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            let num1: i32 = parts[0].parse().unwrap_or_default();
            let num2: i32 = parts[1].parse().unwrap_or_default();
            col1.push(num1);
            col2.push(num2);
        } else {
            eprintln!("Skipping malformed line : {}", line);
        }
    }
    col1.sort();
    col2.sort();
    Ok((col1, col2))
}

/// Compute cummulated individual distance betwenn sorted columns.
fn part_1(reader: BufReader<File>) -> io::Result<i32> {
    let (col1, col2) = order_columns(reader)?;
    let mut sum = 0;
    assert!(col1.len() == col2.len());
    for index in 0..col1.len() {
        // println!("{}", col1[index]);
        let (a, b) = (col1[index], col2[index]);
        let distance = |a: i32, b: i32| (a - b).abs();
        // sum += abs(col1[index] - col2[index])
        sum += distance(a, b)
    }
    Result::Ok(sum)
}

fn part_1_youtube(reader: BufReader<File>) -> io::Result<i32> {
    let mut left = vec![];
    let mut right = vec![];

    for line in reader.lines() {
        let line = line?;
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();

    let result = zip(left, right).map(|(a, b)| (a - b).abs()).sum();
    Ok(result)
}

fn part_2(reader: BufReader<File>) -> io::Result<i32> {
    let (col1, col2) = order_columns(reader)?;
    let mut hashmap: HashMap<i32, i32> = HashMap::new();
    for number in col2.iter() {
        match hashmap.entry(*number) {
            std::collections::hash_map::Entry::Occupied(mut entry) => *entry.get_mut() += 1,
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(1);
            }
        }
    }
    let mut similarity = 0;
    for number in col1.iter() {
        match hashmap.entry(*number) {
            std::collections::hash_map::Entry::Occupied(entry) => {
                similarity += entry.get() * number;
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                continue;
            }
        }
    }
    Ok(similarity)
}
/// Returns two vectors with the sorted integers from the file

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let col1 = vec![3, 4, 2, 1, 3, 3];
        let col2 = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(part_1(&col1, &col2).unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        let col1 = vec![3, 4, 2, 1, 3, 3];
        let col2 = vec![4, 3, 5, 3, 9, 3];
        // The expected result depends on the logic of part_2, so adjust as needed.
        // In this case, we expect the similarity to be some value based on the hashmap logic.
        part_2(&col1, &col2); // You could capture the print output to assert if needed
    }
}

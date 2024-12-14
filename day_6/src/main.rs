mod part_1;
mod part_2;
mod utils;
use utils::file_to_buffer;
fn main() {
    let reader = file_to_buffer(String::from("input"));
    // let result = part_1::part_1(reader);
    let result = part_2::part_2(reader);
    println!("{}", result)
}

#[cfg(test)]
mod tests {
    use crate::{file_to_buffer, part_1};

    #[test]
    fn test_part1() {
        let reader = file_to_buffer(String::from("example"));
        let total_distinct_positions = part_1::part_1(reader);
        println!("total_distinct_positions: {}", total_distinct_positions);
        assert_eq!(total_distinct_positions, 41)
    }
}

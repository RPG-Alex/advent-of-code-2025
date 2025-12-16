use std::{fs::File, io::Read};

fn main() {
    let part1 = part_1();
    println!("part 1: {}\n", part1);
    let part2 = part_2();
    println!("part 2: {}", part2);
}

fn part_1() -> i32 {
    let file = File::open("inputs/day2_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    let mut total = 0;

    total
}

fn part_2() -> i32 {
    let file = File::open("inputs/day2_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    let mut total = 0;

    total
}

fn test_data() -> String {
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string()
}
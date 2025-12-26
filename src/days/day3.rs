use std::{fs::File,io::Read};

fn main() {
    let part1 = part_1();
    println!("part 1: {}\n", part1);
    let part2 = part_2();
    println!("part 2: {}", part2);
}

fn part_1() -> i32 {
    let file = File::open("inputs/day3_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    let mut total: i32 = 0;
    for line in content.lines() {
        let batteries: Vec<i32> = line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        let mut f_max = 0;
        let mut s_max = 0;
        let mut i = 0;
        for j in 0..batteries.len() {
            if batteries[j] > f_max && j < batteries.len()-1 {
                f_max = batteries[j];
                i  = j;
            }
        }
        for j in i+1..batteries.len() {
            if batteries[j] > s_max {
                s_max = batteries[j];
            }
        } 
        let mut charged: String = String::new();
        charged.push_str(&f_max.to_string());
        charged.push_str(&s_max.to_string());
        println!("{}{}", f_max, s_max);
        total += charged.parse::<i32>().unwrap();
    }
    total
}

fn part_2() -> i32 {
    let file = File::open("inputs/day3_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    let mut total = 0;

    total
}

fn test_data() -> String {
    r"987654321111111
811111111111119
234234234234278
818181911112111
".to_string()
}
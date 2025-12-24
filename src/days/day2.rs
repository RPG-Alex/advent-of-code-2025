use std::{fs::File, i32, io::Read};

fn main() {
    let part1 = part_1();
    println!("part 1: {}\n", part1);
    let part2 = part_2();
    println!("part 2: {}", part2);
}

fn part_1() -> i64 {
    let file = File::open("inputs/day2_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    let mut total: i64 = 0;
    for entry in content.split(",") {
        let mut id = entry.split("-");
        let mut min: i64 = id.next().unwrap().parse().unwrap();
        let max: i64 = id.next().unwrap().trim().parse().unwrap();

        while min <= max {
            let num = min.to_string();
            if num.len() % 2 == 0 {
                let (begin,end) = num.split_at(num.len()/2);
                let (mut b_match, mut e_match) = (String::new(), String::new());
                for (i, c) in begin.chars().enumerate() {
                    if let Some(end_c) = end.chars().nth(i) {
                        if c == end_c {
                            b_match.push(c);
                            e_match.push(end_c);
                        } else {
                            b_match.clear();
                            e_match.clear();
                            break;
                        }
                    }
                }
                if b_match == e_match && !b_match.is_empty() && !e_match.is_empty(){
                    total += min;
                }
            }
            min += 1;
        }
    }


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
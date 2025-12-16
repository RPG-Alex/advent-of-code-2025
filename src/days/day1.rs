use std::{fs::File, io::Read};

fn main() {
    let part1 = part_1();
    println!("part 1: {}\n", part1);
    let part2 = part_2();
    println!("part 2: {}", part2);
}

fn part_1() -> i32 {
    let file = File::open("inputs/day1_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    // let content = test_data();
    let mut total = 0;
    let mut location = 50;
    for line in content.lines() {
        let mut direction:char = 'R';
        let mut turn = String::new();
        for (i,c) in line.chars().enumerate() {
            if i == 0 {
                direction = c;
            } else {
                turn.push(c);
            }
        }
        let turn: i32 = turn.parse().unwrap();
        match direction {
            'R' => {
                location += turn; 
                while location > 99 {
                    location = location - 100;
                }
            },
            _ => {
                location -= turn;
                while location < 0 {
                    location = location + 100;
                }
            },
        }
        if location == 0 { total += 1;}
        println!("{location}")
    }
    total
}

fn part_2() -> i32 {
    let file = File::open("inputs/day1_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    let mut total = 0;

    total
}

fn test_data() -> String {
    r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#.to_string()
}
use std::{fs::File,io::Read};

fn main() {
    let part1 = part_1();
    println!("part 1: {}\n", part1);
    let part2 = part_2();
    println!("part 2: {}", part2);
}

fn part_1() -> i32 {
    let file = File::open("inputs/day4_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    //let test = test_data();
    let grid: Vec<Vec<char>> = content.lines().into_iter().map(|f| f.chars().collect()).collect();
    let mut possible = grid.clone();

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == '@' {
                let mut rolls = 0;
                // to the left
                if y > 0  {
                    if grid[x][y-1] == '@' {
                    rolls += 1; 
                    }
                    if x > 0 && grid[x-1][y-1] == '@' {
                        rolls +=1;
                    }
                    if x < grid.len() - 1 && grid[x+1][y-1] == '@' {
                        rolls +=1;
                    }
                }
                // middle top/bottom
                if x > 0 && grid[x-1][y] == '@' {
                    rolls += 1;
                }
                if x < grid.len() - 1 && grid[x+1][y] == '@'{
                    rolls +=1;
                }
                // to the right
                if y < grid[x].len() - 1 {
                    if x > 0 && grid[x-1][y+1] == '@' {
                        rolls +=1;
                    }
                    if grid[x][y+1] == '@' {
                        rolls +=1;
                    }
                    if x < grid.len() - 1 && grid[x+1][y+1] == '@'{
                        rolls += 1;
                    }
                }
                if rolls < 4 {
                    possible[x][y] = 'x';
                }
            }
        }
    }
    possible.iter().flatten().filter(|x| **x == 'x').count() as i32


}

fn part_2() -> i32 {
    let file = File::open("inputs/day4_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    let mut total = 0;

    total
}

fn test_data() -> String {
    r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
".to_string()
}
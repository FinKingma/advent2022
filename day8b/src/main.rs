use std::fs;
// use regex::Regex;
// use substring::Substring;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.lines().collect::<Vec<&str>>();
    const NR_LINES: usize = 99;
    const LINE_LENGTH: usize = 99;
    // const NR_LINES: usize = 5;
    // const LINE_LENGTH: usize = 5;
    let mut state = [[0u8; LINE_LENGTH]; NR_LINES];

    for i in 0..NR_LINES {
        let test: Vec<char> = lines[i as usize].chars().collect();
        for j in 0..LINE_LENGTH {
            state[i][j] = (test[j as usize].to_string()).parse::<u8>().unwrap();
        }
    }

    let mut best_score = 0;

    for i in 0..NR_LINES {
        for j in 0..LINE_LENGTH {
            if i == 0 || i == NR_LINES-1 || j == 0 || j == LINE_LENGTH-1 {
                // println!("tree {} at border", state[i][j]);
            } else {
                let current = &state[i][j];
                // look left
                let mut left_score = 1;
                for x in (1..j).rev() {
                    if state[i][x] < *current {
                        left_score+=1;
                    } else {
                        break;
                    }
                }
                // look right
                let mut right_score = 1;
                for x in j+1..LINE_LENGTH-1 {
                    if state[i][x] < *current {
                        right_score+=1
                    } else {
                        break;
                    }
                }
                // look up
                let mut up_score = 1;
                for x in (1..i).rev() {
                    if state[x][j] < *current {
                        up_score+=1;
                    } else {
                        break;
                    }
                }
                // look down
                let mut down_score = 1;
                for x in i+1..NR_LINES-1 {
                    if state[x][j] < *current {
                        down_score+=1
                    } else {
                        break;
                    }
                }
                let total = left_score * right_score * up_score * down_score;
                if total > best_score {
                    best_score = total;
                }
                println!("score {}", best_score);
            }
        }
    }
}
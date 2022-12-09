use std::fs;
// use regex::Regex;
// use substring::Substring;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.lines().collect::<Vec<&str>>();
    const NR_LINES: usize = 99;
    const LINE_LENGTH: usize = 99;
    let mut state = [[0u8; LINE_LENGTH]; NR_LINES];

    for i in 0..NR_LINES {
        let test: Vec<char> = lines[i as usize].chars().collect();
        for j in 0..LINE_LENGTH {
            state[i][j] = (test[j as usize].to_string()).parse::<u8>().unwrap();
        }
    }

    let mut score = 0;
    for i in 0..NR_LINES {
        for j in 0..LINE_LENGTH {
            if i == 0 || i == NR_LINES-1 || j == 0 || j == LINE_LENGTH-1 {
                println!("tree {} at border", state[i][j]);
                score+=1;
            } else {
                // highest tree to left
                let left = &state[i][0..j].iter().max().unwrap();
                let right = &state[i][j+1..LINE_LENGTH].iter().max().unwrap();
                let top = &state[0..i].iter().map(|f| f[j]).max().unwrap();
                let bottom = &state[i+1..NR_LINES].iter().map(|f| f[j]).max().unwrap();
                let current = &state[i][j];
                if current > left || current > right || current > top || current > bottom {
                    println!("tree {}", current);
                    score+=1;
                }
            }
        }
    }
    println!("score {}", score);
}
use std::fs;
use regex::Regex;
// use substring::Substring;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut cycles = Vec::new();
    let mut current_number = 1;

    for line in lines {
        if line == "noop" {
            cycles.push(current_number);
        } else {
            let re = Regex::new(r"addx (-?\d*)").unwrap();
            let caps = re.captures(line).unwrap();
            let change: i32 = caps[1].parse().unwrap();
            cycles.push(current_number);
            cycles.push(current_number);
            current_number += change;
        }
    }
    let mut score = 0;
    for i in 1..7 {
        let cycle_selector = (i * 40) - 20;
        let cycle_value = cycles[(cycle_selector-1) as usize] * cycle_selector;
        score += cycle_value;
        println!("cycle {}: {}", cycle_selector, cycle_value);
    }
    println!("score {}", score);
    // println!("test {:?}", cycles);
}
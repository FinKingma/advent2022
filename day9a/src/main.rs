use std::fs;
use regex::Regex;
// use substring::Substring;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut head_position_left:i32=1000;
    let mut head_position_top:i32=1000;
    let mut tail_position_left:i32=1000;
    let mut tail_position_top:i32=1000;

    let mut tail_positions = Vec::new();
    tail_positions.push("1000:1000".to_string());

    let mut score = 1;

    for line in lines {
        let re = Regex::new(r"([A-Z]) (\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        let direction: &str = &caps[1];
        let steps: i32 = caps[2].parse().unwrap();
        println!("test {} {}", direction,steps);
        for i in 0..steps {
            if direction == "L" {
                head_position_left -= 1;
            } else if direction == "R" {
                head_position_left += 1;
            } else if direction == "U" {
                head_position_top -= 1;
            } else if direction == "D" {
                head_position_top += 1;
            }
            println!("position top {} left {}", head_position_top, head_position_left);

            if (head_position_left - tail_position_left).abs() > 1 || (head_position_top - tail_position_top).abs() > 1 {
                // move tail
                if head_position_left > tail_position_left {
                    tail_position_left += 1;
                } else if head_position_left < tail_position_left {
                    tail_position_left -= 1;
                }

                if head_position_top > tail_position_top {
                    tail_position_top += 1;
                } else if head_position_top < tail_position_top {
                    tail_position_top -= 1;
                }
            }
            let position_string = format!("{}:{}", tail_position_top, tail_position_left);
            // println!("tail position {}", position_string);
            if !tail_positions.contains(&position_string) {
                println!("new position! {}", position_string);
                tail_positions.push(position_string);
                score += 1;
            }
            
        }
    }
    println!("final score {}", score)
}
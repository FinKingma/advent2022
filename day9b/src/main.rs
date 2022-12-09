use std::fs;
use regex::Regex;
// use substring::Substring;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut head_position_left:i32=1000;
    let mut head_position_top:i32=1000;
    let mut tail_positions_left= vec![1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000];
    let mut tail_positions_top= vec![1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000];

    println!("test {:?}", tail_positions_left);

    let mut tail_positions = Vec::new();
    tail_positions.push("1000:1000".to_string());

    let mut canvas = [[0u8; 100]; 100];

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
            // println!("position top {} left {}", head_position_top, head_position_left);

            if (head_position_left - tail_positions_left[0]).abs() > 1 || (head_position_top - tail_positions_top[0]).abs() > 1 {
                // move tail
                if head_position_left > tail_positions_left[0] {
                    tail_positions_left[0] += 1;
                } else if head_position_left < tail_positions_left[0] {
                    tail_positions_left[0] -= 1;
                }

                if head_position_top > tail_positions_top[0] {
                    tail_positions_top[0] += 1;
                } else if head_position_top < tail_positions_top[0] {
                    tail_positions_top[0] -= 1;
                }
            }

            for t in 0..9 {
                if (tail_positions_left[t] - tail_positions_left[t+1]).abs() > 1 || (tail_positions_top[t] - tail_positions_top[t+1]).abs() > 1 {
                    // move tail
                    if tail_positions_left[t] > tail_positions_left[t+1] {
                        tail_positions_left[t+1] += 1;
                    } else if tail_positions_left[t] < tail_positions_left[t+1] {
                        tail_positions_left[t+1] -= 1;
                    }
    
                    if tail_positions_top[t] > tail_positions_top[t+1] {
                        tail_positions_top[t+1] += 1;
                    } else if tail_positions_top[t] < tail_positions_top[t+1] {
                        tail_positions_top[t+1] -= 1;
                    }
                }
            }

            for a in 0..100 {
                for b in 0..100 {
                    canvas[a][b] = 0
                }
            }
            // canvas[(head_position_left-950) as usize][(head_position_top-950) as usize] = 9;
            // for t in 0..9 {
            //     canvas[(tail_positions_left[t]-950) as usize][(tail_positions_top[t]-950) as usize] = t
            // }

            // println!("---------");
            // for line in canvas {
            //     let s = line.iter().map(|s| s.to_string()).collect::<String>();
            //     println!("{:?}", s);
            // }
            // println!("---------");

            // for t in 0..9 {
            //     let position_test_string = format!("{}:{}", tail_positions_top[t], tail_positions_left[t]);
            //     println!("position of tail {} is {}", t, position_test_string);
            // }
            

            let position_string = format!("{}:{}", tail_positions_top[8], tail_positions_left[8]);
            // println!("tail position {}", position_string);
            if !tail_positions.contains(&position_string) {
                // println!("new position! {}", position_string);
                tail_positions.push(position_string);
                score += 1;
            }
            
        }
    }
    println!("tail positions {:?}", tail_positions);
    println!("final score {}", score);
}
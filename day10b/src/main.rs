use std::fs;
use regex::Regex;
// use substring::Substring;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.lines().collect::<Vec<&str>>();

    // let mut cycles = Vec::new();
    let mut current_number: i32 = 1;
    let mut canvas = [[0u8; 40]; 6];
    for i in 0..6 {
        for j in 0..40 {
            canvas[i][j] = 0
        }
    }


    let mut sprite_position: usize = 1;
    let mut current_position: usize = 0;
    let mut current_row:usize = 0;
    for line in lines {
        if line == "noop" {
            // cycles.push(current_number);
            if current_position >= 40 {
                current_position = 0;
                current_row += 1;
            }
            let mut canvas_value;
            if ((current_position as i32) - (sprite_position as i32)).abs() < 2 {
                // draw sprite
                canvas_value = 1;
            } else {
                canvas_value = 0;
            }
            canvas[current_row][current_position] = canvas_value;
            current_position += 1;
        } else {
            let re = Regex::new(r"addx (-?\d*)").unwrap();
            let caps = re.captures(line).unwrap();
            let change: i32 = caps[1].parse().unwrap();

            if current_position >= 40 {
                current_position = 0;
                current_row += 1;
            }
            println!("test {} {}", current_position, current_row);
            let mut canvas_value;
            if ((current_position as i32) - (sprite_position as i32)).abs() < 2 {
                // draw sprite
                canvas_value = 1;
            } else {
                canvas_value = 0;
            }
            canvas[current_row][current_position] = canvas_value;
            current_position += 1;


            if current_position >= 40 {
                current_position = 0;
                current_row += 1;
            }
            let mut canvas_value;
            if ((current_position as i32) - (sprite_position as i32)).abs() < 2 {
                // draw sprite
                canvas_value = 1;
            } else {
                canvas_value = 0;
            }
            canvas[current_row][current_position] = canvas_value;
            current_position += 1;

            // cycles.push(current_number);
            // cycles.push(current_number);
            current_number += change;
            sprite_position = current_number as usize;
            // println!("finished {}, sprite is now at {} checking at {}", change, sprite_position, current_position)

        }
    }

    
    // for c in 0..40 {
    //     let cycle_value = cycles[c];
    //     let cycle_num = c + 1;
    //     println!("cycle {} register {}, sprite {}", cycle_num, cycle_value, sprite_position);
    //     if (sprite_position - (cycle_num as i32)).abs() < 2 {
    //         canvas[0][c] = 1;
    //     } else {
    //         canvas[0][c] = 0;
    //     }
    //     sprite_position = cycles[c];
        
    // }

    for i in 0..6 {
        println!("{:?}", canvas[i].iter().map(|s| s.to_string().replace("0", ".").replace("1", "#")).collect::<String>());
    }

    // BACEKLHF
}
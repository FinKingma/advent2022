use std::fs;
// use regex::Regex;
// use substring::Substring;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.lines();
    let goal:i32 = 528671;
    let mut diff:i32 = 1000000;

    // directory of 528671 needs to be deleted
    for line in lines {
        if line == "" {
            // println!("wrong");
        } else {
            let number = line.parse::<i32>().unwrap();
            if number > goal {
                let curr_diff = number - goal;
                if curr_diff < diff {
                    diff = curr_diff;
                    println!("test {}", line);
                }
                
            }
        }        
    }
    // println!("total {}", total);
}

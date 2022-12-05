use std::fs;
use regex::Regex;
use substring::Substring;

fn main() {
    // let mut array: [&str; 10] = unsafe {
    //     mem::uninitialized()
    // };
    // let mut array = vec!["".to_string()];
    #![allow(unused)]
    let mut array = Vec::new();

    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut score = 0;
    let mut header = true;
    let mut first_line = true;

    for line in lines {
        if line.starts_with(" 1") {
            println!("forming groups");
            continue;
        } else if line == "" {
            println!("End of header found");
            header = false;
            continue;
        }

        // put letters appropriately in vector
        if header {
            let line2 = &line.replace("    ", "[]");
            let line3 = &line2.replace(" ", "");
            let groups: Vec<&str> = line3.split("]").collect();
            let length = groups.len()-1;
            if first_line {
                for i in 0..length {
                    let val = groups[i].replace("[","");
                    array.push(val);
                }
                first_line = false
            } else {
                for i in 0..length {
                    let val = groups[i].replace("[","");
                    let mut old_val: String = array[i].to_owned();
                    array[i] = old_val + &val;
                }
            }
            
            println!("letters setup {:?}", array);
        } else {
            // now we should have something like ["NZ", "DCM", "P"]
            let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            let caps = re.captures(line).unwrap();
            let amount: i32 = caps[1].parse().unwrap();
            let pull: i32 = caps[2].parse().unwrap();
            let push: i32 = caps[3].parse().unwrap();

            let mut old_val: String = array[(pull-1) as usize].to_owned();
            let mut length = old_val.chars().count();
            let mut slice = old_val.substring(0, amount.try_into().unwrap()).to_owned();
            let mut remainder = old_val.substring(amount.try_into().unwrap(), length);
            array[(pull-1) as usize] = remainder.to_string();
            let mut new_old_val: String = array[(push-1) as usize].to_owned();
            array[(push-1) as usize] = slice.to_owned() + &new_old_val;
            println!("move {} from {} to {}: take {} size {} remainder {}", amount, pull, push, old_val, length, remainder);
            println!("step {:?}", array);
        }
    }
    println!("With text: {:?}", array);
    for arr in array {
        println!("{}", arr.substring(0,1));
    }
    
    
    
}

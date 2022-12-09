use std::fs;
use regex::Regex;
use substring::Substring;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.lines();

    // find occurances that start with cd ([az]*).*$ ls and contain only files with a total size of < 100000
    // regex: \$ cd ([a-z])*\n\$ ls(\n[a-z0-9\s.]*)*\$

    let mut total = 0;
    // find files that are less than 100000
    for line in lines {
        // let values = line.split(" ");
        if (line.substring(0,1) == "$") {
            // println!("wrong");
        } else if(line.substring(0,3) == "dir") {
            // println!("test {}", line);
        } else {
            let re = Regex::new(r"([0-9]*) ([a-z\.]*)").unwrap();
            let caps = re.captures(line).unwrap();
            let opponent = &caps[1].parse::<i32>().unwrap();;
            total += opponent;
            println!("test {}", line);
        }
        // }
        // directory of 528671 needs to be deleted
    }
    println!("total {}", total);
    
}

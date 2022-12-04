use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut score = 0;

    for line in lines {
        let re = Regex::new(r"(\d+)\-(\d+),(\d+)-(\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        let elf1_start: i32 = caps[1].parse().unwrap();
        let elf1_end: i32 = caps[2].parse().unwrap();
        let elf2_start: i32 = caps[3].parse().unwrap();
        let elf2_end: i32 = caps[4].parse().unwrap();

        if elf2_start >= elf1_start && elf2_start <= elf1_end {
            println!("hit: {} {} {} {}", elf1_start, elf1_end, elf2_start, elf2_end, );
            score += 1;
        } else if elf2_end >= elf1_start && elf2_end <= elf1_end {
            println!("hit2: {} {} {} {}", elf1_start, elf1_end, elf2_start, elf2_end, );
            score += 1;
        } else if elf1_start >= elf2_start && elf1_start <= elf2_end {
            println!("hit: {} {} {} {}", elf1_start, elf1_end, elf2_start, elf2_end, );
            score += 1;
        } else if elf1_end >= elf2_start && elf1_end <= elf2_end {
            println!("hit2: {} {} {} {}", elf1_start, elf1_end, elf2_start, elf2_end, );
            score += 1;
        }
    }
    println!("With text: {}", score);
    
    
}

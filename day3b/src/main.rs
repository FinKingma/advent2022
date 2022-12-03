use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let length = lines.len();
    let groups = length / 3;
    let mut score = 0;

    for i in 0..groups {
        let elf1 = lines[(i*3)];
        let elf2 = lines[(i*3) + 1];
        let elf3 = lines[(i*3) + 2];
        let item = get_item_in_compartments(elf1, elf2, elf3);
        let value = get_value_from_item(item);
        score = score + value;
        
    }
    
    println!("With text: {}", score);
}

fn get_item_in_compartments(first: &str, second: &str, third: &str) -> char {
    for x in first.chars() {
        for y in second.chars() {
            for z in third.chars() {
                if &x == &y && &y == &z {
                    return x
                }
            }
        }
    }
    panic!("crash and burn");
}

fn get_value_from_item(item: char) -> i32 {
    let mut value:i32 = item as i32;
    if item.is_uppercase() {
        value = value - 38
    } else {
        value = value - 96
    }
    return value
}
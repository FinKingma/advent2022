use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut score = 0;
    for line in lines {
        let length = line.chars().count();
        let first_comp = &line[..length/2];
        let second_comp = &line[(length/2)..length];

        let item_in_both = get_item_in_both_compartments(first_comp, second_comp);
        let value = get_value_from_item(item_in_both);
        score = score + value;
    }
    
    println!("With text: {}", score);
}

fn get_item_in_both_compartments(first_comp: &str, second_comp: &str) -> char {
    for x in first_comp.chars() {
        for y in second_comp.chars() {
            if &x == &y {
                return x
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
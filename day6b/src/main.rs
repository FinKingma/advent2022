use std::fs;
// use regex::Regex;
use substring::Substring;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let chars = contents.chars();
    let distinct_chars = 14;

    for i in 0..(chars.count()-distinct_chars) {
        let contents = &contents.substring(i, i+distinct_chars);
        match unique(contents) {
            None => {
                println!("{} is unique after character {}", contents, i+distinct_chars);
                return;
            },
            Some((i, j, c)) => println!(
                " is not unique\n\tfirst duplicate: \"{}\" (U+{:0>4X}) at indices {} and {}",
                c, c as usize, i, j
            ),
        }
    }
    
    
}

fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}
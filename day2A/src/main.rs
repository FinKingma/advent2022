use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut score = 0;
    for line in lines {
        let re = Regex::new(r"([A|B|C]) ([Y|X|Z])").unwrap();
        let caps = re.captures(line).unwrap();
        let opponent = &caps[1];
        let suggestion = &caps[2];

        let result = decide_result_score(opponent, suggestion);
        let bonus = decide_suggestion_score(suggestion);

        score = score + result;
        score = score + bonus;
    }
    println!("With text: {}", score);
}

fn decide_result_score(opponent: &str, suggestion: &str) -> i32 {
    if opponent.eq("A") {
        if suggestion.eq("X") {
            3
        } else if suggestion.eq("Y") {
            6
        } else {
            0
        }
    } else if opponent.eq("B") {
        if suggestion.eq("X") {
            0
        } else if suggestion.eq("Y") {
            3
        } else {
            6
        }
    } else {
        if suggestion.eq("X") {
            6
        } else if suggestion.eq("Y") {
            0
        } else {
            3
        }
    }
}

fn decide_suggestion_score(suggestion: &str) -> i32 {
    if suggestion.eq("X") {
        1
    } else if suggestion.eq("Y") {
        2
    } else {
        3
    }
}
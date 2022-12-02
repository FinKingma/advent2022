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

        score = score + result;
    }
    println!("With text: {}", score);
}

fn decide_result_score(opponent: &str, suggestion: &str) -> i32 {
    if opponent.eq("A") { // rock
        if suggestion.eq("X") { // lose
            // use scissor
            3
        } else if suggestion.eq("Y") { // draw
            // use rock
            3 + 1
        } else { // win
            // use paper
            6 + 2
        }
    } else if opponent.eq("B") { // paper
        if suggestion.eq("X") { // lose
            // use rock
            1
        } else if suggestion.eq("Y") { // draw
            // use paper
            3 + 2
        } else { // win
            // use scissor
            6 + 3
        }
    } else { // scissor
        if suggestion.eq("X") { // lose
            // use paper
            2
        } else if suggestion.eq("Y") { // draw
            // use scissor
            3 + 3
        } else { // win
            // use rock
            6 + 1
        }
    }
}

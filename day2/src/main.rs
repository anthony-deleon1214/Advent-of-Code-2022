// Importing filesystem from standard library
use std::fs;

// Creating enums for opponent plays, my plays, and outcomes
enum Outcome {
    Loss,
    Draw,
    Win,
}
fn main() {
    let input: String = fs::read_to_string("src/input.txt")
        .expect("input.txt file is required");

    let mut score: u32 = 0;
    let mut lines = input.lines();

    // Loop through all lines of the input
    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break
        };

        let line = line.trim();
        score += selection_score(&line);

        let round_score = match check_outcome(&line) {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };

        score += round_score;
    };

    println!("The final score is {}", score)
}

// Takes a line as a string slice and returns the proper selection score
fn selection_score(line: &str) -> u32 {
    if line.ends_with("X") {
        return 1
    } else if line.ends_with("Y") {
        return 2
    } else { // Only X, Y, or Z should be possible options given the input
        return 3
    }
}

// 
fn check_outcome(line: &str) -> Outcome {
    match line {
        "A X" => Outcome::Draw,
        "A Y" => Outcome::Win,
        "A Z" => Outcome::Loss,
        "B X" => Outcome::Loss,
        "B Y" => Outcome::Draw,
        "B Z" => Outcome::Win,
        "C X" => Outcome::Win,
        "C Y" => Outcome::Loss,
        "C Z" => Outcome::Draw,
        _ => panic!("Impossible round found"),
    }
}
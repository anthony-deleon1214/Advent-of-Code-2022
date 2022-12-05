// Importing filesystem from standard library
use std::fs;

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
    
    loop {
        let mut line = match lines.next() {
            Some(line) => line,
            None => break,
        };

        line = line.trim();
        let outcome = get_outcome(line);

        score += match outcome {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };
        
        score += selection_score(line, outcome);
    }

    println!("The total score is {}", score)
}

// Takes a line from the puzzle input and determines player output and score
fn selection_score(line: &str, outcome: Outcome) -> u32 {
    let selection_score: u32;
    if line.starts_with("A") {
        selection_score = match outcome {
            Outcome::Loss => 3,
            Outcome::Draw => 1,
            Outcome::Win => 2,
        }
    } else if line.starts_with("B") {
        selection_score = match outcome {
            Outcome::Loss => 1,
            Outcome::Draw => 2,
            Outcome::Win => 3,
        }
    } else if line.starts_with("C") {
        selection_score = match outcome {
            Outcome::Loss => 2,
            Outcome::Draw => 3,
            Outcome::Win => 1,
        }
    } else {
        panic!("Invalid opponent selection")
    };

    selection_score
}

fn get_outcome(line: &str) -> Outcome {
    if line.ends_with("X") {
        return Outcome::Loss
    } else if line.ends_with("Y") {
        return Outcome::Draw
    } else if line.ends_with("Z") {
        return Outcome::Win
    } else {
        panic!("Impossible line ending found")
    }
}
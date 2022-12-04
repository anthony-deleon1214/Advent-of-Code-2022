// Importing filesystem from standard library
use std::fs;

// Creating enums for opponent plays, my plays, and outcomes
enum Opponent {
    A(String),
    B(String),
    C(String),
}

enum Player {
    X(String),
    Y(String),
    Z(String),
}
enum Outcome {
    Loss,
    Draw,
    Win,
}
fn main() {
    let input: String = fs::read_to_string("src/input.txt")
        .expect("input.txt file is required");

    
}

fn selection_score(selection: Player<&str>)

fn check_outcome(line: &str) -> u32 {

}
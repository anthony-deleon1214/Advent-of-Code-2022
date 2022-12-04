// Importing filesystem from standard library
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
        let line = match lines.next() {
            Some(line) => line,
            None => break,
        };

    }
}

// Takes a line from the puzzle input and determines player output and score
fn selection_score(line: &str) -> u32 {

}
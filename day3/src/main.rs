// Importing filesystem from standard library
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt")
        .expect("Requires input.txt file");

    let mut lines = input.lines();

    loop {
        let mut line = match lines.next() {
            Some(line) => line,
            None => break,
        };

        if !line.is_ascii() {
            panic!("Line contains non-ASCII characters")
        }

        line = line.trim();
        let line_length = line.len();
        let comp_length = line_length/2;
        let first_half = match line.get(..comp_length) {
            Some(line) => line,
            None => panic!("First half of line not found")
        };
        let second_half = match line.get(comp_length+1..) {
            Some(line) => line,
            None => panic!("Second half of line not found")
        };

        compare_halves(first_half, second_half);
    }
}

fn compare_halves(first_half: &str, second_half: &str) {
    
}
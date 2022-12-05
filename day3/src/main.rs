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
        let first_half = &line[..comp_length];
        let second_half = &line[comp_length+1..];
    }
}

fn create_bit_representation(compartment: &str) -> u64 {
    // Creates a binary value based on the priority of the characters in the string slice
    let mut str_iter = compartment.chars();
    let bin_rep: u64 = 0;
}
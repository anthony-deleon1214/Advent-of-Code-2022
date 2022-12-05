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

        compare_halves(first_half, second_half);
    }
}

fn compare_halves(first_half: &str, second_half: &str) {
    // Bitwise AND to determine what values are shared between halves
    let first_half_bytes = first_half.as_bytes();
    let second_half_bytes = second_half.as_bytes();

    let mut first_byte = first_half_bytes.iter();
    let mut second_byte = second_half_bytes.iter();
    let mut shared_values = Vec::new();

    loop {
        let first_result = match first_byte.next() {
            Some(byte) => byte,
            None => break,
        };
        let second_result = match second_byte.next() {
            Some(byte) => byte,
            None => break,
        };

        shared_values.push(first_result & second_result)
    }
}
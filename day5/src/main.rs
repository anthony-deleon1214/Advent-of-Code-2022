// Importing filesystem from standard library
use std::fs;

fn main() {
    part1()
}

fn part1() {
    let input = fs::read_to_string("src/test.txt")
        .expect("Requires input.txt file");

    let mut lines = input.lines();
    
}
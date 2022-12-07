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

fn load_crates(line: &str, mut stack_vec: Vec<Vec<&str>>) {
    let mut char_count = 0;

    let mut char_iter = line.chars();

    loop {
        let char = match char_iter.next() {
            Some(char) => char,
            None => break
        };
        
        char_count += 1;
        let target_vec = char_count/4;
        
    }
}
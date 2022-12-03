// Importing File struct from standard library
use std::fs::File;

fn main() {
    let puzzle_input = File::open("puzzleInput.txt").expect("Requires puzzleInput.txt file");

    let mut cal_vec: Vec<u32> = Vec::new();
}

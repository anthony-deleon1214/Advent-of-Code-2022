// Importing File struct from standard library
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let mut calorie_list: Vec<i32> = Vec::new();
}

fn readFile() -> Result<String, io::Error> {
    let puzzle_input: File = File::open("puzzleInput.txt").expect("Requires puzzleInput.txt file");


}
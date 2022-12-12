// Importing filesystem from standard library
use std::fs;

// Defining an enum for input lines
enum TerminalOutput<'a> {
    Command(&'a str),
    Directory(&'a str),
    Size(u32),
}

fn main() {
    part1()
}

fn part1() {
    let input = fs::read_to_string("src/test.txt")
        .expect("Requires input.txt file");

    let mut lines = input.lines();
    let mut cwd = "/";
    
    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break,
        };
        let parsed_line = match parseOutput(line) {
            TerminalOutput::Command => 
        }
    }
}

fn parseOutput(line: &str) -> TerminalOutput {
    if line.starts_with('&') {
        let parsed_command = line.strip_prefix('$').unwrap();
        return TerminalOutput::Command(parsed_command)
    } else if line.starts_with("dir") {
        let directory_name = line.strip_prefix("dir").unwrap();
        return TerminalOutput::Directory(directory_name);
    }
    let file_size:u32 = line.split(' ').next().unwrap().parse().expect("First part of line is not numeric");
    TerminalOutput::Size(file_size)
}
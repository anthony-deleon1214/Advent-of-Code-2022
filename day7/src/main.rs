// Importing filesystem from standard library
use std::fs;
use std::collections::HashMap;

// Defining an enum for input lines
enum TerminalOutput<'a> {
    Command(&'a str),
    Directory(&'a str),
    Size(u32),
}

// Creating a Directory structure
struct Directory {
    
}

fn main() {
    part1()
}

fn part1() {
    let input = fs::read_to_string("src/test.txt")
        .expect("Requires input.txt file");

    let mut lines = input.lines();
    let mut parent_dir = "/";
    let mut cwd = "/";
    let mut size_map: HashMap<&str, u32> = HashMap::new();
    
    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break,
        };
        let parsed_line = match parseOutput(line) {
            TerminalOutput::Command(parsed) => {
                if parsed.starts_with("cd") {
                    parsed.strip_prefix("cd").unwrap();
                    if parsed.starts_with("..") {

                    }
                } 
                continue
            }
            TerminalOutput::Directory(parsed) => continue,
            TerminalOutput::Size(size) => {
                let dir_size = size_map.entry(cwd).or_insert(0);
                *dir_size += size;
            }
        };
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
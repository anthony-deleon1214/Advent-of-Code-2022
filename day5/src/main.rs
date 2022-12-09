// Importing filesystem from standard library
use std::fs;

fn main() {
    part2()
}

fn part1() {
    let input = fs::read_to_string("src/input.txt")
        .expect("Requires input.txt file");

    let mut lines = input.lines();
    let mut stack_vec: Vec<Vec<char>> = Vec::new();
    let mut line = match lines.next() {
        Some(line) => line,
        None => panic!("First line not found")
    };

    // Populating the stack vector with empty vectors
    let line_length: i32 = line.len().try_into().unwrap();
    let mut count: i32 = line_length/4;
    while count >= 0 {
        stack_vec.push(Vec::new());
        count -= 1;
    };
    
    // Loads crates into the stacks from top to bottom
    loop {
        load_crates(line, &mut stack_vec);
        // Currently this is iterating through the entire document
        println!("{}", line);
        line = match lines.next() {
            Some(line) => line,
            None => break,
        };
        if line.is_empty() {
            break
        }
    };

    // Reversing the stacks
    for vector in stack_vec.iter_mut() {
        vector.reverse();
    };

    // Perform instructions to move crates
    loop {
        line = match lines.next() {
            Some(line) => line,
            None => break,
        };
        move_single_crates(line, &mut stack_vec);
    }

    for stack in &mut stack_vec {
        let top_value = match stack.pop() {
            Some(value) => value,
            None => ' ',
        };
        println!("Top of stack: {}", top_value)
    }
}

fn part2() {
    let input = fs::read_to_string("src/input.txt")
        .expect("Requires input.txt file");

    let mut lines = input.lines();
    let mut stack_vec: Vec<Vec<char>> = Vec::new();
    let mut line = match lines.next() {
        Some(line) => line,
        None => panic!("First line not found")
    };

    // Populating the stack vector with empty vectors
    let line_length: i32 = line.len().try_into().unwrap();
    let mut count: i32 = line_length/4;
    while count >= 0 {
        stack_vec.push(Vec::new());
        count -= 1;
    };
    
    // Loads crates into the stacks from top to bottom
    loop {
        load_crates(line, &mut stack_vec);
        // Currently this is iterating through the entire document
        println!("{}", line);
        line = match lines.next() {
            Some(line) => line,
            None => break,
        };
        if line.is_empty() {
            break
        }
    };

    // Reversing the stacks
    for vector in stack_vec.iter_mut() {
        vector.reverse();
    };

    // Perform instructions to move crates
    loop {
        line = match lines.next() {
            Some(line) => line,
            None => break,
        };
        stack_vec = move_multiple_crates(line, stack_vec);
    }

    for stack in &mut stack_vec {
        let top_value = match stack.pop() {
            Some(value) => value,
            None => ' ',
        };
        println!("Top of stack: {}", top_value)
    }
}

fn load_crates(line: &str, stack_vec: &mut Vec<Vec<char>>) {
    let mut char_count = 0;

    let mut char_iter = line.chars();

    loop {
        let char = match char_iter.next() {
            Some(char) => char,
            None => break
        };
        
        char_count += 1;
        if !char.is_ascii_uppercase() {
            continue
        }

        // Calculate index of target stack
        let target_index = char_count/4;
        stack_vec[target_index].push(char);
    }
}

fn move_single_crates(line: &str, stack_vec: &mut Vec<Vec<char>>) {
    let digit_arr = parse_digits(line);
    let mut count = digit_arr[0];
    let target_stack = digit_arr[2]-1;
    let supply_stack = digit_arr[1]-1;
    while count > 0 {
        let moved_crate = match stack_vec[supply_stack].pop() {
            Some(value) => value,
            None => panic!("Supplying stack is empty")
        };
        stack_vec[target_stack].push(moved_crate);
        count -= 1;
    }
}

fn parse_digits(line: &str) -> [usize; 3] {
    let mut word_iter = line.split(' ');
    let mut digit_arr: [usize; 3] = [0; 3];
    let mut digit_count = 0;

    loop {
        let curr_word = match word_iter.next() {
            Some(word) => word,
            None => break,
        };
        let mut char_iter = curr_word.chars();
        let curr_char = match char_iter.next() {
            Some(char) => char,
            None => continue
        };
        if !curr_char.is_ascii_digit() {
            continue;
        };
        let number: u32 = match curr_word.parse() {
            Ok(digit) => digit,
            Err(e) => panic!("Word is not a number")
        };
        // Needed to parse words into numbers instead of turning characters into digits
        digit_arr[digit_count] = number as usize;
        digit_count += 1;
    }
    digit_arr
}

// Move multiple crates takes ownership of stack vector
fn move_multiple_crates(line: &str, mut stack_vec: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let num_arr = parse_digits(line);
    let mut count = num_arr[0];
    let supply_stack = &mut stack_vec[num_arr[2]-1];
    let mut interim_vec: Vec<char> = Vec::new();
    while count > 0 {
        let moving_crate = match supply_stack.pop() {
            Some(value) => value,
            None => panic!()
        };
        count -= 1;
    }
    let target_stack = &mut stack_vec[num_arr[1]-1];
    target_stack.push(interim_vec.pop().unwrap());
    stack_vec
}
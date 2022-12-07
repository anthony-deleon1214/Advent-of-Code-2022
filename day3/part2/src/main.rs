// Importing filesystem from standard library
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt")
        .expect("Requires input.txt file");

    let mut lines = input.lines();
    let mut priority_score = 0;
    let mut curr_line = 1;

    // Binaries are initialized to 0 so they do not effect the later bitwise OR
    let mut line1_bin: u64 = 0;
    let mut line2_bin: u64 = 0;
    let mut line3_bin: u64 = 0;
    
    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break,
        };

        match curr_line % 3 {
            1 => line1_bin = create_bit_representation(line),
            2 => line2_bin = create_bit_representation(line),
            0 => {
                line3_bin = create_bit_representation(line);
                let shared_char = line1_bin & line2_bin & line3_bin;
                priority_score += determine_priority(shared_char);
            },
            _ => panic!("Modulus value not 0, 1, or 2"),
        }

        // Increment line counter after assigning binary representation
        curr_line += 1;
    }

    println!("Total priority score: {}", priority_score)
}

// Exact same function as in part 1 to create binary representations
fn create_bit_representation(compartment: &str) -> u64 {
    // Creates a binary value based on the priority of the characters in the string slice
    let mut str_iter = compartment.chars();
    let mut bin_rep: u64 = 0b0000000000000000000000000000000000000000000000000000;
    let base_char: u64 = 0b1000000000000000000000000000000000000000000000000000;
    
    // Loop through each character in input string
    loop {
        let curr_char = match str_iter.next() {
            Some(char) => char,
            None => break,
        };
        // Bit shift base character based on current character
        let char_binary: u64 = match curr_char {
            'a' => base_char,
            'b' => base_char >> 1,
            'c' => base_char >> 2,
            'd' => base_char >> 3,
            'e' => base_char >> 4,
            'f' => base_char >> 5,
            'g' => base_char >> 6,
            'h' => base_char >> 7,
            'i' => base_char >> 8,
            'j' => base_char >> 9,
            'k' => base_char >> 10,
            'l' => base_char >> 11,
            'm' => base_char >> 12,
            'n' => base_char >> 13,
            'o' => base_char >> 14,
            'p' => base_char >> 15,
            'q' => base_char >> 16,
            'r' => base_char >> 17,
            's' => base_char >> 18,
            't' => base_char >> 19,
            'u' => base_char >> 20,
            'v' => base_char >> 21,
            'w' => base_char >> 22,
            'x' => base_char >> 23,
            'y' => base_char >> 24,
            'z' => base_char >> 25,
            'A' => base_char >> 26,
            'B' => base_char >> 27,
            'C' => base_char >> 28,
            'D' => base_char >> 29,
            'E' => base_char >> 30,
            'F' => base_char >> 31,
            'G' => base_char >> 32,
            'H' => base_char >> 33,
            'I' => base_char >> 34,
            'J' => base_char >> 35,
            'K' => base_char >> 36,
            'L' => base_char >> 37,
            'M' => base_char >> 38,
            'N' => base_char >> 39,
            'O' => base_char >> 40,
            'P' => base_char >> 41,
            'Q' => base_char >> 42,
            'R' => base_char >> 43,
            'S' => base_char >> 44,
            'T' => base_char >> 45,
            'U' => base_char >> 46,
            'V' => base_char >> 47,
            'W' => base_char >> 48,
            'X' => base_char >> 49,
            'Y' => base_char >> 50,
            'Z' => base_char >> 51,
            _ => panic!("Non alphabetic character found"),
        };

        // Bitwise OR between char_binary and bin_rep
        bin_rep = bin_rep | char_binary;
    };
    return bin_rep;
}

fn determine_priority(mut bin_rep: u64) -> u64 {
    let mut shift_count: u64 = 0;

    loop {
        if bin_rep == 1 {
            break
        }
        bin_rep = bin_rep >> 1;
        shift_count += 1
    };

    let priority_value: u64 = 52 - shift_count;
    priority_value
}
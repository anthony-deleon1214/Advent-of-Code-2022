// Importing filesystem from standard library
use std::{fs, collections::{VecDeque, HashMap}};

fn main() {
    part2();
}

fn _part1() {
    let input = fs::read_to_string("src/input.txt")
        .expect("Requires input.txt file");

    // Create a character iterator for the input
    let mut char_iter = input.chars();

    // Store characters in a queue and track characters pushed
    let mut char_count: u32 = 0;
    let mut char_queue: VecDeque<char> = VecDeque::new();
    let mut char_counts_map: HashMap<char, u32> = HashMap::new();
    
    'main_loop: loop {
        let curr_char = match char_iter.next() {
            Some(char) => char,
            None => break
        };
        char_count += 1;
        char_queue.push_back(curr_char);
        let map_count = char_counts_map.entry(curr_char).or_insert(0);
        *map_count += 1;
        if char_queue.len() > 4 {
            let rem_char = char_queue.pop_front().expect("Queue empty");
            let rem_count = char_counts_map.entry(rem_char).or_insert(0);
            *rem_count -= 1;
            if *rem_count == 0 {
                char_counts_map.remove(&rem_char);
            }
        } else if char_queue.len() < 4 {
            continue;
        }
        for value in char_counts_map.values() {
            if *value > 1 {
                continue 'main_loop;
            }
        }
        break;
    }
    println!("Character count before packet start: {}", char_count)
}

fn part2() {
    let input = fs::read_to_string("src/input.txt")
        .expect("Requires input.txt file");

    // Create a character iterator for the input
    let mut char_iter = input.chars();

    // Store characters in a queue and track characters pushed
    let mut char_count: u32 = 0;
    let mut char_queue: VecDeque<char> = VecDeque::new();
    let mut char_counts_map: HashMap<char, u32> = HashMap::new();
    
    'main_loop: loop {
        let curr_char = match char_iter.next() {
            Some(char) => char,
            None => break
        };
        char_count += 1;
        char_queue.push_back(curr_char);
        let map_count = char_counts_map.entry(curr_char).or_insert(0);
        *map_count += 1;
        if char_queue.len() > 14 {
            let rem_char = char_queue.pop_front().expect("Queue empty");
            let rem_count = char_counts_map.entry(rem_char).or_insert(0);
            *rem_count -= 1;
            if *rem_count == 0 {
                char_counts_map.remove(&rem_char);
            }
        } else if char_queue.len() < 14 {
            continue;
        }
        for value in char_counts_map.values() {
            if *value > 1 {
                continue 'main_loop;
            }
        }
        break;
    }
    println!("Character count before message start: {}", char_count)
}
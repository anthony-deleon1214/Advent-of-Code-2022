// Importing File struct from standard library
use std::fs;
fn main() {
    // Reading puzzleInput into a variable as a string
    let puzzle_input: String = fs::read_to_string("src/puzzleInput.txt")
        .expect("puzzleInput.txt is required for this file");

    // Defining empty vectors to store data
    let mut calorie_list: Vec<i32> = Vec::new();
    let mut elf_list: Vec<i32> = Vec::new();

    // Creating a loop to iterate through the lines of puzzle_input
    let mut lines = puzzle_input.lines();
    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break,
        };

        // Check if line has a number in it
        if line.len() > 1 {
            // Trim newline and convert line to i32 and push to calorie_list
            let line: i32 = line.trim().parse().unwrap();
            calorie_list.push(line);
        } else {
            // Calculate sum of calorie_list, store result, and clear calorie_list for continued use
            elf_list.push(sum(&calorie_list));
            calorie_list.clear();
        }
    }

    println!("The elf carrying the most calories has {} calories.", max(&elf_list));

    println!("The total calories carried by the top 3 elves is {}", three_highest(&mut elf_list));
}

fn sum(list: &Vec<i32>) -> i32 {
    let mut sum = 0;

    let list_iter = list.iter();

    for num in list_iter {
        sum = sum + num;
    };

    sum
}

fn max(list: &Vec<i32>) -> &i32 {
    let mut max = &list[0];

    let list_iter = list.iter();

    for number in list_iter {
        if number > max {
            max = number;
        }
    }

    max
}

fn three_highest(list: &mut Vec<i32>) -> i32 {
    list.sort();

    let highest = list.pop().expect("Elf list should be populated");

    let second_highest = list.pop().expect("Elf list should be populated");

    let third_highest = list.pop().expect("Elf list should be populated");

    let total = highest + second_highest + third_highest;
    total
}
// Importing filesystem from standard library
use std::fs;

fn main() {
    part2()
}

fn _part1() {
    let input: String = fs::read_to_string("src/input.txt")
        .expect("Requires input.txt file");

    let mut lines = input.lines();
    let mut count = 0;

    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break,
        };

        let mut elf_iter = line.split(',');
        let elf_1_range = elf_iter.next().expect("No first range found");
        let elf_2_range = elf_iter.next().expect("No second range found");

        let mut range_1_iter = elf_1_range.split('-');
        let mut range_2_iter = elf_2_range.split('-');

        let elf_1_min: i32 = range_1_iter.next().expect("Empty first range").parse().unwrap();
        let elf_1_max: i32 = range_1_iter.next().expect("Empty first range").parse().unwrap();
        let elf_2_min: i32 = range_2_iter.next().expect("Empty first range").parse().unwrap();
        let elf_2_max: i32 = range_2_iter.next().expect("Empty first range").parse().unwrap();

        // Elf 2 is the subset
        if elf_1_min <= elf_2_min && elf_1_max >= elf_2_max {
            count += 1;
        } else if elf_2_min <= elf_1_min && elf_2_max >= elf_1_max {    // Elf 1 is the subset
            count += 1;
        };
    }
    println!("Total count: {}", count);
}

fn part2() {
    let input: String = fs::read_to_string("src/input.txt")
        .expect("Requires input.txt file");

    let mut lines = input.lines();
    let mut count = 0;

    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break,
        };

        let mut elf_iter = line.split(',');
        let elf_1_range = elf_iter.next().expect("No first range found");
        let elf_2_range = elf_iter.next().expect("No second range found");

        let mut range_1_iter = elf_1_range.split('-');
        let mut range_2_iter = elf_2_range.split('-');

        let elf_1_min: i32 = range_1_iter.next().expect("Empty first range").parse().unwrap();
        let elf_1_max: i32 = range_1_iter.next().expect("Empty first range").parse().unwrap();
        let elf_2_min: i32 = range_2_iter.next().expect("Empty first range").parse().unwrap();
        let elf_2_max: i32 = range_2_iter.next().expect("Empty first range").parse().unwrap();

        if elf_1_min >= elf_2_min && elf_1_min <= elf_2_max {
            count += 1
        } else if elf_1_max >= elf_2_min && elf_1_max <= elf_2_max {
            count += 1
        } else if elf_2_min >= elf_1_min && elf_2_min <= elf_1_max {
            count += 1
        } else if elf_2_max >= elf_1_min && elf_2_max <= elf_1_max {
            count += 1
        }
    }
    println!("Total count: {}", count);
}
use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("inputs/day6.txt").expect("file should exist");
    println!("Part 1: {}", get_start_of_message(&input, 4));
    println!("Part 2: {}", get_start_of_message(&input, 14));
}

fn get_start_of_message(signal: &String, unique_chars: usize) -> usize {
    let mut packet_start = VecDeque::new();
    for (i, character) in signal.chars().enumerate() {
        if packet_start.contains(&character) {
            while character != packet_start.pop_front().unwrap() {}
        }
        packet_start.push_back(character);
        if packet_start.len() == unique_chars {
            return i + 1;
        }
    }
    panic!("no start of message");
}

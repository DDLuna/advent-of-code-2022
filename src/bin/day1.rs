use std::{cmp::Reverse, collections::BinaryHeap, fs};

fn main() {
    let mut max = 0;
    let mut top3 = BinaryHeap::new();
    for elf in read_input() {
        let calories = elf.iter().sum();
        max = i32::max(max, calories);
        top3.push(Reverse(calories));
        if top3.len() > 3 {
            top3.pop();
        }
    }
    println!("Part 1 {}", max);
    println!("Part 2 {}", top3.iter().map(|e| e.0).sum::<i32>());
}

fn read_input() -> Vec<Vec<i32>> {
    fs::read_to_string("inputs/day1.txt")
        .expect("Error reading the file")
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|calories| calories.parse().expect("not an int"))
                .collect()
        })
        .collect()
}

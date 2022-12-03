use std::{collections::HashSet, fs, hash::Hash};

fn main() {
    let mut priority_sum = 0;
    let input = read_input();
    for rucksack in &input {
        let mut first_compartment = HashSet::new();
        let mut elements = rucksack.chars();
        for _ in 0..rucksack.len() / 2 {
            first_compartment.insert(elements.next().expect("expected element"));
        }
        while let Some(element) = elements.next() {
            if first_compartment.contains(&element) {
                priority_sum += get_priority(&element);
                break;
            }
        }
    }
    let mut priority_sum_2 = 0;
    for i in (0..input.len()).step_by(3) {
        let mut elements = get_elements(&input[i]);
        elements.retain(|e| get_elements(&input[i + 1]).contains(e));
        elements.retain(|e| get_elements(&input[i + 2]).contains(e));
        priority_sum_2 += get_priority(elements.iter().next().expect("empty intersection"));
    }
    println!("Part 1: {priority_sum}");
    println!("Part 2: {priority_sum_2}");
}

fn read_input() -> Vec<String> {
    fs::read_to_string("inputs/day3.txt")
        .expect("error reading file")
        .split("\n")
        .map(str::to_string)
        .collect()
}

fn get_priority(letter: &char) -> u32 {
    if letter.is_lowercase() {
        *letter as u32 - 'a' as u32 + 1
    } else if letter.is_uppercase() {
        *letter as u32 - 'A' as u32 + 27
    } else {
        panic!("not a letter")
    }
}

fn get_elements(ruckstack: &String) -> HashSet<char> {
    ruckstack.chars().collect()
}

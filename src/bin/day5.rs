use std::{fs, str::SplitWhitespace};

fn main() {
    let (mut stacks, moves) = read_input();
    let mut stacks_2 = stacks.clone();
    for mov in moves {
        let mut it = mov.split_whitespace();
        let count = parse_next(&mut it);
        let from = parse_next(&mut it) - 1;
        let to = parse_next(&mut it) - 1;

        for _ in 0..count {
            let from_crate = &stacks[from].pop().expect("should not be empty");
            stacks[to].push(*from_crate);
        }

        let from_stack = &mut stacks_2[from];
        let mut crates: Vec<_> = from_stack.drain(from_stack.len() - count..).collect();
        stacks_2[to].append(&mut crates);
    }
    let part_1 = get_tops(stacks);
    let part_2 = get_tops(stacks_2);
    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}

fn get_tops(stacks: Vec<Vec<char>>) -> String {
    let mut tops = Vec::new();
    for mut stack in stacks {
        tops.push(stack.pop().expect("should not be empty"));
    }
    tops.iter().collect()
}

fn parse_next(it: &mut SplitWhitespace) -> usize {
    it.nth(1)
        .expect("should have number")
        .parse()
        .expect("should be int")
}

fn read_input() -> (Vec<Vec<char>>, Vec<String>) {
    let binding = fs::read_to_string("inputs/day5.txt").expect("file should exist");
    let mut file = binding.split("\n\n");
    let initial_state = file.next().expect("should contain initial state");
    let moves = file
        .next()
        .expect("should contain moves")
        .split("\n")
        .map(str::to_string)
        .collect();
    (load_inital_state(initial_state), moves)
}

fn load_inital_state(initial_state: &str) -> Vec<Vec<char>> {
    let split = initial_state.split("\n").collect::<Vec<_>>();
    let mut lines = split.iter().rev();

    let number_of_stacks = lines
        .next()
        .expect("should contain number of stacks")
        .split_whitespace()
        .count();
    let mut stacks = Vec::new();
    for _ in 0..number_of_stacks {
        stacks.push(Vec::new());
    }
    while let Some(line) = lines.next() {
        let mut iter = line.chars();
        iter.next();
        for i in 0..number_of_stacks {
            let element = iter.next().expect("should be element or empty");
            if !element.is_whitespace() {
                stacks[i].push(element);
            }
            iter.nth(2);
        }
    }
    stacks
}

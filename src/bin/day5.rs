use std::{fs, str::SplitWhitespace};

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from_str(text: &str) -> Move {
        fn next(it: &mut SplitWhitespace) -> usize {
            it.nth(1)
                .expect("should be Some")
                .parse()
                .expect("should be int")
        }
        let mut it = text.split_whitespace();
        let amount = next(&mut it);
        let from = next(&mut it) - 1;
        let to: usize = next(&mut it) - 1;
        Move { amount, from, to }
    }
}

fn main() {
    let (mut stacks, moves) = read_input();
    let mut stacks_2 = stacks.clone();
    for mov in moves {
        for _ in 0..mov.amount {
            let from_crate = &stacks[mov.from].pop().expect("should not be empty");
            stacks[mov.to].push(*from_crate);
        }

        let from_stack = &mut stacks_2[mov.from];
        let mut crates: Vec<_> = from_stack.drain(from_stack.len() - mov.amount..).collect();
        stacks_2[mov.to].append(&mut crates);
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

fn read_input() -> (Vec<Vec<char>>, Vec<Move>) {
    let binding = fs::read_to_string("inputs/day5.txt").expect("file should exist");
    let mut file = binding.split("\n\n");
    let initial_state = file.next().expect("should contain initial state");
    let moves = file
        .next()
        .expect("should contain moves")
        .split("\n")
        .map(Move::from_str)
        .collect();
    (load_inital_state(initial_state), moves)
}

fn load_inital_state(initial_state: &str) -> Vec<Vec<char>> {
    let split = initial_state.split("\n").collect::<Vec<_>>();
    let mut lines = split.iter().rev();

    let stack_count = lines
        .next()
        .expect("should contain number of stacks")
        .split_whitespace()
        .count();
    let mut stacks = vec![Vec::new(); stack_count];
    while let Some(line) = lines.next() {
        let mut iter = line.chars();
        iter.next();
        for i in 0..stack_count {
            let element = iter.next().expect("should be element or empty");
            if element.is_alphabetic() {
                stacks[i].push(element);
            }
            iter.nth(2);
        }
    }
    stacks
}

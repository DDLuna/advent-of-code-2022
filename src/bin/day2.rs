use std::{collections::HashMap, fs};

fn main() {
    let score_map = HashMap::from([
        ("A X", 3 + 1),
        ("A Y", 6 + 2),
        ("A Z", 0 + 3),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 6 + 1),
        ("C Y", 0 + 2),
        ("C Z", 3 + 3),
    ]);

    let score_map_2 = HashMap::from([
        ("A X", 0 + 3),
        ("A Y", 3 + 1),
        ("A Z", 6 + 2),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 0 + 2),
        ("C Y", 3 + 3),
        ("C Z", 6 + 1),
    ]);

    let mut score = 0;
    let mut score_2 = 0;
    for round in read_input() {
        score += score_map
            .get(round.as_str())
            .expect("not found in score map");
        score_2 += score_map_2
            .get(round.as_str())
            .expect("not found in score map 2");
    }
    println!("Part 1: {}", score);
    println!("Part 2: {}", score_2);
}

fn read_input() -> Vec<String> {
    fs::read_to_string("inputs/day2.txt")
        .expect("error reading file")
        .split("\n")
        .map(str::to_string)
        .collect()
}

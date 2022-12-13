use std::{cmp::Ordering, fs};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn from_str(input: &str) -> Packet {
        if let Ok(num) = input.parse::<i32>() {
            return Packet::Num(num);
        }
        if input == "[]" {
            return Packet::List(Vec::new());
        }
        let mut list = Vec::new();
        let mut start = 1;
        let mut it = input.chars().enumerate();
        it.next();
        while let Some((end, character)) = it.next() {
            if character == '[' {
                let mut matches = 1;
                while let Some((_, character)) = it.next() {
                    if character == '[' {
                        matches += 1;
                    }
                    if character == ']' {
                        matches -= 1;
                    }
                    if matches == 0 {
                        break;
                    }
                }
                continue;
            }
            if character == ',' || character == ']' {
                list.push(Packet::from_str(&input[start..end]));
                start = end + 1;
            }
        }
        Packet::List(list)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::Num(this) => match other {
                Packet::Num(other) => match this - other {
                    i32::MIN..=-1 => Ordering::Less,
                    0 => Ordering::Equal,
                    1..=i32::MAX => Ordering::Greater,
                },
                Packet::List(_) => Packet::List(vec![Packet::Num(*this)]).cmp(other),
            },
            Packet::List(this) => match other {
                Packet::Num(other) => self.cmp(&Packet::List(vec![Packet::Num(*other)])),
                Packet::List(other) => {
                    let mut i = 0;
                    let mut j = 0;
                    while i < this.len() && j < other.len() {
                        match this[i].cmp(&other[j]) {
                            Ordering::Less => return Ordering::Less,
                            Ordering::Equal => (),
                            Ordering::Greater => return Ordering::Greater,
                        };
                        i += 1;
                        j += 1;
                    }
                    if i >= this.len() && j >= other.len() {
                        Ordering::Equal
                    } else if i >= this.len() {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            },
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

impl Pair {
    fn from_str(input: &str) -> Pair {
        let mut it = input.split("\n");
        let left = Packet::from_str(it.next().unwrap());
        let right = Packet::from_str(it.next().unwrap());
        Pair { left, right }
    }

    fn is_in_order(&self) -> bool {
        self.left <= self.right
    }
}

fn read_input() -> Vec<Pair> {
    fs::read_to_string("inputs/day13.txt")
        .unwrap()
        .split("\n\n")
        .map(Pair::from_str)
        .collect()
}
fn main() {
    let input = read_input();
    let mut sum = 0;
    for (i, pair) in input.iter().enumerate() {
        if pair.is_in_order() {
            sum += i + 1;
        }
    }
    println!("Part 1: {}", sum);

    let divider_packets = vec![
        Packet::List(vec![Packet::List(vec![Packet::Num(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Num(6)])]),
    ];
    let mut input_2 = divider_packets.clone();
    for pair in input {
        input_2.push(pair.left);
        input_2.push(pair.right);
    }
    input_2.sort();
    let mut product = 1;
    for (i, packet) in input_2.iter().enumerate() {
        if divider_packets.contains(packet) {
            product *= i + 1;
        }
    }
    println!("Part 2: {}", product);
}

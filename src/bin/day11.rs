use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
    fs,
};

#[derive(Debug, Clone)]
enum Op {
    Mul(u64),
    Sum(u64),
    Pow(u32),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    division_test: u64,
    divisible: usize,
    not_divisible: usize,
    items_inspected: u64,
}

impl Monkey {
    fn round(&mut self, part_1: bool, modulo: u64) -> (VecDeque<u64>, VecDeque<u64>) {
        let mut divisible = VecDeque::new();
        let mut not_divisible = VecDeque::new();
        self.items_inspected += self.items.len() as u64;
        while let Some(item) = self.items.pop_front() {
            let mut new_worry = match &self.op {
                Op::Sum(by) => (item + by) % modulo,
                Op::Mul(by) => (item * by) % modulo,
                Op::Pow(by) => u64::pow(item, *by) % modulo,
            };
            if part_1 {
                new_worry /= 3;
            }
            if new_worry % self.division_test == 0 {
                divisible.push_back(new_worry);
            } else {
                not_divisible.push_back(new_worry);
            };
        }
        (divisible, not_divisible)
    }

    fn from_str(monkey: &str) -> Monkey {
        let mut lines = monkey.split("\n");
        let items = lines
            .nth(1)
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|n| n.parse::<u64>().unwrap())
            .collect();

        let mut op_line = lines
            .next()
            .unwrap()
            .split("old ")
            .nth(1)
            .unwrap()
            .split_whitespace();
        let op = match op_line.next().unwrap() {
            "+" => Op::Sum(op_line.next().unwrap().parse().unwrap()),
            "*" => match op_line.next().unwrap() {
                "old" => Op::Pow(2),
                n => Op::Mul(n.parse().unwrap()),
            },
            err => panic!("not valid operator {}", err),
        };

        let division_test = lines
            .next()
            .unwrap()
            .split_whitespace()
            .nth(3)
            .unwrap()
            .parse()
            .unwrap();
        let divisible = lines
            .next()
            .unwrap()
            .split_whitespace()
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();
        let not_divisible = lines
            .next()
            .unwrap()
            .split_whitespace()
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();
        Monkey {
            items,
            op,
            division_test,
            divisible,
            not_divisible,
            items_inspected: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct MonkeyGroup {
    monkeys: Vec<Monkey>,
    modulo: u64,
}

impl MonkeyGroup {
    fn round(&mut self, part_1: bool) {
        for i in 0..self.monkeys.len() {
            let (mut divisible, mut not_divisible) = self.monkeys[i].round(part_1, self.modulo);
            let divisible_index = self.monkeys[i].divisible;
            self.monkeys[divisible_index].items.append(&mut divisible);
            let not_divisible_index = self.monkeys[i].not_divisible;
            self.monkeys[not_divisible_index]
                .items
                .append(&mut not_divisible);
        }
    }
}

fn read_input() -> MonkeyGroup {
    let monkeys: Vec<Monkey> = fs::read_to_string("inputs/day11.txt")
        .unwrap()
        .split("\n\n")
        .map(Monkey::from_str)
        .collect();
    let modulo = monkeys.iter().map(|m| m.division_test).product();
    MonkeyGroup { monkeys, modulo }
}

fn top_2_mul(items: Vec<u64>) -> u64 {
    let mut min_heap = BinaryHeap::new();
    for item in items {
        min_heap.push(Reverse(item));
        while min_heap.len() > 2 {
            min_heap.pop();
        }
    }
    min_heap.iter().map(|e| e.0).product()
}

fn main() {
    let mut monkeys = read_input();
    let mut monkeys_2 = monkeys.clone();

    for _ in 0..20 {
        monkeys.round(true);
    }
    let part_1 = top_2_mul(monkeys.monkeys.iter().map(|m| m.items_inspected).collect());
    println!("Part 1: {}", part_1);

    for _ in 0..10000 {
        monkeys_2.round(false);
    }
    let part_2 = top_2_mul(
        monkeys_2
            .monkeys
            .iter()
            .map(|m| m.items_inspected)
            .collect(),
    );
    println!("Part 2: {}", part_2);
}

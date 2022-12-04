use std::fs;

struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn from_str(range: &str) -> Range {
        let mut iter = range.split("-");
        let start = iter
            .next()
            .expect("no start value")
            .parse()
            .expect("no int value");
        let end = iter
            .next()
            .expect("no end value")
            .parse()
            .expect("no int value");
        Range { start, end }
    }
}

struct RangePair {
    first: Range,
    second: Range,
}

impl RangePair {
    fn from_str(pair: &str) -> RangePair {
        let mut iter = pair.split(",");
        let first = Range::from_str(iter.next().expect("no first range"));
        let second = Range::from_str(iter.next().expect("no second range"));
        RangePair { first, second }
    }

    fn contained(a: &Range, b: &Range) -> bool {
        a.start >= b.start && a.end <= b.end
    }

    fn intersects(a: &Range, b: &Range) -> bool {
        a.start >= b.start && a.start <= b.end || a.end >= b.start && a.end <= b.end
    }

    fn is_one_range_contained(&self) -> bool {
        RangePair::contained(&self.first, &self.second)
            || RangePair::contained(&self.second, &self.first)
    }

    fn is_one_range_partially_contained(&self) -> bool {
        RangePair::intersects(&self.first, &self.second)
            || RangePair::intersects(&self.second, &self.first)
    }
}

fn read_input() -> Vec<RangePair> {
    fs::read_to_string("inputs/day4.txt")
        .expect("file should exist")
        .split("\n")
        .map(RangePair::from_str)
        .collect()
}

fn main() {
    let pairs = read_input();
    let mut count = 0;
    let mut count_2 = 0;
    for pair in pairs {
        if pair.is_one_range_contained() {
            count += 1;
        }
        if pair.is_one_range_partially_contained() {
            count_2 += 1;
        }
    }
    println!("Part 1: {count}");
    println!("Part 2: {count_2}");
}

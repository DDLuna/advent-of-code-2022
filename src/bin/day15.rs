use std::{collections::HashSet, fs};

#[derive(Debug)]
struct SensorBeacon {
    sx: i32,
    sy: i32,
    bx: i32,
    by: i32,
}

impl SensorBeacon {
    fn from_str(input: &str) -> SensorBeacon {
        let mut words = input.split_whitespace();
        let sx = words.nth(2).unwrap();
        let sx = sx[2..sx.len() - 1].parse::<i32>().unwrap();
        let sy = words.next().unwrap();
        let sy = sy[2..sy.len() - 1].parse::<i32>().unwrap();
        let bx = words.nth(4).unwrap();
        let bx = bx[2..bx.len() - 1].parse::<i32>().unwrap();
        let by = words.next().unwrap()[2..].parse::<i32>().unwrap();
        SensorBeacon { sx, sy, bx, by }
    }

    fn row_coverage(&self, row: i32) -> Vec<i32> {
        let vertical_distance = i32::abs(row - self.sy);
        let distance_to_beacon = self.m_dist(self.bx, self.by);
        if vertical_distance > distance_to_beacon {
            return vec![];
        }
        let mut x = self.sx;
        let mut result = Vec::new();
        while self.m_dist(x, row) <= distance_to_beacon {
            if x != self.bx || row != self.by {
                result.push(x);
            }
            x += 1;
        }
        x = self.sx - 1;
        while self.m_dist(x, row) <= distance_to_beacon {
            if x != self.bx || row != self.by {
                result.push(x);
            }
            x -= 1;
        }
        result
    }

    fn row_coverage_2(&self, row: i32) -> Option<Range> {
        let distance_to_beacon = self.m_dist(self.bx, self.by);
        let distance_to_row = self.m_dist(self.sx, row);
        let leftover = distance_to_beacon - distance_to_row;
        if leftover < 0 {
            return None;
        }
        let from = i32::max(0, self.sx - leftover);
        let to = i32::min(4_000_000, self.sx + leftover);
        Some(Range { from, to })
    }

    fn m_dist(&self, x: i32, y: i32) -> i32 {
        i32::abs(self.sx - x) + i32::abs(self.sy - y)
    }
}

#[derive(Debug, Copy, Clone)]
struct Range {
    from: i32,
    to: i32,
}

impl Range {
    pub fn overlaps(&self, other: &Range) -> bool {
        Range::intersects(self, other) || Range::intersects(other, self)
    }

    pub fn merge(&self, other: &Range) -> Option<Range> {
        if self.overlaps(other) {
            let from = i32::min(self.from, other.from);
            let to = i32::max(self.to, other.to);
            Some(Range { from, to })
        } else {
            None
        }
    }

    pub fn merge_n(ranges: &mut Vec<Range>) -> Vec<Range> {
        for i in 0..ranges.len() {
            for j in i + 1..ranges.len() {
                if let Some(merge) = ranges[i].merge(&ranges[j]) {
                    ranges.remove(i);
                    ranges.remove(j - 1);
                    ranges.push(merge);
                    return Range::merge_n(ranges);
                }
            }
        }
        ranges.clone()
    }

    fn intersects(a: &Range, b: &Range) -> bool {
        a.from >= b.from && a.from <= b.to || a.to >= b.from && a.to <= b.to
    }
}

fn read_input() -> Vec<SensorBeacon> {
    fs::read_to_string("inputs/day15.txt")
        .unwrap()
        .split("\n")
        .map(SensorBeacon::from_str)
        .collect()
}

fn main() {
    let input = read_input();
    let target_row = 2000000;
    let mut coverage: HashSet<i32> = HashSet::new();
    for sensor in &input {
        sensor.row_coverage(target_row).iter().for_each(|x| {
            coverage.insert(*x);
        });
    }
    println!("Part 1: {}", coverage.len());
    for row in 0..4_000_000 {
        let mut ranges = Vec::new();
        for sensor in &input {
            if let Some(range) = sensor.row_coverage_2(row) {
                ranges.push(range);
            }
        }
        ranges = Range::merge_n(&mut ranges);
        if ranges.len() > 1 {
            let range_1 = ranges[0];
            let x = if range_1.from == 0 {
                range_1.to + 1
            } else {
                range_1.from - 1
            };
            let tuning_frequency = x as u128 * 4_000_000 + row as u128;
            println!("Part 2: {}", tuning_frequency);
            break;
        }
    }
}

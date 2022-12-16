use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_str(point: &str) -> Point {
        let mut it = point.split(",");
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        Point { x, y }
    }
}

fn main() {
    let (points, lowest) = read_input();
    let points_2 = points.clone();
    let grains = part_1(points, lowest);
    let grains_2 = part_2(points_2, lowest + 2);
    println!("Part 1: {}", grains);
    println!("Part 2: {}", grains_2);
}

fn part_1(mut points: HashSet<Point>, lowest: i32) -> i32 {
    let mut grains = 0;
    while add_grain(&mut points, lowest) {
        grains += 1;
    }
    grains
}

fn part_2(mut points: HashSet<Point>, floor: i32) -> i32 {
    let mut grains = 0;
    while add_grain_2(&mut points, floor) {
        grains += 1;
    }
    grains + 1
}

fn add_grain(points: &mut HashSet<Point>, lowest: i32) -> bool {
    let mut grain = Point { x: 500, y: 0 };
    loop {
        while check_down(points, &grain) && grain.y < lowest {
            grain.y += 1;
        }
        if check_down_left(points, &grain) && grain.y < lowest {
            grain.x -= 1;
            grain.y += 1;
            continue;
        }
        if check_down_right(points, &grain) && grain.y < lowest {
            grain.x += 1;
            grain.y += 1;
            continue;
        }
        return if grain.y == lowest {
            false
        } else {
            points.insert(grain.clone());
            true
        };
    }
}

fn add_grain_2(points: &mut HashSet<Point>, floor: i32) -> bool {
    let mut grain = Point { x: 500, y: 0 };
    let origin = grain.clone();
    loop {
        while check_down(points, &grain) && grain.y + 1 != floor {
            grain.y += 1;
        }
        if check_down_left(points, &grain) && grain.y + 1 != floor {
            grain.x -= 1;
            grain.y += 1;
            continue;
        }
        if check_down_right(points, &grain) && grain.y + 1 != floor {
            grain.x += 1;
            grain.y += 1;
            continue;
        }
        points.insert(grain.clone());
        break;
    }
    grain != origin
}

fn check_down(points: &HashSet<Point>, point: &Point) -> bool {
    !points.contains(&Point {
        x: point.x,
        y: point.y + 1,
    })
}

fn check_down_left(points: &HashSet<Point>, point: &Point) -> bool {
    !points.contains(&Point {
        x: point.x - 1,
        y: point.y + 1,
    })
}

fn check_down_right(points: &HashSet<Point>, point: &Point) -> bool {
    !points.contains(&Point {
        x: point.x + 1,
        y: point.y + 1,
    })
}

fn read_input() -> (HashSet<Point>, i32) {
    let mut points = HashSet::new();
    let mut lowest = 0;
    fs::read_to_string("inputs/day14.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            let mut it = line.split(" -> ");
            let mut start = Point::from_str(it.next().unwrap());
            lowest = i32::max(lowest, start.y);
            while let Some(end) = it.next() {
                let end = Point::from_str(end);
                lowest = i32::max(lowest, end.y);
                let mut low = i32::min(start.x, end.x);
                let mut high = i32::max(start.x, end.x);
                for i in low..=high {
                    points.insert(Point { x: i, y: start.y });
                }
                low = i32::min(start.y, end.y);
                high = i32::max(start.y, end.y);
                for i in low..=high {
                    points.insert(Point { x: start.x, y: i });
                }
                start = end;
            }
        });
    (points, lowest)
}

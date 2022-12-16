use std::{
    collections::{HashSet, VecDeque},
    fs,
    io::{self, Write},
    thread,
    time::Duration,
};

use termion::{clear, color, cursor};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point(usize, usize);

fn main() {
    let (grid, start, end) = read_input();
    let result = bfs(&grid, start, end, false);
    let result_2 = bfs(&grid, end, start, true);
    println!("Part 1: {}", result);
    println!("Part 2: {}", result_2);
}

fn bfs(grid: &Vec<Vec<u8>>, start: Point, end: Point, part_2: bool) -> u32 {
    let mut visited = HashSet::new();
    let mut step = 0;
    let mut q = VecDeque::new();
    q.push_back(start);
    draw(&grid, &visited, &q);
    while !q.is_empty() {
        let size = q.len();
        let prev = q.clone();
        for _ in 0..size {
            let curr = q.pop_front().unwrap();
            if visited.contains(&curr) {
                continue;
            }
            visited.insert(curr);

            if part_2 {
                if grid[curr.0][curr.1] == 'a' as u8 {
                    return step;
                }
            } else if curr == end {
                return step;
            }

            let row = curr.0;
            let col = curr.1;
            if row > 0 {
                let p = Point(curr.0 - 1, curr.1);
                if !visited.contains(&p) && valid_height_diff(&grid, curr, p, part_2) {
                    q.push_back(p);
                }
            }
            let p = Point(curr.0 + 1, curr.1);
            if row < grid.len() - 1
                && !visited.contains(&p)
                && valid_height_diff(&grid, curr, p, part_2)
            {
                q.push_back(p);
            }
            if col > 0 {
                let p = Point(curr.0, curr.1 - 1);
                if !visited.contains(&p) && valid_height_diff(&grid, curr, p, part_2) {
                    q.push_back(p);
                }
            }
            let p = Point(curr.0, curr.1 + 1);
            if col < grid[0].len() - 1
                && !visited.contains(&p)
                && valid_height_diff(&grid, curr, p, part_2)
            {
                q.push_back(p);
            }
        }
        update(&grid, &prev, &q);
        step += 1;
    }
    panic!("No path found");
}

const DRAW_SPEED_MS: u64 = 50;

fn draw(grid: &Vec<Vec<u8>>, visited: &HashSet<Point>, next: &VecDeque<Point>) {
    let grid: Vec<Vec<char>> = grid
        .iter()
        .map(|row| row.iter().map(|n| *n as char).collect())
        .collect();

    print!("{}{}", cursor::Goto(1, 1), clear::All);
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let point = Point(i, j);
            if visited.contains(&point) {
                print!("{}{}", color::Fg(color::Green), cell);
            } else if next.contains(&point) {
                print!("{}{}", color::Fg(color::Yellow), cell);
            } else {
                print!("{}{}", color::Fg(color::White), cell)
            };
        }
        println!();
    }
    thread::sleep(Duration::from_millis(DRAW_SPEED_MS));
}

fn update(grid: &Vec<Vec<u8>>, prev: &VecDeque<Point>, next: &VecDeque<Point>) {
    for point in prev {
        print!(
            "{}{}{}{}",
            cursor::Goto(point.1 as u16 + 1, point.0 as u16 + 1),
            cursor::Hide,
            color::Fg(color::Green),
            grid[point.0][point.1] as char
        );
    }
    for point in next {
        print!(
            "{}{}{}{}",
            cursor::Goto(point.1 as u16 + 1, point.0 as u16 + 1),
            cursor::Hide,
            color::Fg(color::Yellow),
            grid[point.0][point.1] as char
        );
    }
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(DRAW_SPEED_MS));
}

fn valid_height_diff(grid: &Vec<Vec<u8>>, a: Point, b: Point, part_2: bool) -> bool {
    if part_2 {
        grid[a.0][a.1] <= grid[b.0][b.1] + 1
    } else {
        grid[b.0][b.1] <= grid[a.0][a.1] + 1
    }
}

fn read_input() -> (Vec<Vec<u8>>, Point, Point) {
    let mut grid: Vec<Vec<_>> = fs::read_to_string("inputs/day12.txt")
        .unwrap()
        .split("\n")
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect();
    let mut start = Point(0, 0);
    let mut end = Point(0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' as u8 {
                start = Point(i, j);
                grid[i][j] = 'a' as u8
            }
            if grid[i][j] == 'E' as u8 {
                end = Point(i, j);
                grid[i][j] = 'z' as u8;
            }
        }
    }
    (grid, start, end)
}

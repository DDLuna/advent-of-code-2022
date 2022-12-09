use std::{collections::HashSet, fs};

struct Rope {
    knots: Vec<(i32, i32)>,
    tail_visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(knots: usize) -> Rope {
        Rope {
            knots: vec![(0, 0); knots],
            tail_visited: HashSet::from([(0, 0)]),
        }
    }
    fn mov_rope(&mut self, dir: &str) {
        Rope::mov(&mut self.knots[0], dir);
        for i in 1..self.knots.len() {
            let head = self.knots[i - 1];
            let mut tail = &mut self.knots[i];
            if i32::abs(head.0 - tail.0) > 1 {
                if head.0 > tail.0 {
                    Rope::mov(&mut tail, "R");
                } else {
                    Rope::mov(&mut tail, "L");
                }
                if head.1 > tail.1 {
                    Rope::mov(&mut tail, "U");
                }
                if head.1 < tail.1 {
                    Rope::mov(&mut tail, "D");
                }
            }
            if i32::abs(head.1 - tail.1) > 1 {
                if head.1 > tail.1 {
                    Rope::mov(&mut tail, "U");
                } else {
                    Rope::mov(&mut tail, "D");
                }
                if head.0 > tail.0 {
                    Rope::mov(&mut tail, "R");
                }
                if head.0 < tail.0 {
                    Rope::mov(&mut tail, "L");
                }
            }
        }
        self.tail_visited.insert(*self.knots.last().unwrap());
    }
    fn mov(knot: &mut (i32, i32), dir: &str) {
        match dir {
            "R" => knot.0 += 1,
            "L" => knot.0 -= 1,
            "U" => knot.1 += 1,
            "D" => knot.1 -= 1,
            _ => panic!("unrecognized dir"),
        }
    }
}

fn main() {
    let mut rope = Rope::new(2);
    let mut rope_2 = Rope::new(10);
    fs::read_to_string("inputs/day9.txt")
        .unwrap()
        .split("\n")
        .map(|line| {
            let mut line_split = line.split(" ");
            (
                line_split.next().unwrap(),
                line_split.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .for_each(|mov| {
            for _ in 0..mov.1 {
                rope.mov_rope(mov.0);
                rope_2.mov_rope(mov.0);
            }
        });

    println!("Part 1: {}", rope.tail_visited.iter().count());
    println!("Part 2: {}", rope_2.tail_visited.iter().count());
}

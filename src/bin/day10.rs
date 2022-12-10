use std::fs;

enum Instruction {
    Addx(i32),
    Nope,
}

fn read_input() -> Vec<Instruction> {
    fs::read_to_string("inputs/day10.txt")
        .unwrap()
        .split("\n")
        .map(|line| {
            let mut split = line.split_whitespace();
            match split.next().unwrap() {
                "addx" => Instruction::Addx(split.next().unwrap().parse().unwrap()),
                "noop" => Instruction::Nope,
                i => panic!("instruction not recognized {i}"),
            }
        })
        .collect()
}

fn check_signal(cycle: u32, reg: i32, signal: &mut i32) {
    if cycle >= 20 && (cycle - 20) % 40 == 0 {
        *signal += reg * cycle as i32;
    }
}

fn draw_pixel(display: &mut [[char; 40]; 6], mut cycle: u32, x_reg: i32) {
    cycle -= 1;
    let row = (cycle / 40) as usize;
    let col = (cycle % 40) as usize;
    display[row][col] = if col as i32 >= x_reg - 1 && col as i32 <= x_reg + 1 {
        '#'
    } else {
        '.'
    }
}

fn main() {
    let mut x_reg: i32 = 1;
    let mut cycle: u32 = 1;
    let mut signal_strength: i32 = 0;
    let mut display = [['$'; 40]; 6];
    let instructions = read_input();
    for instruction in instructions {
        match instruction {
            Instruction::Addx(num) => {
                for _ in 0..2 {
                    draw_pixel(&mut display, cycle, x_reg);
                    check_signal(cycle, x_reg, &mut signal_strength);
                    cycle += 1;
                }
                x_reg += num;
            }
            Instruction::Nope => {
                draw_pixel(&mut display, cycle, x_reg);
                check_signal(cycle, x_reg, &mut signal_strength);
                cycle += 1;
            }
        }
    }

    println!("Part 1: {}", signal_strength);
    println!("Part 2:");
    for row in display {
        println!("{}", String::from_iter(row));
    }
}

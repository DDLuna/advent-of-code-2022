use std::fs;

fn main() {
    let grid = read_input();
    println!("Part 1: {}", part_1(&grid));
    println!("Part 2: {}", part_2(&grid));
}

fn part_2(grid: &Vec<Vec<u32>>) -> u32 {
    let mut scenic_score = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            scenic_score = u32::max(scenic_score, calc_scenic_score(&grid, row, col));
        }
    }
    scenic_score
}

fn calc_scenic_score(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let base_tree = grid[row][col];

    let mut down_score = 0;
    for row_index in row + 1..grid.len() {
        down_score += 1;
        let tree = grid[row_index][col];
        if tree >= base_tree {
            break;
        }
    }
    let mut up_score = 0;
    for row_index in (0..row).rev() {
        up_score += 1;
        let tree = grid[row_index][col];
        if tree >= base_tree {
            break;
        }
    }
    let mut right_score = 0;
    for col_index in col + 1..grid[0].len() {
        right_score += 1;
        let tree = grid[row][col_index];
        if tree >= base_tree {
            break;
        }
    }
    let mut left_score = 0;
    for col_index in (0..col).rev() {
        left_score += 1;
        let tree = grid[row][col_index];
        if tree >= base_tree {
            break;
        }
    }
    left_score * right_score * up_score * down_score
}

fn part_1(grid: &Vec<Vec<u32>>) -> u32 {
    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];

    for (row_index, row) in grid.iter().enumerate() {
        let mut high = -1;
        for (col_index, tree) in row.iter().enumerate() {
            if *tree as i32 > high {
                seen[row_index][col_index] = true;
                high = *tree as i32;
            }
        }
    }

    for row_index in 0..grid.len() {
        let mut high = -1;
        for col_index in (0..grid[0].len()).rev() {
            let tree = grid[row_index][col_index];
            if tree as i32 > high {
                seen[row_index][col_index] = true;
                high = tree as i32;
            }
        }
    }

    for col_index in 0..grid[0].len() {
        let mut high = -1;
        for row_index in 0..grid.len() {
            let tree = grid[row_index][col_index];
            if tree as i32 > high {
                seen[row_index][col_index] = true;
                high = tree as i32;
            }
        }
    }

    for col_index in 0..grid[0].len() {
        let mut high = -1;
        for row_index in (0..grid.len()).rev() {
            let tree = grid[row_index][col_index];
            if tree as i32 > high {
                seen[row_index][col_index] = true;
                high = tree as i32;
            }
        }
    }

    seen.iter()
        .map(|row| {
            row.iter()
                .map(|seen| if *seen { 1 } else { 0 })
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn read_input() -> Vec<Vec<u32>> {
    fs::read_to_string("inputs/day8.txt")
        .expect("file should exist")
        .split("\n")
        .map(|line| line.chars().map(|n| n.to_digit(10).unwrap()).collect())
        .collect()
}

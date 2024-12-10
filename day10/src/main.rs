use std::collections::HashSet;
use std::fs;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn dfs(
    grid: &Vec<Vec<i32>>,
    row: i32,
    col: i32,
    next: i32,
    target: i32,
    final_stops: &mut HashSet<(i32, i32)>,
) -> i32 {
    if row < 0 || row >= grid.len() as i32 || col < 0 || col >= grid[0].len() as i32 {
        return 0;
    }
    if grid[row as usize][col as usize] != next {
        return 0;
    }
    if grid[row as usize][col as usize] == target {
        final_stops.insert((row, col));
        return 1;
    }
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .map(|(dr, dc)| (row + dr, col + dc))
        .map(|(r, c)| dfs(&grid, r, c, next + 1, target, final_stops))
        .iter()
        .sum()
}

fn part1(grid: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let mut final_stops = HashSet::new();
            dfs(&grid, row as i32, col as i32, 0, 9, &mut final_stops);
            sum += final_stops.len() as i32
        }
    }
    sum
}

fn part2(grid: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let mut final_stops = HashSet::new();
            sum += dfs(&grid, row as i32, col as i32, 0, 9, &mut final_stops);
        }
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let grid = parse_input(&input);
    println!("{}", part1(grid.clone()));
    println!("{}", part2(grid.clone()));
}

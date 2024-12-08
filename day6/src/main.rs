use std::collections::HashSet;
use std::fs;
use std::sync::{Arc, Mutex}; // Add this line at the top of your file
use std::thread; // Add this line at the top of your file

#[derive(Debug)]
struct Guard {
    location: (i32, i32),
    direction: Direction,
    on_grid: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Guard {
    fn walk(&mut self, grid: &Vec<Vec<char>>) -> &Guard {
        let mut new_location = self.next_location();
        if new_location.1 < 0
            || new_location.1 >= grid.len() as i32
            || new_location.0 < 0
            || new_location.0 >= grid[0].len() as i32
        {
            self.on_grid = false;
            return self;
        }
        if grid[new_location.1 as usize][new_location.0 as usize] == '#' {
            self.turn();
            new_location = self.next_location();
        }
        self.location = new_location;
        self
    }

    fn next_location(&self) -> (i32, i32) {
        let mut new_location = self.location;
        match self.direction {
            Direction::Up => new_location.1 -= 1,
            Direction::Right => new_location.0 += 1,
            Direction::Down => new_location.1 += 1,
            Direction::Left => new_location.0 -= 1,
        }
        new_location
    }

    fn next_direction(&self, direction: &Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn(&mut self) -> &Guard {
        self.direction = self.next_direction(&self.direction);
        self
    }
}

fn char_to_direction(c: char) -> Direction {
    match c {
        '^' => Direction::Up,
        '>' => Direction::Right,
        'v' => Direction::Down,
        '<' => Direction::Left,
        _ => panic!("Invalid direction"),
    }
}

fn find_guard(grid: &Vec<Vec<char>>) -> Guard {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '^' || cell == '>' || cell == 'v' || cell == '<' {
                return Guard {
                    location: (x as i32, y as i32),
                    direction: char_to_direction(cell),
                    on_grid: true,
                };
            }
        }
    }
    panic!("Guard not found");
}

fn has_cycle(grid: &Vec<Vec<char>>) -> bool {
    let mut fast_guard = find_guard(grid);
    let mut slow_guard = find_guard(grid);
    while fast_guard.on_grid && slow_guard.on_grid {
        fast_guard.walk(grid);
        fast_guard.walk(grid);
        slow_guard.walk(grid);
        if fast_guard.location == slow_guard.location && fast_guard.direction == slow_guard.direction {
            return true;
        }
    }
    false
}

fn part2(grid: Vec<Vec<char>>) -> i32 {
    let grid_len = grid.len();
    let grid_width = grid[0].len();
    let mut cycles = 0;
    for y in 0..grid_len {
        for x in 0..grid_width {
            if grid[y][x] == '.' {
                let mut grid_clone = grid.clone();
                grid_clone[y][x] = '#';
                if has_cycle(&grid_clone) {
                    cycles += 1;
                }
            }
        }
    }
    cycles
}

fn part1(grid: Vec<Vec<char>>) -> i32 {
    let mut guard = find_guard(&grid);

    let mut distinct_locations = HashSet::new();
    distinct_locations.insert(guard.location);

    while guard.on_grid {
        guard.walk(&grid);
        distinct_locations.insert(guard.location);
    }
    distinct_locations.len() as i32
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    println!("Part 1: {}", part1(grid.clone()));
    println!("Part 2: {}", part2(grid.clone()));
}

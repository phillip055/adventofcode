use std::fs;
use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| line.trim().split("").map(|s| s.to_string()).filter(|s| s != "").collect())
        .collect()
}

fn find_antennas(grid: &Vec<Vec<String>>) -> HashMap<&String, Vec<(i32, i32)>> {
    let mut antennas = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell != "." {
                antennas.entry(cell).or_insert(vec![]).push((i as i32, j as i32));
            }
        }
    }
    antennas
}

fn get_combinations(antennas: Vec<(i32, i32)>) -> Vec<((i32, i32), (i32, i32))> {
    let mut combinations = vec![];
    for i in 0..antennas.len() {
        for j in i+1..antennas.len() {
            combinations.push((antennas[i], antennas[j]));
        }
    }
    combinations
}

fn get_distance(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 as i32 - b.0 as i32, a.1 as i32 - b.1 as i32)
}

fn add_distance(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn subtract_distance(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
}

fn inbounds(a: (i32, i32), grid: &Vec<Vec<String>>) -> bool {
    a.0 >= 0 && a.0 < grid.len() as i32 && a.1 >= 0 && a.1 < grid[0].len() as i32
}

fn part1(grid: &Vec<Vec<String>>, combs: HashMap<&String, Vec<((i32, i32), (i32, i32))>>) -> i32 {
    let mut visited = HashSet::new();
    let mut grid: Vec<Vec<String>> = grid.clone();
    for (_, combinations) in combs {
        for (a, b) in combinations {
            let distance = get_distance(a, b);
            let new_a = add_distance(a, distance);
            let new_b = subtract_distance(b, distance);            
            if inbounds(new_a, &grid) {
                visited.insert(new_a);
            }
            if inbounds(new_b, &grid) {
                visited.insert(new_b);
            }
        }
    }
    for item in visited.iter() {
        grid[item.0 as usize][item.1 as usize] = "#".to_string();
    }
    return visited.len() as i32;
}

fn part2(grid: &Vec<Vec<String>>, combs: HashMap<&String, Vec<((i32, i32), (i32, i32))>>) -> i32 {
    let mut visited = HashSet::new();
    for (_, combinations) in combs {
        for (a, b) in combinations {
            let distance = get_distance(a, b);
            let mut a = a;
            let mut b = b;
            visited.insert(a);
            visited.insert(b);
            while inbounds(add_distance(a, distance), &grid) {
                a = add_distance(a, distance);
                visited.insert(a);
            }
            while inbounds(subtract_distance(b, distance), &grid) {
                b = subtract_distance(b, distance);
                visited.insert(b);
            }
        }
        
    }
    return visited.len() as i32;
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let grid = parse(&input);
    let antennas = find_antennas(&grid);

    let mut combs = HashMap::new();

    for (antenna, positions) in antennas {
        let combinations = get_combinations(positions);
        combs.insert(antenna, combinations);
    }

    println!("{}", part1(&grid, combs.clone()));
    println!("{}", part2(&grid, combs.clone()));
}

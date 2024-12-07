use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

static DIRECTION: [(i32, i32); 8] = [(1,0), (0, 1), (1, 1), (1, -1), (-1, 0), (0, -1), (-1, -1), (-1, 1)];

fn dfs(s: Vec<Vec<String>>, i: i32, j: i32, next: String, direction: (i32, i32), next_letter: HashMap<String, String>) -> usize {
    if next == "" {
        return 1;
    }
    if i == s.len() as i32 || j == s[0].len() as i32 || i < 0 || j < 0 {
        return 0;
    }
    if s[i as usize][j as usize] != next {
        return 0;
    }
    let x = i + direction.0;
    let y = j + direction.1;
    if dfs(s, x, y, next_letter[&next].clone(), direction, next_letter) == 1 {
        return 1;
    }
    return 0;
}

fn part1(s: &Vec<Vec<String>>) -> usize {
    let mut next_letter = HashMap::new();
    next_letter.insert("X".to_string(), "M".to_string());
    next_letter.insert("M".to_string(), "A".to_string());
    next_letter.insert("A".to_string(), "S".to_string());
    next_letter.insert("S".to_string(), "".to_string());
    let mut matches_right_order = 0;
    for i in 0..s.len() {
        for j in 0..s[i].len() {
            for direction in DIRECTION.iter() {
                if s[i][j] == "X" {
                    if dfs(s.clone(), i as i32, j as i32, s[i][j].clone(), *direction, next_letter.clone()) == 1 {
                        matches_right_order += 1;
                    }
                }
            }
        }
    }
    return matches_right_order;
}

fn in_bound(s: &Vec<Vec<String>>, (i, j): (i32, i32)) -> bool {
    return i >= 0 && j >= 0 && i < s.len() as i32 && j < s[0].len() as i32;
}

fn get_val(s: &Vec<Vec<String>>, (i, j): (i32, i32)) -> String {
    return s[i as usize][j as usize].clone();
}

fn find_xmas(s: &Vec<Vec<String>>, i: i32, j: i32) -> bool {
    let top_left = (i-1, j-1);
    let bottom_right = (i+1, j+1);
    let top_right = (i-1, j+1);
    let bottom_left = (i+1, j-1);
    
    if in_bound(&s, top_left) && in_bound(&s, bottom_right) && in_bound(&s, top_right) && in_bound(&s, bottom_left) {
        let top_left_val = get_val(&s, top_left);
        let bottom_right_val = get_val(&s, bottom_right);
        let top_right_val = get_val(&s, top_right);
        let bottom_left_val = get_val(&s, bottom_left);
        
        if (top_left_val == "M" && bottom_right_val == "S") || (top_left_val == "S" && bottom_right_val == "M") {
            if (top_right_val == "M" && bottom_left_val == "S") || (top_right_val == "S" && bottom_left_val == "M") {
                return true;
            }
        }
        return false
    }
    return false;
}

fn part2(s: &Vec<Vec<String>>) -> usize {
    let mut count = 0;
    for i in 0..s.len() {
        for j in 0..s[i].len() {
            if s[i][j] == "A" {
                if find_xmas(&s, i as i32, j as i32) {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn main() {
    let file = File::open("input/input.txt").unwrap();
    let reader = BufReader::new(file);
    let s: Vec<Vec<String>> = reader
        .lines()
        .map(|line| line.unwrap().split("").map(|x| x.to_string()).filter(|x| x != "").collect())
        .collect();

    println!("matches: {}", part1(&s));
    println!("count: {}", part2(&s));
}

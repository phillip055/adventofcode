use std::{collections::HashMap, fs};

fn parse_input(input: &str) -> Vec<i64> {
    input.to_string().split_ascii_whitespace().map(|num| {
        num.parse::<i64>().unwrap() // Convert string to i64
    }).collect()
}

fn split_stone(stone: i64) -> (i64, i64) {
    let stone_str = stone.to_string();
    let (left, right) = stone_str.split_at(stone_str.len() / 2);
    (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap())
}

fn blink_stone(stone: i64, remaining_blinks: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if let Some(result) = cache.get(&(stone, remaining_blinks)) {
        return *result;
    }
    if remaining_blinks == 0 {
        cache.insert((stone, remaining_blinks), 1);
        return 1;
    }
    if stone == 0 { 
        let result = blink_stone(1, remaining_blinks - 1, cache);
        cache.insert((stone, remaining_blinks), result);
        return result;
    }
    if stone.to_string().len() % 2 == 0 {
        let (left, right) = split_stone(stone);
        let result = blink_stone(left, remaining_blinks - 1, cache) + blink_stone(right, remaining_blinks - 1, cache);
        cache.insert((stone, remaining_blinks), result);
        return result;
    }
    let result = blink_stone(stone * 2024, remaining_blinks - 1, cache);
    cache.insert((stone, remaining_blinks), result);
    return result;
}

fn blink_stones(stones: Vec<i64>, blinks: i64) -> i64 {   
    stones.iter().map(|&stone| {
        let mut cache = HashMap::new();
        let res = blink_stone(stone, 75, &mut cache);
        res
    }).sum()
}

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let stones = parse_input(&input);
    println!("{:?}", blink_stones(stones, 75));
}

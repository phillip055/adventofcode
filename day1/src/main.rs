use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

fn part1(mut a_locations: Vec<i32>, mut b_locations: Vec<i32>) -> i32 {
    a_locations.sort();
    b_locations.sort();
    let result: i32 = zip(a_locations.iter(), b_locations.iter())
        .map(|(x, y)| (x - y).abs())
        .sum();
    return result;
}

fn part2(a_locations: Vec<i32>, b_locations: Vec<i32>) -> i32 {
    let freq = b_locations.into_iter().counts();
    let result = a_locations
        .into_iter()
        .map(|x| (*freq.get(&x).unwrap_or(&0) as i32) * x)
        .sum();
    return result;
}

fn main() {
    let file = File::open("input/input.txt").unwrap_or_else(|_| panic!("File not found"));
    let reader = BufReader::new(file);
    let s: Vec<(i32, i32)> = reader
        .lines()
        .filter_map(|x| {
            x.ok().and_then(|line_str| {
                line_str.split_once(&" ".repeat(3)).and_then(|(a, b)| {
                    let a_parsed = a.parse::<i32>().ok();
                    let b_parsed = b.parse::<i32>().ok();
                    match (a_parsed, b_parsed) {
                        (Some(a), Some(b)) => Some((a, b)),
                        _ => panic!("Something weird happend"),
                    }
                })
            })
        })
        .collect();
    let (a_locations, b_locations): (Vec<i32>, Vec<i32>) = s.into_iter().unzip();
    let result = part1(a_locations.clone(), b_locations.clone());
    println!("Part 1 Result: {}", result);
    let result = part2(a_locations.clone(), b_locations.clone());
    println!("Part 2 Result: {}", result);
}

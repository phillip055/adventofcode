use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_decreasing(report: &Vec<i32>) -> bool {
    report.windows(2).all(|w| {
        let diff = w[1] - w[0];
        diff <= -1 && diff >= -3
    })
}

fn is_increasing(report: &Vec<i32>) -> bool {
    report.windows(2).all(|w| {
        let diff = w[1] - w[0];
        diff >= 1 && diff <= 3
    })
}

fn is_safe(report: &Vec<i32>, dampener: bool) -> bool {
    if dampener {
        if is_decreasing(report) || is_increasing(report) {
            return true
        }
        for i in 0..report.len() {
            let new_report: Vec<i32> = report.iter().enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, &v)| v)
                .collect();

            if is_safe(&new_report, false) {
                return true;
            }
        }
        false
    } else {
        is_decreasing(report) || is_increasing(report)
    }
}

fn main() {
    let file = File::open("input/input.txt").unwrap_or_else(|_| panic!("File not found"));
    let reader = BufReader::new(file);
    let s: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| line.unwrap().split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    let f = s.clone().into_iter().filter(|x| is_safe(x, false)).count();
    let g = s.clone().into_iter().filter(|x| is_safe(x, true)).count();
    println!("{} {}", f, g);
}

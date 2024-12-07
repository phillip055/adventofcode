use std::fs::File;
use std::io::{BufRead, BufReader};

  
fn consume_keyword(s: &str, idx: usize, keyword: &str) -> bool {
    s.get(idx..idx + keyword.len()) == Some(keyword)
}

fn consume_char(s: &str, idx: usize, c: char) -> bool {
    s.get(idx..idx + 1) == Some(&c.to_string())
}

fn consume_number(s: &str, idx: usize) -> Option<i32> {
    let end = s[idx..].chars().take_while(|c| c.is_digit(10)).count();
    if end == 0 { return None; }
    s[idx..idx + end].parse::<i32>().ok()
}

fn main() {
    let file = File::open("input/input.txt").unwrap();
    let reader = BufReader::new(file);
    let s = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>().join("");

    let mut i = 0;
    let mut sum = 0;
    let mut enabled = true;
    while i < s.len() {
        let mut candidate: usize = i;
        if !enabled {
            if consume_keyword(&s, candidate, "do()") {
                enabled = true;
                i += 4;
                continue;
            }
        }
        if enabled {
            if consume_keyword(&s, candidate, "don't()") {
                enabled = false;
                i += 7;
                continue;
            }
        }
        if !enabled {
            i += 1;
            continue;
        }
        let mut multiplicand: Option<i32> = None;
        let mut multiplier: Option<i32> = None;
        let mut result = 0;
        if !consume_keyword(&s, candidate, "mul") {
            i += 1;
            continue;
        }
        candidate += 3;
        if !consume_char(&s, candidate, '(') {
            i += 1;
            continue;
        }
        candidate += 1;
        if let Some(num) = consume_number(&s, candidate) {
            candidate += num.to_string().len();
            multiplicand = Some(num);
        } else {
            i += 1;
            continue;
        }
        if !consume_char(&s, candidate, ',') {
            i += 1;
            continue;
        }
        candidate += 1;
        if let Some(num) = consume_number(&s, candidate) {
            candidate += num.to_string().len();
            multiplier = Some(num);
        } else {
            i += 1;
            continue;
        }
        if !consume_char(&s, candidate, ')') {
            i += 1;
            continue;
        }
        candidate += 1;
        match (multiplicand, multiplier) {
            (Some(m), Some(n)) => {
                result = m * n;
            }
            _ => {
                i += 1;
                continue;
            }
        }
        sum += result;
        i = candidate - 1
    }
    println!("{}", sum);
}

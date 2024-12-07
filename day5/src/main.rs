use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn parse_ints(line: &str, delim: &str) -> Vec<i32> {
    line.split(delim)
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn parse_rules(rules: &str) -> Vec<(i32, i32)> {
    rules
        .split("\n")
        .map(|line| parse_ints(line, "|"))
        .map(|ints| (ints[0], ints[1]))
        .collect()
}

fn parse_orders(orders: &str) -> Vec<Vec<i32>> {
    orders
        .split("\n")
        .map(|line| parse_ints(line, ","))
        .collect()
}

fn parse(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    input
        .split_once("\n\n")
        .map(|(rules, orders)| (parse_rules(rules), parse_orders(orders)))
        .unwrap()
}

fn is_sorted(order: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    let mut position = HashMap::new();

    for (index, &value) in order.iter().enumerate() {
        position.insert(value, index);
    }

    for (from, to) in rules {
        if let (Some(&from_pos), Some(&to_pos)) = (position.get(from), position.get(to)) {
            if from_pos > to_pos {
                return false;
            }
        }
    }
    true
}

fn part1(rules: &Vec<(i32, i32)>, orders: &Vec<Vec<i32>>) -> i32 {
    orders
        .iter()
        .filter(|order| is_sorted(order, rules))
        .map(|order| order[order.len() / 2])
        .sum()
}

fn sort(mut order: Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    order.sort_by(|a, b| {
        if a == b {
            return Ordering::Equal;
        }
        if rules.iter().any(|rule| rule.0 == *a && rule.1 == *b) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    order
}

fn part2(rules: &Vec<(i32, i32)>, orders: &Vec<Vec<i32>>) -> i32 {
    orders
        .iter()
        .filter(|order| !is_sorted(order, rules))
        .map(|order| sort(order.clone(), rules)[order.len() / 2])
        .sum()
}

fn main() {
    let (rules, orders) = parse(&fs::read_to_string("input/input.txt").unwrap());
    println!("{}", part1(&rules, &orders));
    println!("{}", part2(&rules, &orders));
}

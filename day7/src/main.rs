use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let equations = parse(&input);
    println!("part 1: {:?}", part1(&equations));
    println!("part 2: {:?}", part2(&equations));
}

fn parse_ints(input: &str, delimiter: &str) -> Vec<i64> {
    input
        .split(delimiter)
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| line.split_once(":").unwrap())
        .map(|(result, values)| {
            (result.parse().unwrap(), parse_ints(values, " "))
        })
        .collect()
}

fn get_operator_combinations(n: usize, choices: &[String]) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    
    fn generate_combinations(
        current: &mut Vec<String>, 
        n: usize, 
        operators: &[String], 
        result: &mut Vec<Vec<String>>
    ) {
        if current.len() == n {
            result.push(current.clone());
            return;
        }
        for operator in operators {
            current.push(operator.clone());
            generate_combinations(current, n, operators, result);
            current.pop();
        }
    }

    generate_combinations(&mut Vec::new(), n, choices, &mut result);
    result
}

fn evaluate_expression(values: Vec<i64>, operators: Vec<String>) -> i64 {
    if values.is_empty() {
        return 0;
    }
    
    if operators.is_empty() {
        return values[0];
    }

    let mut result = values[0];

    for (i, operator) in operators.iter().enumerate() {
        let next_value = values[i + 1];
        result = match operator.as_str() {
            "+" => result + next_value,
            "*" => result * next_value,
            "||" => {
                let concatenated = result.to_string() + &next_value.to_string();
                concatenated.parse().unwrap_or(0)
            },
            _ => result
        };
    }

    result
}

fn part1(equations: &Vec<(i64, Vec<i64>)>) -> i64 {
    let mut sum = 0;
    for (result, values) in equations {
        let operators = get_operator_combinations(values.len() - 1, &["+".to_string(), "*".to_string()]);
        for operator in operators {
            let value = evaluate_expression(values.clone(), operator);
            if value == *result {
                sum += value;
                break;
            }
        }
        
    }
    sum
}

fn part2(equations: &Vec<(i64, Vec<i64>)>) -> i64 {
    equations
        .iter()
        .map(|(result, values)| {
            let operators = get_operator_combinations(values.len() - 1, &["+".to_string(), "*".to_string(), "||".to_string()]);
            for operator in operators {
                let value = evaluate_expression(values.clone(), operator);
                if value == *result {
                    return value;
                }
            }
            return 0
        })
        .sum()
}

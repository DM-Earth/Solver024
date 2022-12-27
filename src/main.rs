// A simple 24 game solver

pub mod base;
pub mod math;

use std::process;
use std::time::Instant;

use base::Component;
use base::Expression;

use base::SimpleComponent;
use math::permute;

fn main() {
    println!("Enter 4 numbers and split them using spaces.");

    let mut super_mode = false;
    let mut max: usize = 16;

    loop {
        let input = input_string();
        if input == "exit" || input == "quit" {
            break;
        }
        if input == "super" {
            super_mode = !super_mode;
            println!(
                "Super mode {}",
                if super_mode { "enabled" } else { "disabled" }
            );
            continue;
        }
        if input.starts_with("max ") {
            let max_str = input.split_whitespace().nth(1).unwrap();
            max = max_str.parse().unwrap();
            println!("Max solutions set to {}", max);
            continue;
        }
        let mut numbers: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let msg = "Invalid input, please enter 4 numbers between 1 and 13";
        for i in numbers.iter() {
            if *i < 1 || *i > 13 {
                println!("{}", msg);
                process::exit(1);
            }
        }
        if numbers.len() != 4 {
            println!("{}", msg);
            process::exit(1);
        }

        numbers.sort();

        let start = Instant::now();
        let solutions = solve(&numbers, &super_mode, &max);
        if solutions.len() == 0 {
            println!("No solutions found");
        } else {
            for (i, _item) in solutions.iter().enumerate() {
                println!("{}", solutions[i].to_string());
            }
            println!(
                "Found {} solutions in {} ms",
                solutions.len(),
                (start.elapsed().as_micros() as f64) * 0.001
            );
        }
    }
}

fn solve(numbers_raw: &Vec<i32>, super_mode: &bool, limit: &usize) -> Vec<Expression> {
    let nums = {
        let mut temp = Vec::new();
        for i in numbers_raw {
            temp.push(SimpleComponent::Number(*i));
        }
        temp
    };
    let mut solutions: Vec<Expression> = Vec::new();
    let mut operators = ['+', '-', '*', '/'].to_vec();
    if *super_mode {
        operators.append(&mut ['^', '>', '<', '|', '&'].to_vec());
    }
    for item in permute(nums) {
        for operator in operators.iter() {
            for operator1 in operators.iter() {
                for operator2 in operators.iter() {
                    let exp1 = Expression::create(
                        Component::create(item[0], *operator1, item[1]),
                        *operator,
                        Component::create(item[2], *operator2, item[3]),
                    );
                    let exp2 = Expression::create(
                        Component::of_simple(item[0]),
                        *operator,
                        Component::create(
                            item[1],
                            *operator1,
                            SimpleComponent::create(
                                item[2].get_num(),
                                *operator2,
                                item[3].get_num(),
                            ),
                        ),
                    );
                    if limit <= &1 {
                        if is_24(&exp1) {
                            return vec![exp1];
                        }
                        if is_24(&exp2) {
                            return vec![exp2];
                        }
                    } else {
                        if is_24(&exp1) {
                            push_if_absent(&mut solutions, exp1);
                        }
                        if is_24(&exp2) {
                            push_if_absent(&mut solutions, exp2);
                        }
                    }

                    if solutions.len() >= *limit {
                        return solutions;
                    }
                }
            }
        }
    }

    solutions
}

fn push_if_absent<T>(vec: &mut Vec<T>, item: T)
where
    T: PartialEq,
{
    if !vec.contains(&item) {
        vec.push(item);
    }
}

fn is_24(expression: &Expression) -> bool {
    match expression.calculate() {
        Ok(n) => (n as f64 - 24.0).abs() < 0.000001,
        Err(_) => false,
    }
}

fn input_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("read_line error!");
    input.lines().next().unwrap().to_string()
}

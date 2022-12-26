// A simple 24 game solver

pub mod base;
pub mod math;

use base::Component;
use base::Expression;

use base::SimpleComponent;
use math::permute;

fn main() {
    println!("Enter 4 numbers and split them using spaces, or use 'exit' 'quit' to quit: ");
    loop {
        let input = input_string();
        if input == "exit" || input == "quit" {
            break;
        }
        let mut numbers: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut exit = false;
        let msg = "Invalid input, please enter 4 numbers between 1 and 13";
        for i in numbers.iter() {
            if *i < 1 || *i > 13 {
                println!("{}", msg);
                exit = true;
                break;
            }
        }
        if numbers.len() != 4 {
            println!("{}", msg);
            exit = true;
        }
        if exit {
            continue;
        }

        numbers.sort();

        let solutions = solve(&numbers);

        if solutions.len() == 0 {
            println!("No solutions found");
        } else {
            for (i, _item) in solutions.iter().enumerate() {
                println!("{}", solutions[i].to_string());
            }
        }
    }
}

fn solve(numbers_raw: &Vec<i32>) -> Vec<Expression> {
    let nums = {
        let mut temp = Vec::new();
        for i in numbers_raw {
            temp.push(SimpleComponent::Number(i.clone()));
        }
        temp
    };
    let mut solutions: Vec<Expression> = Vec::new();
    let operators = ['+', '-', '*', '/'];
    for item in permute(nums) {
        for operator in operators {
            for operator1 in operators {
                for operator2 in operators {
                    push_if_absent(
                        &mut solutions,
                        Expression::create(
                            Component::create(item[0], operator1, item[1]),
                            operator,
                            Component::create(item[2], operator2, item[3]),
                        ),
                    );
                    push_if_absent(
                        &mut solutions,
                        Expression::create(
                            Component::of_simple(item[0]),
                            operator,
                            Component::create(
                                item[1],
                                operator1,
                                SimpleComponent::create(
                                    item[2].get_num(),
                                    operator2,
                                    item[3].get_num(),
                                ),
                            ),
                        ),
                    );
                }
            }
        }
    }

    let mut results: Vec<Expression> = Vec::new();
    for (i, _item) in solutions.iter().enumerate() {
        if is_24(&solutions[i]) {
            results.push(solutions[i].clone());
        }
    }

    results
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

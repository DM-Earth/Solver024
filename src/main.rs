// A simple 24 game solver

pub mod base;
pub mod math;
pub mod solver;

use std::process;
use std::time::Instant;

fn main() {
    println!("Enter 4 numbers and split them using spaces.");

    let mut super_mode = false;
    let mut max: usize = 16;

    loop {
        let input = input_string();
        if input == "exit" || input == "quit" {
            process::exit(1);
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

        let mut jump = false;
        let msg = "Invalid input, please enter 4 numbers between 1 and 13";
        for i in numbers.iter() {
            if *i < 1 || *i > 13 {
                println!("{}", msg);
                jump = true;
            }
        }
        if numbers.len() != 4 {
            println!("{}", msg);
            jump = true;
        }
        if input == "" {
            jump = true;
        }
        if jump {
            continue;
        }

        numbers.sort();
        let start = Instant::now();
        let solutions = solver::solve(&numbers, &super_mode, &max);
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

fn input_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("read_line error!");
    input.lines().next().unwrap().to_string()
}

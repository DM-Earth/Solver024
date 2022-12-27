use base::Component;
use base::Expression;

use base::SimpleComponent;
use math::permute;

use crate::base;
use crate::math;

pub fn solve(numbers_raw: &Vec<i32>, super_mode: &bool, limit: &usize) -> Vec<Expression> {
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
    for item1 in permute(nums) {
        let items = if *super_mode {
            math::roll_vec(item1.clone())
        } else {
            [item1.clone()].to_vec()
        };
        for operator in operators.iter() {
            for operator1 in operators.iter() {
                for operator2 in operators.iter() {
                    for item in items.iter() {
                        let mut exps = Vec::new();

                        exps.push(Expression::create(
                            Component::create(item[0], *operator1, item[1]),
                            *operator,
                            Component::create(item[2], *operator2, item[3]),
                        ));
                        exps.push(Expression::create(
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
                        ));

                        exps.push(Expression::create(
                            Component::create(
                                SimpleComponent::create(
                                    item[0].get_num(),
                                    *operator,
                                    item[1].get_num(),
                                ),
                                *operator1,
                                item[2],
                            ),
                            *operator2,
                            Component::of_simple(item[3]),
                        ));

                        exps.push(Expression::create(
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
                        ));

                        for exp in exps {
                            if limit <= &1 {
                                if is_24(&exp) {
                                    return vec![exp];
                                }
                            } else {
                                if is_24(&exp) {
                                    push_if_absent(&mut solutions, exp);
                                }
                            }
                        }

                        if solutions.len() >= *limit {
                            return solutions;
                        }
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

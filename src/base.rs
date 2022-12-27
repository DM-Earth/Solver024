use std::fmt::Debug;

use crate::math;

pub enum SimpleComponent {
    Number(i32),
    SimpleExp {
        number1: i32,
        number2: i32,
        op: char,
    },
}

impl SimpleComponent {
    pub fn reverse(self) -> SimpleComponent {
        match self {
            SimpleComponent::Number(n) => SimpleComponent::Number(-n),
            _ => self,
        }
    }

    pub fn create_simple(n: i32) -> SimpleComponent {
        SimpleComponent::Number(n)
    }

    pub fn create(n1: i32, op: char, n2: i32) -> SimpleComponent {
        SimpleComponent::SimpleExp {
            number1: n1,
            number2: n2,
            op: op,
        }
    }

    pub fn create_inv(n2: i32, op: char, n1: i32) -> SimpleComponent {
        Self::create(n1, op, n2)
    }

    pub fn get_num(&self) -> i32 {
        match self {
            SimpleComponent::Number(n) => *n,
            _ => -1,
        }
    }

    pub fn calculate(&self) -> Result<f64, &str> {
        match self {
            SimpleComponent::Number(n) => Ok(*n as f64),
            SimpleComponent::SimpleExp {
                number1,
                number2,
                op,
            } => match solve_simple(&(*number1 as f64), &(*number2 as f64), &op) {
                Some(n) => Ok(n),
                None => Err("Divide by zero"),
            },
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SimpleComponent::Number(n) => n.to_string(),
            SimpleComponent::SimpleExp {
                number1,
                number2,
                op,
            } => format!("({} {} {})", number1, map_op_string(op), number2),
        }
    }
}

impl PartialEq for SimpleComponent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SimpleComponent::Number(n1), SimpleComponent::Number(n2)) => n1 == n2,
            (
                SimpleComponent::SimpleExp {
                    number1: n11,
                    number2: n12,
                    op: op1,
                },
                SimpleComponent::SimpleExp {
                    number1: n21,
                    number2: n22,
                    op: op2,
                },
            ) => {
                if op1 == op2 && is_op_ordered(op1) {
                    return n11 == n21 && n12 == n22;
                }
                math::compare_arr(&[n11, n12], &[n21, n22]) && op1 == op2
            }
            _ => false,
        }
    }
}

impl Clone for SimpleComponent {
    fn clone(&self) -> SimpleComponent {
        match self {
            SimpleComponent::Number(n) => SimpleComponent::Number(*n),
            SimpleComponent::SimpleExp {
                number1,
                number2,
                op,
            } => SimpleComponent::SimpleExp {
                number1: *number1,
                number2: *number2,
                op: *op,
            },
        }
    }
}

impl Copy for SimpleComponent {}

impl Debug for SimpleComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(_arg0) => f.debug_tuple("Number").field(&self.to_string()).finish(),
            _ => f
                .debug_struct("SimpleExp")
                .field("exp", &self.to_string())
                .finish(),
        }
    }
}

pub enum Component {
    Single(SimpleComponent),
    Exp {
        number1: SimpleComponent,
        number2: SimpleComponent,
        op: char,
    },
}

impl Component {
    pub fn of_simple(n: SimpleComponent) -> Component {
        Component::Single(n)
    }

    pub fn create(n1: SimpleComponent, op: char, n2: SimpleComponent) -> Component {
        Component::Exp {
            number1: n1,
            number2: n2,
            op: op,
        }
    }

    pub fn create_inv(n2: SimpleComponent, op: char, n1: SimpleComponent) -> Component {
        Self::create(n1, op, n2)
    }

    pub fn get_component(&self) -> i32 {
        match self {
            Component::Single(n) => n.get_num(),
            _ => -1,
        }
    }

    pub fn calculate(&self) -> Result<f64, &str> {
        match self {
            Component::Single(n) => Ok(match n.calculate() {
                Ok(n) => n,
                _ => return Err("err"),
            } as f64),
            Component::Exp {
                number1,
                number2,
                op,
            } => match solve_simple(
                &(match number1.calculate() {
                    Ok(n) => n,
                    _ => return Err("err"),
                }),
                &(match number2.calculate() {
                    Ok(n) => n,
                    _ => return Err("err"),
                }),
                &op,
            ) {
                Some(n) => Ok(n),
                None => Err("Divide by zero"),
            },
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Component::Single(n) => n.to_string(),
            Component::Exp {
                number1,
                number2,
                op,
            } => format!(
                "({} {} {})",
                number1.to_string(),
                map_op_string(op),
                number2.to_string()
            ),
        }
    }
}

impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Component::Single(n1), Component::Single(n2)) => n1 == n2,
            (
                Component::Exp {
                    number1: n11,
                    number2: n12,
                    op: op1,
                },
                Component::Exp {
                    number1: n21,
                    number2: n22,
                    op: op2,
                },
            ) => {
                if op1 == op2 && is_op_ordered(op1) {
                    return n11 == n21 && n12 == n22;
                }
                math::compare_arr(&[n11, n12], &[n21, n22]) && op1 == op2
            }
            _ => false,
        }
    }
}

impl Clone for Component {
    fn clone(&self) -> Component {
        match self {
            Component::Single(n) => Component::Single(*n),
            Component::Exp {
                number1,
                number2,
                op,
            } => Component::Exp {
                number1: *number1,
                number2: *number2,
                op: *op,
            },
        }
    }
}

pub struct Expression {
    component1: Component,
    component2: Component,
    op: char,
}

impl Expression {
    pub fn create(c1: Component, op: char, c2: Component) -> Expression {
        Expression {
            op,
            component1: c1,
            component2: c2,
        }
    }

    pub fn create_inv(c1: Component, op: char, c2: Component) -> Expression {
        Self::create(c2, op, c1)
    }

    pub fn calculate(&self) -> Result<f64, &str> {
        let number1 = match self.component1.calculate() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        let number2 = match self.component2.calculate() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        match solve_simple(&number1, &number2, &self.op) {
            Some(n) => Ok(n),
            None => Err("Divide by zero"),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.component1.to_string(),
            map_op_string(&self.op),
            self.component2.to_string()
        )
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        if self.op == other.op && is_op_ordered(&self.op) {
            return self.component1.eq(&other.component1) && self.component2.eq(&other.component2);
        }
        math::compare_arr(
            &[&self.component1, &self.component2],
            &[&other.component1, &other.component2],
        ) && self.op == other.op
    }
}

impl Clone for Expression {
    fn clone(&self) -> Expression {
        Expression {
            op: self.op,
            component1: self.component1.clone(),
            component2: self.component2.clone(),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn solve_simple(number1: &f64, number2: &f64, op: &char) -> Option<f64> {
    match op {
        '+' => Some(number1 + number2),
        '-' => Some(number1 - number2),
        '*' => Some(number1 * number2),
        '/' => {
            if number2 == &0.0 {
                None
            } else {
                Some(number1 / number2)
            }
        }
        '^' => Some((*number1 as i32 ^ *number2 as i32).into()),
        '&' => Some((*number1 as i32 & *number2 as i32).into()),
        '|' => Some((*number1 as i32 | *number2 as i32).into()),
        '>' => match (*number1 as u32).checked_shr(*number2 as u32) {
            Some(n) => Some(n.into()),
            None => None,
        },
        '<' => match (*number1 as u32).checked_shl(*number2 as u32) {
            Some(n) => Some(n.into()),
            None => None,
        },
        _ => None,
    }
}

fn is_op_ordered(op: &char) -> bool {
    match op {
        '<' | '>' | '/' => true,
        _ => false,
    }
}

fn map_op_string(op: &char) -> String {
    match op {
        '>' => String::from(">>"),
        '<' => String::from("<<"),
        _ => op.to_string(),
    }
}

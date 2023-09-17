use crate::ast::{BinaryOp, Term, Value};

pub fn eval(term: Term) -> Value {
    match term {
        Term::Int(int) => Value::Int(int.value),
        Term::Str(str) => Value::Str(str.value),
        Term::Bool(bool) => Value::Bool(bool.value),
        Term::Print(print) => {
            let value = eval(*print.value);
            match value {
                Value::Int(int) => println!("{}", int),
                Value::Str(str) => println!("{}", str),
                Value::Bool(bool) => println!("{}", bool),
                _ => println!("Error"),
            };
            Value::Nil
        }
        Term::Binary(binary) => {
            let left = eval(*binary.lhs);
            let right = eval(*binary.rhs);
            match binary.op {
                BinaryOp::Add => match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left + right),
                    (Value::Str(left), Value::Str(right)) => Value::Str(left + &right),
                    (Value::Str(left), Value::Int(right)) => Value::Str(left + &right.to_string()),
                    (Value::Int(left), Value::Str(right)) => Value::Str(left.to_string() + &right),
                    _ => panic!("Error"),
                },
                BinaryOp::Sub => match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left - right),
                    _ => panic!("Error"),
                },
                BinaryOp::Mul => match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left * right),
                    _ => panic!("Error"),
                },
                BinaryOp::Div => match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left / right),
                    _ => panic!("Error"),
                },
                BinaryOp::Rem => match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left % right),
                    _ => panic!("Error"),
                },
                BinaryOp::Eq => match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left == right),
                    (Value::Str(left), Value::Str(right)) => Value::Bool(left == right),
                    (Value::Bool(left), Value::Bool(right)) => Value::Bool(left == right),
                    _ => panic!("Error"),
                },
                BinaryOp::Neq => match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left != right),
                    (Value::Str(left), Value::Str(right)) => Value::Bool(left != right),
                    (Value::Bool(left), Value::Bool(right)) => Value::Bool(left != right),
                    _ => panic!("Error"),
                },
                BinaryOp::Lt => match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left < right),
                    _ => panic!("Error"),
                },
                BinaryOp::Gt => match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left > right),
                    _ => panic!("Error"),
                },
                BinaryOp::Lte => match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left <= right),
                    _ => panic!("Error"),
                },
                BinaryOp::Gte => match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Bool(left >= right),
                    _ => panic!("Error"),
                },
                BinaryOp::And => match (left, right) {
                    (Value::Bool(left), Value::Bool(right)) => Value::Bool(left && right),
                    _ => panic!("Error"),
                },
                BinaryOp::Or => match (left, right) {
                    (Value::Bool(left), Value::Bool(right)) => Value::Bool(left || right),
                    _ => panic!("Error"),
                },
            }
        }
    }
}

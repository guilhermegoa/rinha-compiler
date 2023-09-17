mod ast;

use ast::{BinaryOp, File, Term, Value};
use std::{env, fs};

fn eval(term: Term) -> Value {
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
        Term::Binary(binary) => match binary.op {
            BinaryOp::Add => {
                let left = eval(*binary.lhs);
                let right = eval(*binary.rhs);
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left + right),
                    (Value::Str(left), Value::Str(right)) => Value::Str(left + &right),
                    (Value::Str(left), Value::Int(right)) => Value::Str(left + &right.to_string()),
                    (Value::Int(left), Value::Str(right)) => Value::Str(left.to_string() + &right),
                    _ => panic!("Error"),
                }
            }
            BinaryOp::Sub => {
                let left = eval(*binary.lhs);
                let right = eval(*binary.rhs);
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left - right),
                    _ => panic!("Error"),
                }
            }
            BinaryOp::Mul => {
                let left = eval(*binary.lhs);
                let right = eval(*binary.rhs);
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left * right),
                    _ => panic!("Error"),
                }
            }
            BinaryOp::Div => {
                let left = eval(*binary.lhs);
                let right = eval(*binary.rhs);
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left / right),
                    _ => panic!("Error"),
                }
            }
            BinaryOp::Rem => {
                let left = eval(*binary.lhs);
                let right = eval(*binary.rhs);
                match (left, right) {
                    (Value::Int(left), Value::Int(right)) => Value::Int(left % right),
                    _ => panic!("Error"),
                }
            }
        },
    }
}

fn main() {
    let file_path = env::args().nth(1).unwrap();

    let file_string = fs::read_to_string(file_path).unwrap();
    let json: File = serde_json::from_str(&file_string).unwrap();

    let term = json.expression;

    eval(term);
}

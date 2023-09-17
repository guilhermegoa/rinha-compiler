mod ast;

use ast::{File, Term, Value};
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
    }
}

fn main() {
    let file_path = env::args().nth(1).unwrap();

    let file_string = fs::read_to_string(file_path).unwrap();
    let json: File = serde_json::from_str(&file_string).unwrap();

    let term = json.expression;

    eval(term);
}

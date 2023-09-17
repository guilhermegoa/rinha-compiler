mod ast;
mod interpreter;

use std::{env, fs};

use ast::File;
use interpreter::eval;

fn main() {
    let file_path = env::args().nth(1).unwrap();
    let file_string = fs::read_to_string(file_path).unwrap();

    let json: File = serde_json::from_str(&file_string).unwrap();

    let term = json.expression;

    eval(term);
}

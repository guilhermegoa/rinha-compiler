mod ast;
mod interpreter;

use std::{collections::HashMap, io::stdin, io::Read};

use ast::File;
use interpreter::eval;

fn main() {
    //     let file_path = env::args().nth(1).unwrap();
    //     let file_string = fs::read_to_string(file_path).unwrap();

    let mut file_string = String::new();
    stdin().lock().read_to_string(&mut file_string).unwrap();

    let json: File = serde_json::from_str(&file_string).unwrap();

    let term = json.expression;

    let mut context = HashMap::new();

    eval(term, &mut context);
}

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct File {
    name: String,
    pub expression: Term,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Location {
    start: i32,
    end: i32,
    filename: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Print {
    pub value: Box<Term>,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Str {
    pub value: String,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Int {
    pub value: i32,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Bool {
    pub value: bool,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: BinaryOp,
    pub rhs: Box<Term>,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct If {
    pub condition: Box<Term>,
    pub then: Box<Term>,
    pub otherwise: Box<Term>,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Var {
    pub text: String,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Parameter {
    pub text: String,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Let {
    pub name: Parameter,
    pub value: Box<Term>,
    pub next: Box<Term>,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Function {
    pub parameters: Vec<Parameter>,
    pub value: Box<Term>,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Call {
    pub callee: Box<Term>,
    pub arguments: Vec<Term>,
    location: Location,
}

//ENUNS

#[derive(Deserialize, Clone)]
pub enum BinaryOp {
    Add, // Soma	            3 + 5 = 8, "a" + 2 = "a2", 2 + "a" = "2a", "a" + "b" = "ab"
    Sub, // Subtração	        0 - 1 = -1
    Mul, // Multiplicação	    2 * 2 = 4
    Div, // Divisão	            3 / 2 = 1
    Rem, // Resto da divisão	4 % 2 = 0
    Eq,  // Igualdade	        "a" == "a", 2 == 1 + 1, true == true
    Neq, // Diferente	        "a" != "b", 3 != 1 + 1, true != false
    Lt,  // Menor	            1 < 2
    Gt,  // Maior	            2 > 3
    Lte, // Menor ou igual	    1 <= 2
    Gte, // Maior ou igual	    1 >= 2
    And, // Conjunção	        true && false
    Or,  // Disjunção           true || false
}

#[derive(Deserialize, Clone)]
pub enum Value {
    Int(i32),
    Str(String),
    Bool(bool),
    Closure(Vec<Parameter>, Term),
    Nil,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    Bool(Bool),
    Print(Print),
    Binary(Binary),
    If(If),
    Let(Let),
    Var(Var),
    Function(Function),
    Call(Call),
}

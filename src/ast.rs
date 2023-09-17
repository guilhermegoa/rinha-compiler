use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct File {
    name: String,
    pub expression: Term,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Location {
    start: i32,
    end: i32,
    filename: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Print {
    pub value: Box<Term>,
    location: Location,
}

//VALUES

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Str {
    pub value: String,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Int {
    pub value: i32,
    location: Location,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Bool {
    pub value: bool,
    location: Location,
}

//ENUNS

#[derive(Deserialize)]
pub enum Value {
    Int(i32),
    Str(String),
    Bool(bool),
    Nil,
}

#[derive(Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    Bool(Bool),
    Print(Print),
}

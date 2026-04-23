use std::collections::HashMap;

use crate::parser::Parser;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(text: String) {
        let mut map = HashMap::new();

        for line in text.lines() {
            let mut parser = Parser::new(line);
            let expr = parser.parse_expression();

            match expr {
                ExpressionType::Var(v) => {
                    map.insert(v.identifier.clone(), v);
                }
                ExpressionType::Print(se) => {
                    if !se.var.as_ref().unwrap().is_empty() {
                        let val = map.get(&se.var.clone().unwrap());
                        if let Some(var) = val {
                            println!("{}{}{}", se.head.unwrap(), var.data, se.tail.unwrap())
                        } else {
                            panic!("variable {} not found", se.var.unwrap())
                        }
                    } else {
                        println!("{}", se.head.unwrap())
                    }
                }
                ExpressionType::Unrecognized => panic!("error in code"),
            }
        }
    }
}
#[derive(Debug)]
pub enum Modifier {
    Let,
}
#[derive(Debug)]
pub enum DataType {
    String,
}

#[derive(Debug)]
pub struct Variable {
    pub modifier: Modifier,
    pub data_type: DataType,
    pub identifier: String,
    pub data: String,
}

#[derive(Debug)]
pub enum ExpressionType {
    Var(Variable),
    Print(StringExpression),
    Unrecognized,
}

#[derive(Debug)]
pub struct StringExpression {
    pub head: Option<String>,
    pub var: Option<String>,
    pub tail: Option<String>,
}

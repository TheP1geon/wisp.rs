pub mod ast;
pub mod lexer;
pub mod native;
pub mod parser;
pub mod token;
pub mod value;

use std::collections::HashMap;
use crate::value::Value;
use crate::ast::Node;

// use crate::{lexer::Lexer, token::TokenKind};

fn main() {
    // let mut lexer = Lexer::new("This is ()     \"a\" string");
    //
    // let mut tok = lexer.next_token();
    // while tok.is_ok() && tok.as_ref().unwrap().kind != TokenKind::Eof {
    //     println!("{:?}", tok);
    //
    //     tok = lexer.next_token();
    // }

    let mut env: HashMap<&str, Value> = HashMap::new();
    env.insert("add", Value::Native(native::add));
    // env.insert("add", Value::Number(1.));

    let root = Node::Block(vec![Box::new(Node::List(vec![
        Box::new(Node::Symbol(String::from("add"))),
        Box::new(Node::Number(32.)),
        Box::new(Node::Number(10.)),
    ]))]);

    println!("{:?}", root.eval(&env));
}

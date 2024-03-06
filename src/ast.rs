use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug)]
pub enum Node {
    Number(f32),
    Symbol(String),
    String(String),
    List(Vec<Box<Node>>),
    Block(Vec<Box<Node>>),
    Nil,
}

impl Node {
    pub fn eval(&self, env: &HashMap<&str, Value>) -> Value {
        match self {
            Node::Block(program) => {
                let mut val: Value = Value::Nil;
                for node in program {
                    val = node.eval(env);
                }

                return val;
            }

            Node::List(list) => {
                if list.len() == 0 {
                    return Value::List(vec![]);
                }

                let sym_val = list[0].eval(env);

                let mut args: Vec<Value> = Vec::new();

                for i in 1..list.len() {
                    args.push(list[i].eval(env));
                }

                return match sym_val {
                    Value::Native(func) => func(args),
                    _ => {
                        args.insert(0, sym_val);
                        Value::List(args)
                    }
                };
            }

            Node::String(str) => return Value::String(str.to_string()),
            Node::Number(num) => return Value::Number(*num),
            Node::Nil => return Value::Nil,
            Node::Symbol(sym) => {
                if env.contains_key(sym.as_str()) {
                    return env.get(sym.as_str()).unwrap().clone();
                }

                return Value::Nil;
            }
        }
    }
}

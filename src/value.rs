type NativeFn = fn(Vec<Value>) -> Value;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Number(f32),
    String(String),
    Symbol(String),
    Native(NativeFn),
    List(Vec<Value>),
}

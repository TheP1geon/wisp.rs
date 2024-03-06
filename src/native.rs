use crate::value::Value;

pub fn add(args: Vec<Value>) -> Value {
    let mut sum = 0f32;

    for val in args {
        sum += match val {
            Value::Number(num) => num,
            _ => 0f32,
        };
    }

    return Value::Number(sum);
}

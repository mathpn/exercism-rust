#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(value) => stack.push(*value),
            _ => {
                if stack.len() < 2 {
                    return None;
                }
                parse_element(&mut stack, input);
            }
        }
    }
    return if stack.len() == 1 { stack.pop() } else { None }
}

fn parse_element(stack: &mut Vec<i32>, input: &CalculatorInput) -> () {
    match input {
        CalculatorInput::Add => perform_operation(stack, |a, b| a + b),
        CalculatorInput::Subtract => perform_operation(stack, |a, b| b - a),
        CalculatorInput::Multiply => perform_operation(stack, |a, b| a * b),
        CalculatorInput::Divide => perform_operation(stack, |a, b| b / a),
        _ => (),
    }
}

fn perform_operation(stack: &mut Vec<i32>, f: impl Fn(i32, i32) -> i32) -> () {
    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();
    stack.push(f(a, b));
}
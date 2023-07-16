fn main() {
    
    // let operator: Operation = Operation::Add;
    // let operator: Operation = Operation::Subtract;
    // let operator: Operation = Operation::Multiply;
    let operator: Operation = Operation::Divide;
    
    let value1: f64 = 93.0;
    let value2: f64 = 10.0;

    match calculate(operator, value1, value2) {
        Some(result) => println!("The result is: {}", result),
        None => println!("Warning: Division by zero is impossible ! Please check your second value."),
    }
    
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(operator: Operation, value1: f64, value2: f64) -> Option<f64> {
    match operator {
        Operation::Add => Some(value1 + value2),
        Operation::Subtract => Some(value1 - value2),
        Operation::Multiply => Some(value1 * value2),
        Operation::Divide => {
            if value2 != 0.0 {
                Some(value1 / value2)
            } else {
                None
            }
        }
    }
}
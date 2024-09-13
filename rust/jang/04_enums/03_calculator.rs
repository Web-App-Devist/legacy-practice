// #[derive(Debug, PartialEq)]
// enum Operation {
//     Add,
//     Sub,
//     Div,
//     Multi,
// }
//
// fn calculate(expression: &str) -> Option<f64> {
//     // Remove any whitespace from the expression
//     let expression = expression.replace(" ", "");
//
//     // Parse the expression into tokens
//     let tokens: Vec<&str> = expression
//         .split(|c| c == '+' || c == '-' || c == '*' || c == '/')
//         .collect();
//     let mut operators: Vec<char> = expression
//         .chars()
//         .filter(|c| *c == '+' || *c == '-' || *c == '*' || *c == '/')
//         .collect();
//
//     // Convert tokens to floats
//     let mut numbers: Vec<f64> = vec![];
//     for token in tokens {
//         if let Ok(num) = token.parse::<f64>() {
//             numbers.push(num);
//         } else {
//             return None; // Invalid number format
//         }
//     }
//
//     // Helper function to perform a single operation
//     fn apply_operation(op: Operation, a: f64, b: f64) -> f64 {
//         match op {
//             Operation::Add => a + b,
//             Operation::Sub => a - b,
//             Operation::Multi => a * b,
//             Operation::Div => a / b,
//         }
//     }
//
//     // Evaluate multiplication and division first
//     let mut i = 0;
//     while i < operators.len() {
//         if operators[i] == '*' || operators[i] == '/' {
//             let op = if operators[i] == '*' {
//                 Operation::Multi
//             } else {
//                 Operation::Div
//             };
//             let result = apply_operation(op, numbers[i], numbers[i + 1]);
//             numbers[i] = result;
//             numbers.remove(i + 1);
//             operators.remove(i);
//         } else {
//             i += 1;
//         }
//     }
//
//     // Evaluate addition and subtraction
//     let mut result = numbers[0];
//     for i in 0..operators.len() {
//         let op = if operators[i] == '+' {
//             Operation::Add
//         } else {
//             Operation::Sub
//         };
//         result = apply_operation(op, result, numbers[i + 1]);
//     }
//
//     Some(result)
// }
//
// fn main() {
//     // Example usage
//     let expression = "5+3*4-2/5+4/2";
//     if let Some(result) = calculate(expression) {
//         println!("Result of {} = {}", expression, result);
//     } else {
//         println!("Invalid expression!");
//     }
// }

#[derive(Debug)]
enum Operations {
    ADD,
    DIFFERENCE,
    DIVISION,
    PRODUCT,
}

impl Operations {
    fn run(&self, x: f64, y: f64) -> f64 {
        match self {
            Operations::ADD => x + y,
            Operations::DIFFERENCE => x - y,
            Operations::DIVISION => x / y,
            Operations::PRODUCT => x * y,
        }
    }
}

#[derive(Debug)]
struct Calculator {
    expression: String,
}

impl Calculator {
    fn new(expression: String) -> Calculator {
        Calculator {
            expression: expression,
        }
    }

    fn calculate(&self) -> Option<f64> {
        let mut operators: Vec<char> = self
            .expression
            .chars()
            .filter(|c| *c == '+' || *c == '-' || *c == '*' || *c == '/')
            .collect();

        let mut numbers: Vec<f64> = self
            .expression
            .split(|c| c == '+' || c == '-' || c == '*' || c == '/')
            .map(|num| num.trim().parse::<f64>().unwrap())
            .collect();

        let mut i = 0;
        while i < operators.len() {
            let result = match operators[i] {
                '/' => Operations::DIVISION.run(numbers[i], numbers[i + 1]),
                '*' => Operations::PRODUCT.run(numbers[i], numbers[i + 1]),
                _ => numbers[i],
            };

            if operators[i] == '*' || operators[i] == '/' {
                numbers[i] = result;
                numbers.remove(i + 1);
                operators.remove(i);
            }
            i += 1;
        }

        i = 0;
        while i < operators.len() {
            let result = match operators[i] {
                '-' => Operations::DIFFERENCE.run(numbers[i], numbers[i + 1]),
                '+' => Operations::ADD.run(numbers[i], numbers[i + 1]),
                _ => 0.0,
            };

            numbers[i] = result;
            numbers.remove(i + 1);
            operators.remove(i);
        }

        Some(numbers[0])
    }
}

fn main() {
    let expression = String::from("5+3*4-2/5+4/2");
    let calcy = Calculator::new(expression.clone());

    if let Some(result) = calcy.calculate() {
        println!("Result of {} = {}", expression, result);
    } else {
        println!("Invalid expression!");
    }
}

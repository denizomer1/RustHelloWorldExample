use std::io;

// Define the Operation enum
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Function to calculate based on the Operation enum
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                eprintln!("Error: Division by zero is not allowed.");
                std::f64::NAN
            }
        }
    }
}

fn main() {
    // Helper function to read input
    fn read_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    // Read the first number
    let first_number: f64 = read_input("Enter the first number:")
        .parse()
        .expect("Invalid input for the first number.");

    // Read the operation
    let operation = read_input("Enter the operation (+, -, *, /):");

    // Read the second number
    let second_number: f64 = read_input("Enter the second number:")
        .parse()
        .expect("Invalid input for the second number.");

    // Create an Operation enum instance based on user input
    let op = match operation.as_str() {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            eprintln!("Invalid operation.");
            return;
        }
    };

    // Calculate and print the result
    let result = calculate(op);
    if !result.is_nan() {
        println!("The result is: {}", result);
    }
}

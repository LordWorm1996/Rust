use std::io::{self, Write};

fn main () {
    println!("Enter an operator (+ - * /): ");

    io::stdout().flush().unwrap();
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).unwrap();

    let operator = operator.trim();

    let mut input = String::new();
    println!("Give me two numbers: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut values = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Parse error"));
    let num1 = values.next().expect("Expected a value for a");
    let num2 = values.next().expect("Expected a value for b");
    calculator(num1, num2, operator);
}

fn calculator (num1: i32, num2: i32, operator: &str) {
    match operator {
        "+" => println!("The result of {num1} {operator} {num2} is {}", num1 + num2),
        "-" => println!("The result of {num1} {operator} {num2} is {}", num1 - num2),
        "*" => println!("The result of {num1} {operator} {num2} is {}", num1 * num2),
        "/" => {
            let num1: f64 = num1 as f64;
            let num2: f64 = num2 as f64;
            if num2 == 0.0 {println!("The second number cannot be 0 in a division, please try again!");}
            else {println!("The result of {num1} {operator} {num2} is {:.2}", num1 / num2)}
        }
            _ => println!("Invalid operator entered, please try again!"),
        }
}
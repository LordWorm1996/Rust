use std::io;

fn main() {
    let mut input = String::new();
    println!("Give me a, b, c for the quadratic equation: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut values = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<f64>().expect("Parse error"));
    let a = values.next().expect("Expected a value for a");
    let b = values.next().expect("Expected a value for b");
    let c = values.next().expect("Expected a value for c");
    quadratic(a, b, c);
}
fn quadratic(a: f64, b: f64, c: f64) {
    if a == 0.0 {
        if b != 0.0 {
            let x = -c / b;
            println!("This is a linear equation. The solution is x = {x}");
        } else {
            println!("No solution, as both a and b are zero.");
        }
    } else {
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let sqrt_d = discriminant.sqrt();
            let x1 = (-b + sqrt_d) / (2.0 * a);
            let x2 = (-b - sqrt_d) / (2.0 * a);
            println!("Two real solutions: x1 = {x1}, x2 = {x2}");
        } else if discriminant == 0.0 {
            let x = -b / (2.0 * a);
            println!("One real solution: x = {x}");
        } else {
            // Complex roots case
            let sqrt_d = (-discriminant).sqrt();
            let real_part = -b / (2.0 * a);
            let imaginary_part = sqrt_d / (2.0 * a);
            println!("Two complex solutions: x1 = {real_part:.2} + {imaginary_part}i, x2 = {real_part} - {imaginary_part}i");
        }
    }
}
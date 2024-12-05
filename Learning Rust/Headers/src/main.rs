mod custom_functions;
use std::io;

fn main() {
    let num1 = custom_functions::intMax(3, 6);
    println!("Max num1: {num1}");
    let num2 = custom_functions::intMin(9, 6);
    println!("Min num2: {num2}");
    let num3 = custom_functions::floatMax(5.34, 3.345);
    println!("Max num3: {num3}");
    let num4 = custom_functions::floatMin(0.34, 9.345);
    println!("Min num4: {num4}");
    let arr1 = [1, 2, 3, 6, 43, 23, 214, 34576, 123, 74755];

    let mut input_vec: Vec<f64> = Vec::new();

        println!("Enter numbers one by one. Type 'x' to stop.");

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();

            if input == "x" {
                break;
            }

            match input.parse::<f64>() {
                Ok(num) => {
                    input_vec.push(num);
                }
                Err(_) => {
                    println!("Invalid input! Please enter a valid number or 'x' to stop.");
                }
            }
        }

    let a1 = custom_functions::intArrayMax(&arr1);
    println!("Max arr1: {a1}");
    let a2 = custom_functions::intArrayMin(&arr1);
    println!("Min arr1: {a2}");
    //let arr2 = [1.345, 2.00, 3.1234, 43546.2456, 5.2456, 6.2465, 0.456, -4657.00, -456.678];
    let a3 = custom_functions::floatArrayMax(&input_vec);
    println!("Max arr2: {a3}");
    let a4 = custom_functions::floatArrayMin(&input_vec);
    println!("Min arr2: {a4}");
}
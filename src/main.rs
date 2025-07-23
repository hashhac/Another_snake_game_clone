use std::any::type_name;
use std::f32::consts::E;
fn main() {
    println!("Hello, world!");
    calculations();
    listeners();
    tuple_test();
    let name : &str = "Jonathon";
    let age : u8 = 25;
    let last_name : &str = "Moviegoer";
    let data = (name, age, last_name);
    let data_array: [&str; 3] = ["Jonathon", "25", "Moviegoer"];
    println!("{:?}", data);
    println!("{:?}", data_array);
    let piont: Piont = Piont{x: 5.2, y: 5.2};
    println!("point coordinates: ({}, {})", piont.x, piont.y);
    let add_result = calculator(MathOperation::Addition, 5.0, 3.0);
    println!("5.0 + 3.0 = {}", add_result); // Output: 8.0

    let sin_result = calculator(MathOperation::Sin, std::f32::consts::PI / 2.0, 0.0); // num2 is ignored
    println!("sin(PI/2) = {}", sin_result); // Output: 1.0

    let power_result = calculator(MathOperation::Power, 2.0, 3.0);
    println!("2.0 ^ 3.0 = {}", power_result); // Output: 8.0

    let sqrt_result = calculator(MathOperation::SquareRoot, 9.0, 0.0); // num2 is ignored
    println!("sqrt(9.0) = {}", sqrt_result); // Output: 3.0

    let div_zero = calculator(MathOperation::Division, 5.0, 0.0);
    println!("5.0 / 0.0 = {}", div_zero); // Output: Error message + NaN

    let neg_sqrt = calculator(MathOperation::SquareRoot, -4.0, 0.0);
    println!("sqrt(-4.0) = {}", neg_sqrt); // Output: Warning message + NaN
}



fn calculations(){
    let x: u32 = 25;
    let y: i32 = -25;
    let z: i32 = x as i32 * y; 
    println!("The values of the 2 integers where the first one is signed {} {} {}",x,y, z);
    // let's do some more examples with char
    for c in 'a'..='z' {
        print!("{} ", c);
    }
    println!();
    
}

fn listeners(){
    let x: [i32; 5] = [1, 2, 3, 4,5 ];
    let fruit: [&str; 5] = ["apple","banna", "orange", "pineapple", "watermelon"];
    println!("{:?}", x);
    println!("{:?}", fruit);
}

//tuple testing code
fn tuple_test(){
    let  alice : &str = "Alice";
    let x: (&str, bool, u128) = (alice,true,1);
    let x = String::from("hello");
    println!("{:?}", x);
    println!("{:?}", x);
    println!(" the string slice is {} the type is {}", &x[0..2], std::any::type_name::<&str>());
}

struct Piont{
    x: f32,
    y: f32
}



// Renamed variants for clarity
#[derive(Debug, Clone, Copy)] // Added derive for easier use (like in panic messages)
enum MathOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    SquareRoot, // Changed from 'root'
    Power,      // Changed from 'power'
    Log10,      // Changed from 'log', specified base 10
    NaturalLog, // Changed from 'ln' for clarity
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Exponent, // Changed from 'e', represents exp(x) or e^x
}

// Renamed function, added basic error handling for division by zero
fn calculator(operation: MathOperation, num1: f32, num2: f32) -> f32 {
    // Match against the input 'operation' variable
    match operation {
        MathOperation::Addition => num1 + num2,
        MathOperation::Subtraction => num1 - num2,
        MathOperation::Multiplication => num1 * num2,
        MathOperation::Division => {
            if num2 == 0.0 {
                // Decide how to handle division by zero. Returning NaN is common.
                println!("Error: Division by zero!");
                f32::NAN // Not a Number
            } else {
                num1 / num2
            }
        }
        // Unary operations: Ignore num2
        MathOperation::SquareRoot => {
            if num1 < 0.0 {
                println!("Warning: Square root of negative number!");
                f32::NAN // Or handle as desired
            } else {
                num1.sqrt()
            }
        }
        MathOperation::Power => num1.powf(num2), // Correctly uses num1 and num2
        MathOperation::Cos => num1.cos(),
        MathOperation::Sin => num1.sin(),
        MathOperation::Tan => num1.tan(),
        MathOperation::Asin => num1.asin(),
        MathOperation::Acos => num1.acos(),
        MathOperation::Atan => num1.atan(),
        MathOperation::NaturalLog => {
            if num1 <= 0.0 {
                println!("Error: Natural logarithm requires positive input!");
                f32::NAN
            } else {
                num1.ln()
            }
        }
        MathOperation::Log10 => {
            if num1 <= 0.0 {
                println!("Error: Base-10 logarithm requires positive input!");
                f32::NAN
            } else {
                num1.log10()
            }
        }
        MathOperation::Exponent => num1.exp(), // Calculates e^num1
    }
    // No catch-all needed - the compiler ensures all variants are covered.
    // If you add a variant to MathOperation later, the compiler will give
    // an error here until you add a matching arm.
}


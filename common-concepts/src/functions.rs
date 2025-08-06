// Functions in Rust

fn main() {
    println!("Hello, Rustando!");

    another_function(); // Call to another function
}

fn another_function() {
    println!("This is another function.");
}

// Parameters

fn function_with_params(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// Statements and Expressions

fn statements_and_expressions() {
    let y = 5; // This is a statement
    let z = {
        let x = 3; // This is an expression
        x + 1 // The value of this expression is returned
    };
    println!("The value of y is: {}, and z is: {}", y, z);
}

// Functions with Return Values

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main_with_return() {
    let result = plus_one(5);
    println!("The result is: {}", result);
}

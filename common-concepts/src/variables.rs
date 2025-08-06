// Variables and Mutability in Rust
fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);
    // compile with error: cannot assign twice to immutable variable `x`
    x = 6;
    println!("The value of x is: {}", x);

    // Mutable variable
    let mut y = 10;
    y = 15;
    println!("The value of y is: {}", y);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    const PI: f64 = 3.14159;
    const SUBSCRIBER_COUNT: u32 = 1_000_000;
    println!(
        "Max points: {}, PI: {}, Subscriber count: {}",
        MAX_POINTS, PI, SUBSCRIBER_COUNT
    );

    // Shadowing
    let z = 20;
    println!("The value of z is: {}", z);
    let z = z + 5; // shadowing the previous value
    println!("The new value of z is: {}", z);

    // Inner scope shadowing
    {
        let z = 25; // shadowing in an inner scope
        println!("The value of z in the inner scope is: {}", z);
    }

    let z = "Now I'm a string"; // shadowing with a different type
    println!("The value of z is now: {}", z);

    let spaces = "   ";
    let spaces = spaces.len(); // shadowing to change type from &str to usize

    // Compile with error: mismatched types
    let mut spaces = "   ";
    spaces = spaces.len(); // shadowing to change type from &str to usize
    println!("The length of spaces is: {}", spaces);
}

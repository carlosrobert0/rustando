// Data types in Rust

fn main() {
    // Scalar Types
    // Integers
    let signed: i32 = -42; // Signed integer
    let unsigned: u32 = 42; // Unsigned integer
    let isize: isize = 100; // Size depends on the architecture (32 or 64 bits)
    let usize: usize = 100; // Size depends on the architecture (32 or 64 bits)

    let decimal: i32 = 98_222; // Decimal integer
    let hex: i32 = 0xff; // Hexadecimal integer
    let octal: i32 = 0o77; // Octal integer
    let binary: i32 = 0b1111_0000; // Binary integer
    let byte: u8 = b'A'; // Byte (ASCII character)

    // Operations on integers
    let sum = 5 + 10; // Addition
    let difference = 95.5 - 4.3; // Subtraction
    let product = 4 * 30; // Multiplication
    let quotient = 56.7 / 32.2; // Division
    let remainder = 43 % 5; // Remainder

    // Floating-point Numbers
    let float: f32 = 3.14; // 32-bit floating point
    let double: f64 = 2.7182818284; // 64-bit floating point

    // Booleans
    let is_true = true; // Boolean value
    let is_false: bool = false; // Boolean value

    // Characters
    let letter: char = 'A'; // Single character
    let emoji: char = '😊'; // Unicode character

    // Compound Types

    // Tuples
    let tuple: (&str, u32) = ("Rustando", 10_000);
    let (channel, subscribers) = tuple; // Destructuring a tuple
    println!("Channel: {}, Subscribers: {}", channel, subscribers);

    let channel = tuple.0; // Accessing tuple elements by index
    let subscribers = tuple.1; // Accessing tuple elements by index

    println!("Channel: {}, Subscribers: {}", channel, subscribers);

    // Arrays
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // Fixed-size array
    let first_element = array[0]; // Accessing array elements by index
    let second_element = array[1];
    println!("First: {}, Second: {}", first_element, second_element);

    let mut mutable_array: [i32; 5] = [1, 2, 3, 4, 5]; // Mutable array
    mutable_array[0] = 10; // Modifying an element
    println!("Modified array: {:?}", mutable_array);

    // Slices
    let slice: &[i32] = &array[1..4]; // Slice of an array
    println!("Slice: {:?}", slice);

    // Strings
    let string: String = String::from("Hello, Rust!"); // Owned string
    let str_slice: &str = "Hello, Rust!"; // String slice (borrowed string)
    let mut mutable_string: String = String::from("Hello"); // Mutable string
    mutable_string.push_str(", Rust!"); // Appending to a string
    println!("String: {}", mutable_string);

    // String concatenation
    let concatenated = string + " is awesome!"; // Concatenating strings
    println!("Concatenated String: {}", concatenated);

    // String formatting
    let formatted_string = format!("{} has {} subscribers", channel, subscribers);
    println!("Formatted String: {}", formatted_string);

    // Type Aliases
    type Kilometers = i32; // Type alias for i32
    let distance: Kilometers = 100; // Using the type alias
    println!("Distance: {} km", distance);

    // Type Inference
    let inferred_integer = 42; // Type inferred as i32
    let inferred_float = 3.14; // Type inferred as f64
    println!(
        "Inferred Integer: {}, Inferred Float: {}",
        inferred_integer, inferred_float
    );

    // End of program
    println!("All types demonstrated successfully!");
}

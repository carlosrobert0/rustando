// Control Flow in Rust

fn main() {
    let number = 5;

    // if-else statement
    if number < 10 {
        println!("The number is less than 10");
    } else {
        println!("The number is 10 or greater");
    }

    // This will not compile because `number` is not a boolean

    if number {
        println!("This will not compile because `number` is not a boolean");
    }

    // if-else with a condition
    if number != 0 {
        println!("The number is not zero");
    } else {
        println!("The number is zero");
    }

    // Handling multiple conditions
    if number < 0 {
        println!("The number is negative");
    } else if number == 0 {
        println!("The number is zero");
    } else {
        println!("The number is positive");
    }

    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 10 }; // This will compile
    println!("The value of number is: {}", number);

    // This will not compile because types do not match
    let number = if condition { 5 } else { "ten" }; // This will not compile because types do not match

    // Repetition with loops
    // Looping with loop
    let mut count = 0;

    loop {
        count += 1;
        if count == 5 {
            break; // Exit the loop when count is 5
        }
        println!("Count: {}", count);
    }

    // Returning values from loops
    let mut count = 0;

    let result = loop {
        count += 1;
        if count == 5 {
            break count * 2; // Return double the count when it reaches 5
        }
    };

    // Loops Labels to Disambiguate Between Multiple Loops

    let mut outer_count = 0;

    'outer: loop {
        let mut inner_count = 0;

        loop {
            inner_count += 1;
            if inner_count == 3 {
                break; // Break the inner loop
            }
            if outer_count == 2 {
                break 'outer; // Break the outer loop
            }
        }
        outer_count += 1;
    }

    // Looping with a while loop
    let mut count = 0;
    while count < 5 {
        count += 1;
        println!("Count: {}", count);
    }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1; // Decrement the number
    }

    // Looping through a collection with for loop
    let numbers = [1, 2, 3, 4, 5];

    // while loop
    let mut index = 0;
    while index < 5 {
        println!("Number: {}", numbers[index]);
        index += 1; // Increment the index
    }

    // for loop
    for number in numbers.iter() {
        println!("Number: {}", number);
    }

    // for loop with range
    for number in 1..6 {
        println!("Number: {}", number);
    }
}


// this is a declarative macro that takes a name as an argument and prints a greeting message to the console.
macro_rules! log {
    ($log:expr) => {
        println!("[LOG]: {}", $log);
    };
}

fn main() {
    // This is a simple Rust program that prints "Hello, world!" to the console.

    /*
     * This is a simple Rust program that prints "Hello, world!" to the console.
     */

    println!("Hello, world!");

    /* a variable without mut is immutable, so it cannot be changed later in the program. Init _ as convention to indicate 
    that the variable is not used. */
    let _y = 10;

    // mut is for making a variable mutable, so that it can be changed later in the program.
    let mut x = 5;
    println!("x = {}", x);

    x = 6; 

    println!("x = {}", x);

    log!("This is an application log message");

    //CONTROL FLOW (If, else-if, else)

    let x = 5;
    let y = 10;

    if x > y {
        println!("X is greater than Y");
    } else if x < y {
        println!("X is less than Y");
    } else {
        println!("X is equal to Y");
    }

    let z = if x > y { x } else { y };
    println!("Z = {}", z);

    //LOOP
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    }; 

    println!("The result is {}", result);

    //LOOP LABELS
    let mut outer_count = 0;

    'outer: loop {
        println!("Outer count: {}", outer_count);
        let mut inner_count = 0;

        'inner: loop {
            println!("Inner count: {}", inner_count);
             inner_count += 1;

            if inner_count == 5 {
                break 'inner;
            }

            if outer_count == 3 {
                break 'outer;
            }
        }

        println!("---------------------------------");
        outer_count += 1;
    }

    println!("Outer loop ended with outer_count = {}", outer_count);

    //FOR LOOP
    let numbers = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("The number is: {}", number);
    }

    //WHILE LOOP
    let mut count = 0;

    while count < 5 {
        println!("Count is: {}", count);
        count += 1;
    }

    println!("Count has reached 5, exiting the loop.");

    //USE BREAK AND CONTINUE IN WHILE LOOP
    let mut count = 0;

    while count < 10 {
        count += 1;

        if count % 2 == 0 {
            continue; // Skip even numbers
        }

        println!("Odd number: {}", count);

        if count >= 7 {
            break; // Exit the loop when count reaches 7
        }
    }

    println!("Exited the loop after reaching an odd number greater than or equal to 7.");

}

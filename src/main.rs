
// this is a declarative macro that takes a name as an argument and prints a greeting message to the console.
macro_rules! log {
    ($log:expr) => {
        println!("[LOG]: {}", $log);
    };
}

fn main() {

    //COMMENTS (SINGLE-LINE AND MULTI-LINE)

    // This is a simple comment.

    /*
     * This is a section of a multi-line comment.
     */

    //VARIABLES (MUTABLE AND IMMUTABLE)

    /* A variable without mut is immutable, so it cannot be changed later in the program. Init _ as convention to indicate 
    that the variable is not used. */
    let _y = 10;

    // mut is for making a variable mutable, so that it can be changed later in the program.
    let mut x = 5;
    println!("x = {}", x);

    x = 6; 

    println!("x = {}", x);

    log!("This is an application log message");

    //CONTROL FLOW (If, else-if, else)

    let x = 10;

    // Using if, else-if, and else to control the flow of the program based on conditions.
    if x % 2 == 0 && x % 5 == 0 {  
        println!("X is divisible by both 2 and 5");    
    } else if x % 2 == 0 {
        println!("X is divisible by 2");
    } else if x % 5 == 0 {
        println!("X is divisible by 5");
    } else {
        println!("X is not divisible by 2 or 5");
    }

    // Using if as an expression to assign a value to a variable based on a condition.
    let z = if x % 2 == 0 { x } else { x % 5 };
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

    //USE BREAK AND CONTINUE IN FOR LOOP
    for number in 1..=10 {
        if number % 2 == 0 {
            continue; // Skip even numbers
        }          

        if number == 7 {
            break; // Exit the loop when number reaches 7
        }

        println!("Odd number: {}", number);
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

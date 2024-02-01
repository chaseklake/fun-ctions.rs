fn main() {
    println!("Hello, world!");

    another_function();
    print_val(5);
    print_labeled_measurement(42, 'F');

    //Example of statement vs expression:
    let _y = 6; // Does not return values
    // let x = (let y = 7); //ERROR: cannot set x because 'let' doesn't return a value.
    // In other languages, assignment returns the value. In C, for example, "x = y = 6" sets both x and y to 6.

    // Here, the curly braces are a scope block, which is an expression because it returns a value.
    let _y = {
        let x = 3;
        x + 1 // Expressions do not end in semicolons
    }; // Statements end in semicolons

    let _x = five(); // Functions have return types. If not specified, the function returns a unit ().
    let _x = plus_one(6); // Example of calling a function with a parameter.
}

// Simple function example
fn another_function() {
    println!("Another function.");
}

// Function with single parameter
fn print_val(x: i32) {
    println!("The value of x is: {x}");
}

// Function with multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Function with a specified return type (default is unit ())
fn five() -> i32 {
    5
}

// Function with parameter and return type
fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; // ERROR: Adding a semicolon to the end of the line makes this a statement, meaning we're not returning the value
    //                  but the function definition (-> i32) promises we'll return an integer.
}
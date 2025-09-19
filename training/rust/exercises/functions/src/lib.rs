/// # Basic Functions in Rust
///
/// **Functions are the building blocks of Rust programs.**
///
/// Key characteristics:
/// - Declared with `fn` keyword
/// - Snake case naming convention
/// - Explicit return types
/// - Last expression is automatically returned
#[test]
#[allow(clippy::needless_return)]
fn basic_functions() {
    // Simple function with no parameters and no return value
    fn greet() {
        println!("Hello, world!");
    }

    // Function with parameters
    fn greet_person(name: &str) {
        println!("Hello, {name}!");
    }

    // Function with return value
    fn add(a: i32, b: i32) -> i32 {
        a + b // No semicolon = return value
    }

    // Function with explicit return
    fn multiply(a: i32, b: i32) -> i32 {
        return a * b; // Explicit return with semicolon
    }

    // Calling functions
    greet();
    greet_person("Alice");
    let sum = add(2, 3);
    let product = multiply(4, 5);

    println!("2 + 3 = {sum}");
    println!("4 * 5 = {product}");

    // EXERCISE: Uncomment and fix the errors
    // fn broken_add(a: i32, b: i32) -> i32 {
    //     a + b;
    // }
    //
    // let result = broken_add(2, 3);
    // println!("2 + 3 = {}", result);
}

/// # Function Parameters and Arguments
///
/// **Parameters are the inputs defined in the function signature.**
/// **Arguments are the actual values passed when calling the function.**
#[test]
fn function_parameters() {
    // Function with multiple parameters
    fn create_point(x: i32, y: i32) -> (i32, i32) {
        (x, y)
    }

    // Calling with positional arguments
    let point1 = create_point(10, 20);
    println!("Point 1: ({}, {})", point1.0, point1.1);

    // EXERCISE: Create a function that takes three i32 parameters
    // and returns their product
    // fn volume(length: i32, width: i32, height: i32) -> i32 {
    // }

    // let box_volume = volume(3, 4, 5);
    // println!("Box volume: {}", box_volume);

    // EXERCISE: Uncomment and fix the type errors
    // let result = add(2.5, 3.7);
    //
    // fn add(a: i32, b: i32) -> i32 {
    //     a + b
    // }
}

/// # Function Return Values
///
/// **Rust functions return exactly one value.**
///
/// - The return type is specified after `->`
/// - The last expression is automatically returned
/// - Use `return` for early returns
#[test]
fn function_returns() {
    // Function returning a tuple
    fn min_max(a: i32, b: i32) -> (i32, i32) {
        if a < b { (a, b) } else { (b, a) }
    }

    let (min, max) = min_max(15, 10);
    println!("Min: {min}, Max: {max}");

    // Function with early return
    fn is_even(num: i32) -> bool {
        if num % 2 == 0 {
            return true;
        }
        false // Returned if not even
    }

    println!("Is 4 even? {}", is_even(4));
    println!("Is 5 even? {}", is_even(5));

    // EXERCISE: Write a function that returns the absolute value
    // fn absolute(value: i32) -> i32 {
    // }

    // println!("Absolute of -5: {}", absolute(-5));
    // println!("Absolute of 10: {}", absolute(10));
}

/// # Closures (Anonymous Functions)
///
/// **Closures are functions that can capture their environment.**
///
/// - Can be stored in variables
/// - Can capture variables from their scope
/// - Have more flexible syntax than functions
#[test]
fn closures() {
    // Basic closure
    let add = |a: i32, b: i32| -> i32 { a + b };
    println!("3 + 4 = {}", add(3, 4));

    // Closure with type inference
    let multiply = |a, b| a * b;
    println!("3 * 4 = {}", multiply(3, 4));

    // Closure capturing environment
    let factor = 2;
    let scaled_add = |a, b| (a + b) * factor;
    println!("(3 + 4) * 2 = {}", scaled_add(3, 4));
}

/// # Basic If/Else Control Flow in Rust
///
/// Rust's `if` expressions are similar to other languages but with a few key differences:
/// 1. Conditions **must** evaluate to a `bool` (no implicit truthy/falsy values).
/// 2. `if/else` is an **expression** (can return a value).
/// 3. Each block can contain multiple statements, with the last one being the return value.

#[test]
fn if_else() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

#[test]
fn if_else_expression() {
    let number = 42;

    // If/else as an expression (returns a value)
    let result = if number % 2 == 0 {
        "even" // No semicolon = return value
    } else {
        "odd"
    };

    println!("The number is {result}"); // Prints "The number is odd"
}

#[test]
fn if_else_multiple_conditions() {
    // Multiple conditions with else if
    let temperature = 25;

    let desc = if temperature > 30 {
        "hot"
    } else if temperature > 20 {
        "pleasant"
    } else if temperature > 10 {
        "chilly"
    } else {
        // this is the default value
        "cold"
    };

    println!("Outside it is {desc}");
}

/// EXERCISE: Complete the function to return the correct discount
fn calculate_discount(total: f64, is_member: bool) -> f64 {
    // Implement the following logic:
    // - If total >= 100.0 and is_member is true: 20% discount
    // - If total >= 100.0 but not a member: 10% discount
    // - If total >= 50.0: 5% discount
    // - Otherwise: no discount (0.0)

    0.0
}

#[test]
fn test_discount() {
    assert_eq!(calculate_discount(120.0, true), 0.20);
    assert_eq!(calculate_discount(120.0, false), 0.10);
    assert_eq!(calculate_discount(60.0, true), 0.05);
    assert_eq!(calculate_discount(30.0, false), 0.0);
}

#[test]
fn iterating_loop() {}

const fn factorial(n: u32) -> u32 {
    let mut result = 1;
    let mut i = 1;
    loop {
        // This loop runs AT COMPILE TIME
        if i > n {
            break;
        }
        result *= i;
        i += 1;
    }
    result
}

const FACT_5: u32 = factorial(5); // Computed during compilation

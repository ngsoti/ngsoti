use std::time::SystemTime;

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

    println!("The number is {result}");
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

/// This uses Rust's `loop` construct which is ideal when:
/// - You need an infinite loop that breaks on a condition
/// - The termination condition is determined during execution
/// - You want to guarantee the loop body executes at least once
///
/// This pattern is particularly useful for:
/// - Retry operations
/// - Processing until a condition is met
/// - Cases where the exit condition isn't known at the start
#[test]
fn iterating_loop() {
    let mut limit = 42;
    loop {
        println!("again!");
        if limit == 0 {
            break;
        }
        limit -= 1;
    }
}

/// In Rust, `loop` can be used in `const` contexts **only in very specific cases**:
///
/// ## Key Requirements:
/// - The loop must have a **clear termination condition**
/// - All operations must be **const-evaluable**
/// - No heap allocations or non-const functions
/// - Limited to basic control flow (no complex patterns)
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

// We have seen `const` are fixed values known at compile time.
// So here factorial fn is ran at compile time
const FACT_5: u32 = factorial(5);

#[test]
fn const_in_loop() {
    println!("{FACT_5}");

    // using a const fn doesn't mean it must run at compile time
    // rather it means: "it is possible to run it a compile time"
    let nanoseconds = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // here the parameter cannot be known at compile time, because we get
    // it at runtime. So the result cannot be computed as a constant value.
    println!("{}", factorial((nanoseconds >> 28) as u32));

    // EXERCISE: uncomment code to verify
    // const FACT_RAND: u32 = factorial((nanoseconds >> 28) as u32);

    // GOING FURTHER: many functions on primary types in Rust can be ran
    // in a `const` context. Take a look at the documentation to have an
    // overview.
}

/// This uses Rust's `while` construct which is ideal when:
/// - You have a clear condition checked BEFORE each iteration
/// - The loop might not need to run at all (condition could be false initially)
/// - The termination condition is simple and obvious
///
/// This pattern is particularly useful for:
/// - Processing collections until empty (while !queue.is_empty())
/// - Polling/waiting for conditions (while !data_ready())
/// - Cases where the exit condition is known before entering the loop
#[test]
fn iterate_with_while() {
    let mut count = 5;

    while count > 0 {
        println!("Countdown: {count}");
        count -= 1;
    }

    println!("Blastoff!")
}

/// This uses Rust's `for` construct which is ideal when:
/// - You want to iterate over a known collection or range
/// - The number of iterations is known or can be determined upfront
/// - You need to process each item in a sequence
///
/// This pattern is particularly useful for:
/// - Iterating over arrays, vectors, or other collections
/// - Processing items in a range (e.g., 0..10)
/// - Cases where you want to explicitly process each element
#[test]
fn iterate_with_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

#[test]
fn iterate_over_range() {
    println!("range [1;4[");
    // iterating over a range from [1; 4[
    for number in 1..4 {
        println!("the value is: {number}");
    }
    println!();

    // EXERCISE: let's explore together ranges usage
    //   - inclusive range
    //   - iterate in reverse order
}

/// One can also walk an array and several other collections
/// implementing `Iterator` and `DoubleEndedIterator`
///
/// ## Common Types That Support `rev()`:
/// - Ranges (`(1..5).rev()`)
/// - Vectors (`vec![1,2,3].into_iter().rev()`)
/// - Arrays (`[1,2,3].iter().rev()`)
/// - Strings (`"hello".chars().rev()`)
#[test]
fn reverse_iterator() {
    let a = [1, 2, 3, 4, 5];

    for element in a.iter().rev() {
        println!("the value is: {element}");
    }

    println!("BOOOOM!");
}

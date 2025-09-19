/// # The Result Enum in Rust
///
/// `Result` is Rust's way of handling operations that might succeed or fail.
/// It's an enum with two variants:
///
/// ```rust
/// enum Result<T, E> {
///     Ok(T),   // Success! Contains a value of type T
///     Err(E),  // Failure! Contains an error of type E
/// }
/// ```
///
/// ## Key Points
/// - `Result` makes error handling explicit
/// - You MUST handle both success and failure cases
/// - The `?` operator makes working with Results easy
/// - Never ignore errors (unlike exceptions in other languages)
///
/// Think of `Result` like a box that either contains:
/// - Your successful result (✅ `Ok`), or
/// - An error message (❌ `Err`)
/// And Rust makes sure you check which one it is!
#[test]
fn result() {
    fn handle_positive_number(i: i32) -> Result<i32, &'static str> {
        if i.is_positive() {
            return Ok(i);
        }
        Err("we expect a positive number")
    }

    let i = 42;
    println!(
        "input={i} handle_positive_number={:?}",
        handle_positive_number(i)
    );

    // EXERCISE: uncomment and run code
    // let i = -42;
    // println!(
    // "input={i} handle_positive_number={:?}",
    // handle_positive_number(i)
    // );
}

/// Question mark operator for Result follow the same philosophy
/// that `?` for Option
///
/// The `?` operator is **Rust's shortcut for error handling**.
/// It does two things in one step:
///
/// 1. If `Ok(value)` → **unwraps** the value
/// 2. If `Err(e)` → **returns the error early**
///
/// ## Rules to Remember
/// 1. Only works in functions that return `Result`
/// 2. Automatically returns errors up the call stack
/// 3. Makes error handling code much shorter
/// 4. Safer than `unwrap()` (which crashes on errors)
///
/// The `?` operator is like saying:
/// "If this worked, keep going. If it failed, return the error immediately."
#[test]
#[allow(clippy::question_mark)]
fn question_mark_operator() {
    fn handle_positive_number(i: i32) -> Result<i32, &'static str> {
        if i.is_positive() {
            return Ok(i);
        }
        Err("we expect a positive number")
    }

    fn square_positive_number(i: i32) -> Result<i32, &'static str> {
        let num = match handle_positive_number(i) {
            Ok(i) => i,
            Err(s) => return Err(s),
        };

        Ok(num * num)
    }

    let i = 42;
    println!("i={i} i^2={:?}", square_positive_number(i));

    let i = -42;
    println!("i={i} i^2={:?}", square_positive_number(i));

    // EXERCISE: create a new fn doing the exact same thing as square_positive_number
    // but using the ? operator
}

/// # Common Result Methods
///
/// Result and Option work similarly and share common
/// patterns and method names
///
/// - `unwrap()` - Get the value (PANICS if Err - avoid in production!)
/// - `unwrap_or(default)` - Get the value or a default
/// - `map()` - Transform the success value
/// - `and_then()` - Chain operations
/// - `is_ok()` / `is_err()` - Check which variant it is
#[test]
fn common_methods() {
    fn handle_positive_number(i: i32) -> Result<i32, &'static str> {
        if i.is_positive() {
            return Ok(i);
        }
        Err("we expect a positive number")
    }

    let res_ok = handle_positive_number(42);
    let res_err = handle_positive_number(-42);

    println!("testing is_ok and is_err");
    println!("res_ok is ok={} err={}", res_ok.is_ok(), res_ok.is_err());
    println!("res_err is ok={} err={}", res_err.is_ok(), res_err.is_err());
    println!();

    println!("testing unwrap_or");
    println!("res_ok.unwrap_or(1337)={}", res_ok.unwrap_or(1337));
    println!("res_err.unwrap_or(1337)={}", res_err.unwrap_or(1337));
    println!();

    println!("testing map to square the result");
    println!("res_ok.map={:?}", res_ok.map(|i| i * i));
    println!("res_err.map={:?}", res_err.map(|i| i * i));

    println!("test unwrapping");
    println!("res_ok.unwrap()={}", res_ok.unwrap());

    // EXERCISE: uncomment and run code below
    // println!("res_err.unwrap()={}", res_err.unwrap());
}

/// In practice Results are used for proper error handling.
/// Most of the times errors can be of different kinds so
/// it is a very common pattern to use an enum Error type
/// to return an error. In this way we are able to handle
/// each kind of error separately.
#[test]
#[allow(dead_code)]
fn result_in_practice() {
    #[derive(Debug)]
    enum Error {
        IsZero,
        IsNegative(i32),
    }

    fn handle_positive_number(i: i32) -> Result<i32, Error> {
        if i == 0 {
            Err(Error::IsZero)
        } else if i.is_negative() {
            Err(Error::IsNegative(i))
        } else {
            Ok(i)
        }
    }

    let i = 42;
    println!("input={i} result={:?}", handle_positive_number(i));

    let i = 0;
    println!("input={i} result={:?}", handle_positive_number(i));

    let i = -42;
    println!("input={i} result={:?}", handle_positive_number(i));

    // EXERCISE: make an example where you handle the result of `handle_positive_number`
    // and where you print a different message for each variant of Error enum.
}

/// # Generics in Rust (the basics)
///
/// **Generics let you write flexible code that works with multiple types.**
///
/// Think of them like "type placeholders" - you define a function/struct/enum
/// with a generic type, and the compiler fills in the actual type when used.
#[test]
fn generic_basics() {
    // T here means any type, this way we don't have to create
    // one enum per type. We define it once and the compiler generate
    // the different flavors.
    #[derive(Debug)]
    enum OptionalValue<T> {
        Some(T),
        None,
    }

    // Here we provide a value in Some variant so the compiler can infer
    // the type T of the enum. Here the compiler generates an enum for T being String
    // enum OptionalValue<String> {
    //    Some(String)),
    //    None,
    // }
    let so = OptionalValue::Some(String::from("Hello World"));
    println!("{so:?}");

    // Here we provide a value in Some variant so the compiler can infer
    // the type T of the enum. Here the compiler generates an enum for T being i32
    // enum OptionalValue<i32> {
    //    Some(i32),
    //    None,
    // }
    let io = OptionalValue::Some(42);
    println!("{io:?}");

    // **`None` is a unit-like variant**:
    //   - `None` doesn't contain any value to infer the type from
    //   - The compiler doesn't know if you want `OptionValue<i32>`, `OptionValue<String>`, etc.
    // So we need to explicitly type the variable to tell the compiler
    // what enum it must generate.
    let none: OptionalValue<()> = OptionalValue::None;
    println!("{none:?}");

    // this was a gentle introduction to generics, there is lot more
    // to learn about it but it is just so that you understand the
    // notation we are going to use just after. Let's dive into the
    // Option Rust enum, which is a way more powerful version of
    // what we have just implemented above.
}

/// # Rust's `Option` Enum
///
/// **`Option` is Rust's way of handling values that might be present or absent - without using `null`.**
///
/// ```rust
/// enum Option<T> {
///     Some(T),  // A value exists
///     None,     // No value exists
/// }
/// ```
///
/// ## Why Use `Option`?
/// - Safer than `null` (compiler forces you to handle the `None` case)
/// - Clearer intent than using special values (like -1 for "not found")
/// - Built into Rust's standard library
#[test]
fn option_1() {
    // Let's assume we want to design a structure holding
    // some information about a user, entered via a form.
    // In this form, fields are mandatory and other are not.
    #[derive(Debug)]
    struct User {
        // this is mandatory, we use it for login
        identifier: String,
        password: String,
        // this is not mandatory
        age: Option<u8>,
    }

    impl User {
        fn new(identifier: String, password: String, age: Option<u8>) -> Self {
            Self {
                identifier,
                password,
                age,
            }
        }

        fn has_age(&self) -> bool {
            // this is a method implemented for Option<T>
            // that returns true if the Option is a Some variant
            self.age.is_some()
        }

        fn print(&self) {
            println!(
                "id:{} passwd:{} filled age:{}",
                self.identifier,
                self.password,
                self.has_age()
            )
        }
    }

    let u1 = User::new(String::from("toto"), String::from("Password123"), Some(42));

    let u2 = User::new(String::from("joe"), String::from("0xdeadbeef"), None);

    u1.print();
    u2.print();

    // Just take a little moment on how you would implement this in other programming languages?
}

/// Previously we have seen match statements that can be
/// used to handle all the possible variants of an enum.
/// Yet it might be verbose to always handle all the cases.
/// This is where `if let` comes into play.
#[test]
#[allow(clippy::single_match)]
fn option_if_let() {
    let o = Some(42);

    // This is a bit verbose especially if we don't want to
    // handle None value
    match o {
        // DESTRUCTURING the enum we can use i as an integer
        Some(i) => println!("i={i}"),
        // This means we do nothing in None
        None => {}
    }

    // # `if let` Syntax
    //
    // `if let` is a concise way to:
    // 1. Match ONE pattern (unlike `match` which requires all cases)
    // 2. Destructure and bind variables in the pattern
    // 3. Execute code only when the pattern matches// So this is a preferred notation
    if let Some(i) = o {
        println!("i={i}")
    }

    // !!!! IMPORTANT !!!!
    // `if let` works with any enum but you will very likely use it a lot
    // to handle Option enums
}

/// Option enum has many useful methods. One of the most important is `map`,
/// which transforms the contained value while preserving the Option structure.
///
/// `map` applies a function to the contained value (if Some) and returns a new Option.
/// - If the input is Some(x), returns Some(f(x))
/// - If the input is None, returns None
#[test]
fn option_map() {
    let os = Some("hello world!");
    println!("optional str: {os:?}"); // Some("hello world!")

    // Transform the string to its length using map
    let opt_cnt = os.map(|s| s.len());
    println!("optional byte count: {opt_cnt:?}"); // Some(12)

    // EXERCISE: uncomment below code
    // let none_str: Option<&str> = None;
    // let none_cnt = none_str.map(|s| s.len());  // None
    // println!("none cnt:{none_cnt:?}");

    // If we look at the prototype of the `map` function
    // we see it returns an Option. So `map` calls can be
    // chained.
    let mul_cnt = Some("Hello World ! OMG this is awesome")
        .map(|s| s.len())
        .map(|len| len * 2);
    println!("byte count * 2: {mul_cnt:?}")
}

/// # Unwrapping Options in Rust
///
/// **Unwrapping means extracting the value from inside an Option.**
///
/// An `Option<T>` is like a box that might contain a value (`Some(value)`) or might be empty (`None`).
/// Unwrapping is how you get the value out of the box.
#[test]
#[allow(clippy::unnecessary_literal_unwrap)]
fn option_unwrap() {
    let some_number = Some(5);
    let number = some_number.unwrap();

    println!("unwrapped number={number}");

    // Unwrapping is equivalent to
    let number = match some_number {
        Some(i) => i,
        // If enum is None the code panics (i.e. program crashes)
        None => panic!(),
    };
    println!("manually unwrapped number={number}");

    // EXERCISE: uncomment below and run the code
    // let opt_number: Option<i32> = None;
    // opt_number.unwrap();

    // !!! IMPORTANT RULES !!!
    // 1. Avoid `unwrap()` in production code - it can crash your program
    // 2. Prefer pattern matching or unwrap variants - it's explicit and safe
}

#[test]
#[allow(clippy::unnecessary_literal_unwrap)]
fn option_safe_unwrap() {
    // There are safe ways to unwrap without making the code panic
    let opt_number: Option<i32> = None;

    // Returns the default value of the type
    let number_default = opt_number.unwrap_or_default();
    println!("default={number_default}");

    // Returns the default value provided
    let number_prov_default = opt_number.unwrap_or(42);
    println!("provided default={number_prov_default}");

    // NB: other safe unwrap method exists in Option enum
    // feel free to explore them
}

/// # The `?` Operator with Option
///
/// The `?` operator is a **shortcut for handling Options**.
/// It does two things in one simple operation:
///
/// 1. If the Option is `Some(value)` → **unwraps** the value
/// 2. If the Option is `None` → **returns early** with `None`
///
/// ## When to Use It
/// - When you have a function that returns `Option`
/// - When you want to "bubble up" `None` cases
/// - To avoid nested `match` statements
///
/// ## Key Points
/// - Only works in functions that return `Option`
/// - Makes code cleaner by handling `None` cases automatically
/// - Much safer than `unwrap()` which panics on `None`
#[test]
#[allow(clippy::manual_find)]
#[allow(clippy::question_mark)]
fn question_mark_operator() {
    fn find_first_even_number(numbers: &[i32]) -> Option<i32> {
        for &num in numbers {
            if num % 2 == 0 {
                // we found an even number
                return Some(num);
            }
        }
        // we didn't find any even number
        None
    }

    fn square_first_even(numbers: &[i32]) -> Option<i32> {
        let n = match find_first_even_number(numbers) {
            Some(banana) => banana,
            None => return None,
        };
        Some(n * n)
    }

    println!("square_first_even = {:?}", square_first_even(&[3, 1, 9, 2]));

    // the above syntax is quite heavy, that is why question mark operator `?`
    // got introduced. We can see it syntax sugar to make more concise code.
    fn square_first_even_short(numbers: &[i32]) -> Option<i32> {
        let n = find_first_even_number(numbers)?;
        Some(n * n)
    }

    println!(
        "square_first_even_short = {:?}",
        square_first_even_short(&[3, 1, 9, 2])
    );
}

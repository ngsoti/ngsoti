//! # Traits in Rust
//!
//! Traits are Rust's way of defining shared behavior.
//!
//! Think of a trait like a **contract** that says:
//! "Any type that implements this trait must provide these methods"
//!
//! ## Why Use Traits?
//! 1. **Shared Behavior**: Define what types can do without knowing what they are
//! 2. **Code Reuse**: Write functions that work with any type that implements the trait
//! 3. **Polymorphism**: Treat different types the same way if they implement the same trait
//!
//! ## Key Points
//! - Like interfaces in other languages, but more powerful
//! - Can define default method implementations
//! - Can be used as type bounds for generics
//! - Enable operator overloading (like `+` for your types)
//!
//! ## Common Traits
//! - `Debug`: For printing with `{:?}`
//! - `Display`: For printing with `{}`
//! - `PartialEq`: For equality comparisons (`==`)
//! - `From`/`Into`: For type conversions
//!
//! Traits let you write code that works with multiple types
//! as long as they implement the required behavior.

#[test]
fn trait_syntax() {
    // Define a trait
    trait Greet {
        fn greet(&self) -> String; // Any type with Greet must implement this
    }

    struct Human;
    // We implement trait Greet for Human
    impl Greet for Human {
        fn greet(&self) -> String {
            String::from("Hi!")
        }
    }

    struct Cat;
    impl Greet for Cat {
        fn greet(&self) -> String {
            String::from("Meow!")
        }
    }

    let human = Human;
    println!("human say: {}", human.greet());

    let cat = Cat;
    println!("cat say: {}", cat.greet());
}

/// Default implementations allow traits to provide standard behavior that:
///
/// This makes traits more powerful and maintainable by:
/// - Giving a standard implementation that works for most cases
/// - Allowing types to opt-out and provide custom behavior when necessary
/// - Keeping the trait definition as the single source of truth for common logic
///
/// Think of it like providing template methods where the default works for
/// most cases, but can be customized when special behavior is needed.
#[test]
fn default_implementation() {
    trait Greet {
        fn greet(&self) -> String;

        // we provide a default implementation for
        // print_greeting function
        fn print_greeting(&self) {
            println!("{}", self.greet())
        }
    }

    struct Human;
    // We implement trait Greet for Human
    impl Greet for Human {
        fn greet(&self) -> String {
            String::from("Hi!")
        }
    }

    struct Cat;
    impl Greet for Cat {
        fn greet(&self) -> String {
            String::from("Meow!")
        }
    }

    let cat = Cat;
    cat.print_greeting();

    let human = Human;
    human.print_greeting();
}

/// Let's explore how to use trait bounds with generics to
/// accept any type that implements a specific trait.
/// This enables polymorphism - treating different types uniformly
/// based on their shared behavior rather than their concrete types.
#[test]
fn traits_and_generics() {
    trait Greet {
        fn greet(&self) -> String;
    }

    struct Human;
    // We implement trait Greet for Human
    impl Greet for Human {
        fn greet(&self) -> String {
            String::from("Hi!")
        }
    }

    struct Cat;
    impl Greet for Cat {
        fn greet(&self) -> String {
            String::from("Meow!")
        }
    }

    // EXERCISE: uncomment below code and let's understand what
    // is going on
    // fn greet<T>(t: T) {
    //     println!("{}", t.greet())
    // }
}

/// # What `#[derive(TraitName)]` Does
///
/// The `derive` macro **automatically implements common traits** for your type.
///
/// Instead of manually writing implementations for traits like:
/// - `Debug` (for printing with `{:?}`)
/// - `Clone` (for copying values)
/// - `PartialEq` (for `==` comparisons)
/// - `Default` (for default values)
///
/// You just add `#[derive(TraitName)]` above your struct/enum,
/// and Rust generates the implementation for you.
///
/// This tells Rust:
/// "Generate the code needed to make this type Debug, Clone, and PartialEq"
///
/// ## Key Points:
/// - Saves you from writing repetitive boilerplate code
/// - Only works for traits that can be automatically implemented
/// - The compiler generates efficient implementations
/// - Works for both structs and enums
///
/// !!! IMPORTANT !!!
/// If you want your trait to be used with `derive` macro
/// you must implement a `proc-macro`, which is beyond the scope
/// of this Rust introduction.
#[test]
fn derive_macro() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let p1 = Person {
        name: String::from("bob"),
        age: 42,
    };

    println!("p1={p1:?}");

    // EXERCISE: let's uncomment below code and fix it
    // let p2 = p1.clone();
    // println!("p2={:?}", p2);

    // !!! IMPORTANT !!!
    // For a struct to be able to derive a trait automatically
    // it is OFTEN required that all members implement this trait
}

/// # From and Into Traits
///
/// These traits provide **type conversion** functionality in Rust.
///
/// ## 1. The `From` Trait
/// ```rust
/// pub trait From<T> {
///     fn from(T) -> Self;
/// }
/// ```
/// - **Purpose**: Define how to create `Self` from another type `T`
/// - **Example**: Convert `String` to `Vec<u8>`
/// - **Usage**: Explicit conversions with `From::from(value)`
///
/// ## 2. The `Into` Trait
/// ```rust
/// pub trait Into<T> {
///     fn into(self) -> T;
/// }
/// ```
/// - **Purpose**: Define how to convert `Self` into another type `T`
/// - **Automatically implemented**: If you implement `From<T> for U`, you get `Into<U> for T` for free
/// - **Usage**: Explicit conversions with `value.into()`
///
/// ## Key Relationship
/// - `From` and `Into` are duals of each other
/// - Implementing `From` automatically gives you `Into`
/// - `Into` is more commonly used in generic bounds
///
/// ## Important Notes
/// - `From` is preferred when implementing conversions
/// - `Into` is preferred when accepting parameters (more flexible)
/// - Many standard library types implement these traits
/// - The conversion is **consuming** (takes ownership)
///
/// ## Why They Matter
/// - Enable ergonomic type conversions
/// - Power the `?` operator for error handling
/// - Allow flexible function parameters with `impl Into<T>`
/// - Provide a standardized way for type conversions
///
/// ## Best Practices
/// - Implement `From` for your types to enable conversions
/// - Use `Into` in function parameters for maximum flexibility:
///   ```rust
///   fn process<T: Into<MyType>>(value: T) { ... }
///   ```
/// - The conversion should be **infallible** (never fail)
#[test]
fn from_and_into() {
    #[derive(Debug)]
    struct MyU8(u8);

    #[derive(Debug)]
    struct MyU64(u64);

    // We implement a way to convert MyU8 into MyU64
    impl From<MyU8> for MyU64 {
        fn from(value: MyU8) -> Self {
            Self(value.0 as u64)
        }
    }

    let my_u8 = MyU8(42);
    // we call the conversion method we've just defined
    let my_u64 = MyU64::from(my_u8);

    println!("my_u64={my_u64:?}");

    // EXERCISE: uncomment below, understand and fix the error
    // println!("my_u8:{:?}", my_u8);

    // EXERCISE: implement a way to convert a u8 into a MyU8

    // EXERCISE: implement a way to convert a MyU64 int a MyU8
}

/// # Stack Allocation Rules
///
/// A type goes on the stack if:
/// 1. Its size is known at compile time
/// 2. It doesn't contain heap-allocated data (directly)
/// 3. It implements `Copy` (usually, but not always - see below)
/// 4. It's not recursively defined (no infinite size)
#[test]
fn on_the_stack() {
    // All stack-allocated:
    let num: i32 = 42; // 4 bytes
    let pair: (f64, bool) = (3.42, true); // 9 bytes (8 + 1, padded)
    let arr: [u8; 4] = [1, 2, 3, 4]; // 4 bytes

    // When we move a type allocated on the stack it gets
    // COPIED (this is a very important concept to understand).
    // The process of copy-ing allocates KNOWN SIZE space on the stack
    // at compile and put the data in it.

    let mut new_num = num;
    new_num -= 1;
    println!("num={num} new={new_num}");
}

/// # Heap Allocation in Rust
///
/// In Rust, data goes on the heap when the size of the type
/// cannot be known at compile time. This is important to know
/// what if a type lives on stack or on the heap as. It will dictate
/// if ownership rules applies or not. Data on the heap WILL NOT be
/// copied when they are used in new variables, functions ...
///
/// REMINDER: data on the stack are just copied when we need to
/// move them somewhere (in a new variable, function ...)
///
/// ## Examples
/// - `String`: Heap-allocated UTF-8 string
/// - `Vec<T>`: Heap-allocated vector
/// - `HashMap<K, V>`: Heap-allocated hash map
/// - `Box<T>`: Single heap-allocated value
#[test]
fn on_the_heap() {
    // the String type is used to manipulate
    // strings modified at runtime. So it must be
    // stored on the heap.
    let mut s = String::from("hello");
    println!("{s}");
    // we grow the string
    s.push_str(" world !");
    println!("{s}");
}

#[test]
fn type_on_heap_string() {
    // Strings are living on the heap so the compiler
    // doesn't know what size it needs to prepare for
    // allocation. So it cannot be copy-ied, according to
    // the rust terminology. If we want to duplicate
    // anything living on the heap, we explicitly need
    // to call the clone() function of the type.
    let s1 = String::from("hello, world !");
    let s2 = s1;
    println!("{s2}");

    // You: Wait you said it cannot be copied yet that is what
    // just happened ?
    // Me: No :) ! The data just got MOVED

    // EXERCISE: uncomment below + try to fix it
    //println!("{s1}");
}

/// # Ownership Rules !!! VERY IMPORTANT !!!
/// - Each value in Rust has an owner
/// - There can only be ONE owner at a time
/// - When the owner goes out of scope, the value gets dropped
#[test]
fn ownership_1() {
    let s = String::from("hello");

    {
        // we enter a new scope
        let s2 = s;
        println!("{s2} world !")
    };

    // EXERCISE: explain why the following doesn't work
    // EXERCISE: make it work
    // println!("{s}");

    // EXERCISE: explain why the following doesn't work
    // println!("{s2}");
}

#[test]
fn ownership_2() {
    let x = 42;

    {
        let y = x;
        println!("y={y}");
    }

    // EXERCISE: explain why this works
    println!("x={x}");
}

fn calculate_length(s: String) -> usize {
    s.len()
}

#[test]
fn ownership_3() {
    let s = String::from("hello rust!");

    println!("len={}", calculate_length(s));

    // EXERCISE: explain why this doesn't work
    // EXERCISE: make it work
    // println!("after fn call s={s}")

    // EXERCISE: do you think the solution is efficient ?
}

/// # Borrowing
///
/// Borrowing is using the reference to a value instead of using
/// the value itself. As a consequence a function taking a reference
/// to a value doesn't OWN the data instead, but instead it BORROW it.
/// When the reference goes out of scope, it gets dropped but this doesn't
/// affect the value itself.
///
/// References can either be IMMUTABLE (we don't need to modify inner value) or
/// MUTABLE (we do need to modify inner value). The choice of mutability
/// affects what you can do with the references as these rules must always
/// be guaranteed:
/// - At any given time, you can have either one mutable reference or any number of immutable references.
/// - References must always be valid (i.e. must always reference something in scope)
fn calculate_length_by_ref(s: &String) -> usize {
    s.len()
}

#[test]
fn borrowing_1() {
    let s = String::from("hello rust!");

    println!("len={}", calculate_length_by_ref(&s));
    println!("after fn call s={s}")
}

fn modify_string(s: &mut String) {
    s.push_str(" world");
}

#[test]
fn borrowing_2() {
    let mut s = String::from("hello");

    // we pass a mutable reference to s to modify it
    modify_string(&mut s);

    println!("s={s} len={}", calculate_length_by_ref(&s));
}

#[test]
fn borrowing_3() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("r1={r1} r2={r2}");

    // EXERCISE: uncomment following
    // let r3 = &mut s; // BIG PROBLEM
    // println!("r1={r1} r2={r2} r3={r3}");
}

// EXERCISE: uncomment function, why does the compiler complains ?
// fn dangle() -> &String {
// let s = String::from("hello");

// &s
// }

/// # Slices in Rust
///
/// A slice is a **reference to a contiguous sequence** of elements in a collection rather than the whole collection.
///
/// - **View into data**: A way to reference a portion of a collection
/// - **Non-owning**: Just borrows the data (doesn't own it)
/// - **Contiguous**: Always refers to a continuous sequence
/// - **Flexible**: Can refer to any portion of the original data
///
/// ## Key Characteristics
/// - **Zero-cost**: No runtime overhead over raw pointers
/// - **Bounds-checked**: Access is safe (panics on out-of-bounds)
/// - **Flexible**: Can refer to arrays, vectors, or other slices
/// - **Borrow rules apply**: Follow same borrowing rules as references
#[test]
fn slice_str() {
    let greeting = String::from("Hello, world!");

    // &str is a slice pointing to contiguous bytes in memory that form a valid UTF-8 string
    let hello = &greeting[0..5]; // "Hello"
    let world = &greeting[7..12]; // "world"
    let all = &greeting[..]; // "Hello, world!"

    println!("First word: {hello}");
    println!("Second word: {world}");
    // the whole string can be a slice
    println!("All: {all}");
}

#[test]
fn array_slice() {
    let numbers = [10, 20, 30, 40, 50];
    let first_two = &numbers[0..2]; // [10, 20]
    let last_two = &numbers[3..]; // [40, 50]
    let middle = &numbers[1..4]; // [20, 30, 40]

    println!("first_two={first_two:?}");
    println!("last_two={last_two:?}");
    println!("middle={middle:?}");
}

/// Slices can be mutable and used to mutate a part of the data
#[test]
fn mut_slice() {
    fn mod_array(s: &mut [u8]) {
        for b in s {
            if *b % 2 == 0 {
                *b = 42
            }
        }
    }

    let mut numbers = [1, 2, 3, 4, 5, 6];
    println!("numbers={numbers:?}");
    mod_array(numbers.as_mut_slice());
    println!("numbers={numbers:?}");
}

/// # Golden Rule of Slice Parameters
///
/// **"If the reference you want to pass to a function is sliceable,
///  then implement the function to accept a slice."**
///
/// ## Why This Rule Works
///
/// 1. **Maximum Flexibility**:
///    - Accepts `&[T]`, `&Vec<T>`, `&Array<T>`, and other sliceable types
///    - Works with both stack and heap allocated data
///
/// 2. **Zero-Cost Abstraction**:
///    - No runtime overhead for the abstraction
///    - Compiles to the same code as if you used concrete types
///
/// 3. **API Ergonomics**:
///    - Callers can pass any sliceable type without conversion
///    - No need for multiple overloads (e.g., one for `&Vec<T>` and one for `&[T; N]`)
///
/// ## When to Apply This Rule
/// - When your function only needs to read/process elements
/// - When the operation doesn't depend on the container type
/// - When you want to accept both arrays and vectors
/// - For string processing (use `&str` instead of `&String`)
///
/// ## Common Sliceable Types
/// | Type          | Slice Type   | Example Function Parameter |
/// |---------------|--------------|----------------------------|
/// | Vec<T>        | &[T]         | `&[T]`                     |
/// | [T; N]        | &[T]         | `&[T]`                     |
/// | String        | &str         | `&str`                     |
/// | Box<[T]>      | &[T]         | `&[T]`                     |
///
/// ## Exceptions
/// - When you specifically need container methods (e.g., `Vec::push`)
/// - When you need to modify the container itself (not just its elements)
/// - When you need ownership of the container
#[test]
fn slice_golden_rule() {
    fn print_greeting(message: &String) {
        println!("{message}");
    }

    // Works with:
    print_greeting(&String::from("hello")); // &String coerces to &str

    // EXERCISE: uncomment below and modify print_greeting so that
    // it accepts both &String and &'static str
    // print_greeting("hello"); // string literal (&'static str)
}

/// # Rust Lifetimes
///
/// ## 1. The Core Concept
/// Lifetimes are Rust's way of ensuring that references are always valid.
/// They track how long references can be safely used.
///
/// !!! The Golden Rule !!! : A reference cannot outlive the data it refers to
///
/// ## 2. The Problem They Solve
/// ```rust
/// // This would create a dangling reference:
/// fn dangling_reference() -> &i32 {
///      let x = 5;          // x is created
///      &x                  // Try to return reference to x
/// }                      // x is dropped here - reference becomes invalid!
/// ```
/// Lifetimes prevent this by ensuring references don't outlive their data.
///
/// ## 3. Lifetime Basics
/// - Every reference (`&T`) has a lifetime
/// - Most lifetimes are inferred by the compiler
/// - You only need to specify them when there's ambiguity
///
/// ## 4. Syntax
/// ```rust
/// &i32        // Reference with inferred lifetime
/// &'a i32     // Reference with explicit lifetime 'a
/// &'a mut i32 // Mutable reference with lifetime 'a
/// ```
///
/// ## 5. Key Properties
/// - Most lifetimes are invisible (compiler infers them)
/// - They don't change how long data lives
/// - They only affect references, not owned types
/// - Zero runtime cost (checked at compile time)
///
/// ## 6. Remember
/// - If code compiles without explicit lifetimes, you're good
/// - Lifetime errors mean a reference might outlive its data
/// - Solution: restructure so references don't outlive their data
///
/// ## 7. Lifetime Elision Rules
/// The compiler uses these rules to infer lifetimes:
/// 1. Each input reference gets its own lifetime
/// 2. If one input lifetime, it's assigned to all outputs
/// 3. For methods with `&self`, self's lifetime is assigned to outputs
#[test]
fn lifetime_example() {
    let s1 = String::from("hello");
    let s2 = String::from("world");

    // The reference must live as long as what it refers to

    // EXERCISE: uncomment the code below
    //
    // fn longest(x: &str, y: &str) -> &str {
    //    if x.len() > y.len() { x } else { y }
    // }
    //
    // let result = longest(&s1, &s2);
    // println!("The longest string is {}", result);

    // EXERCISE: lets break things down and understand what happens
    // - understand lifetime elision
    // - how to fix the error
    // - this function is defined for any &str (this is the issue)
}

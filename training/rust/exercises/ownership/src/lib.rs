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

// EXERCISE: uncomment function, why the compiler complains ?
// fn dangle() -> &String {
// let s = String::from("hello");

// &s
// }

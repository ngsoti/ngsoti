//! # Why Generics Exist (Simple Explanation)
//!
//! **Generics let you write code that works with multiple types without duplication.**
//!
//! ## Key Benefits
//! 1. **Avoid Code Duplication** - Write once, use everywhere
//! 2. **Type Safety** - Compiler checks your types are used correctly
//! 3. **Zero Cost** - Runs as fast as hand-written specific versions
//! 4. **Flexibility** - Works with types you haven't even created yet
//!
//! ## Real-world Example
//! Rust's `Option<T>` and `Result<T, E>` are generic:
//! - `Option<i32>` holds an integer or nothing
//! - `Option<String>` holds a string or nothing
//! - Same enum, different types
//!
//! Generics are like **templates** that the compiler fills in with actual types when used.

#[test]
#[allow(clippy::manual_swap)]
fn generics_in_function() {
    fn first_and_last_i32(list: &[i32]) -> Option<(&i32, &i32)> {
        Some((list.first()?, list.last()?))
    }

    fn first_and_last_f64(list: &[f64]) -> Option<(&f64, &f64)> {
        Some((list.first()?, list.last()?))
    }

    let s_i32 = &[1, 2, 3, 4, 5];
    println!(
        "first_and_last_i32({s_i32:?})={:?}",
        first_and_last_i32(s_i32)
    );

    let s_f64 = &[5.0, 4.0, 3.0, 2.0, 1.0];
    println!(
        "first_and_last_f64({s_f64:?})={:?}",
        first_and_last_f64(s_f64)
    );

    // EXERCISE: let's implement a generic function working
    // with both the slices
}

/// Generics are generated at compile time so if
/// you are trying to define a generic function which isn't
/// allowed, your code will just not compile.
#[test]
fn generics_in_functions_2() {
    // EXERCISE: uncomment code and observe compiler error.
    // - let's understand together why it doesn't work
    //fn largest<T>(list: &[T]) -> &T {
    //    let mut largest = &list[0];

    //    for item in list {
    //        if item > largest {
    //            largest = item;
    //        }
    //    }

    //    largest
    //}
}

#[test]
fn generic_in_structure() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer={integer:?}");
    println!("float={float:?}");

    // EXERCISE: uncomment below code and understand what's wrong
    // let mixed = Point { x: 1, y: 4.0 };

    // EXERCISE: fix Point structure so that it works `mixed` assignment work
}

#[test]
fn generic_in_structure_2() {
    // We can use as many different generics as we want
    // Here we define a point that has a generic type
    // for each of its
    #[derive(Debug)]
    struct Point<X, Y, Z> {
        x: X,
        y: Y,
        z: Z,
    }

    let mix = Point {
        x: 1u8,
        y: 42.0,
        z: Some(1337usize),
    };

    println!("mix={mix:?}");
}

#[test]
fn generic_in_structure_3() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    // `impl<T> Point<T>` means:
    // "Implement methods for the `Point` struct, for ANY type `T`"
    //
    // ## Breaking it down:
    // - `impl` = "I'm going to implement methods for..."
    // - `<T>` = "For any generic type T..."
    // - `Point<T>` = "...the Point struct that uses type T"
    // Think of it like saying:
    // "No matter what type T is, here are methods that all Point<T> will have"
    impl<T> Point<T> {
        fn swap(&mut self) {
            // this is a method from the standard library
            std::mem::swap(&mut self.x, &mut self.y);
        }
    }

    let mut integer = Point { x: 42, y: 1337 };
    println!("integer:{integer:?}");
    integer.swap();
    println!("swapped integer:{integer:?}");
    println!();

    // it doesn't really make sense to define a Point from
    // str but it is just to show you that it is possible
    let mut string = Point {
        x: "hello",
        y: "hola",
    };
    println!("string:{string:?}");
    string.swap();
    println!("swapped string:{string:?}");
}

#[test]
fn generic_structure_3() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    // one can implement function only for a given type
    // which increases the level of possibilities
    impl Point<&'static str> {
        fn concat(&self) -> String {
            let mut s = String::from(self.x);
            s.push_str(self.y);
            s
        }
    }

    let string = Point {
        x: "hello ",
        y: "world !",
    };

    println!("concat:{}", string.concat())

    // EXERCISE: try to call concat on a Point<i32>
}

/// While examples above are not very usable in practice
/// generic becomes very handy when it comes to data structure
/// implementation.
#[test]
fn generic_in_practice() {
    #[derive(Debug)]
    struct Fifo<T> {
        // Vec<T> is a std Rust structure implementing
        // a growable array data structure
        elements: Vec<T>,
    }

    impl<T> Fifo<T> {
        fn new() -> Self {
            Self {
                elements: Vec::new(),
            }
        }

        /// Put an item in the Fifo
        fn put(&mut self, item: T) {
            self.elements.insert(0, item);
        }

        /// Pop an item from the Fifo
        fn pop(&mut self) -> Option<T> {
            self.elements.pop()
        }
    }

    let mut string_fifo = Fifo::new();
    string_fifo.put("hello");
    string_fifo.put("world !");
    println!("first out:{:?}", string_fifo.pop());
    println!("first out:{:?}", string_fifo.pop());

    // EXERCISE: create an example of use of a Fifo with integers
}

// !!! IMPORTANT !!!
// Generic enums work similarly to generic structs in terms of:
// - Syntax for declaring generic types (`enum Name<T> { ... }`)
// - Using generic parameters in variants
// - Implementing methods with `impl<T> EnumName<T>`
// - Type safety and zero-cost abstractions

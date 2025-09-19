/// # Mutability in Rust
///
/// Mutability determines whether a value can be changed after it's created.
/// Rust's approach to mutability is explicit and controlled by the type system.
/// By default, values are immutable.
///
/// ## Why This Matters:
/// - Enables compile-time memory safety
/// - Prevents data races in concurrent code
/// - Makes code intentions clearer
/// - Helps optimize performance (immutable data can be optimized better)
#[test]
fn variable_mutability() {
    // We bind the value 5 to variable x
    let x = 5;
    // EXERCISE: uncomment the following line
    // x = 2;
    println!("{x}");
}

/// # Variable Shadowing in Rust
///
/// Shadowing occurs when a new variable declaration **reuses the name** of a previous variable,
/// effectively "hiding" the previous one within the same scope.
#[test]
fn variable_shadowing() {
    let x = 5;
    println!("original x={x}");

    // We shadow x in the same scope
    let x = -42;
    println!("shadowing x={x}");

    {
        // this is a new code block coming with its own
        // scope. We can shadow variables already defined
        // in the outer block. But it will not affect the
        // outer scope.
        let x: &'static str = "hello world";
        println!("inner scope x={x}");
    }

    println!("outer scope x={x}");
}

/// Signed (i) and unsigned (u) integers in fixed/variable sizes:
/// | Type   | Size (bits) | Range                          | Notes                          |
/// |--------|-------------|--------------------------------|--------------------------------|
/// | `i8`   | 8           | -128 to 127                    |                                |
/// | `u8`   | 8           | 0 to 255                       | Byte-sized; used for raw data  |
/// | `i16`  | 16          | -32,768 to 32,767              |                                |
/// | `u16`  | 16          | 0 to 65,535                    |                                |
/// | `i32`  | 32          | -2³¹ to 2³¹-1                  | Default for integers           |
/// | `u32`  | 32          | 0 to 2³²-1                     |                                |
/// | `i64`  | 64          | -2⁶³ to 2⁶³-1                  |                                |
/// | `u64`  | 64          | 0 to 2⁶⁴-1                     |                                |
/// | `i128` | 128         | -2¹²⁷ to 2¹²⁷-1                |                                |
/// | `u128` | 128         | 0 to 2¹²⁸-1                    |                                |
/// | `isize`| 32/64*      | Platform-dependent             | Pointer-sized (e.g., indexing) |
/// | `usize`| 32/64*      | 0 to 2³²-1 or 2⁶⁴-1            | Used for collection indices    |
///
/// *Depends on CPU architecture (e.g., 64-bit on x86_64).
///
/// Literal suffixes: `42i8`, `42u16`, `42isize`.
/// Supports underscores for readability: `1_000_000`.
#[test]
fn scalar_data_types_integer() {
    // we declare the type
    let small: u8 = 42;

    // this is equivalent to the above notation
    let another = 42u8;

    assert_eq!(another, small);

    let big: u64 = 1_000_000_000;
    assert_eq!(big, 1000000000);

    // EXERCISE: experiment with other data types
    let a = 64usize;
    let b: isize = 42;

    // EXERCISE: do some operations on integers
}

#[test]
fn addressing_integer_under_overflow() {
    // By default when Rust compile in debug mode only
    // integer overflow are checked
    let mut overflow_u8 = 0u8;

    // We will overflow u8 by 4
    for x in 0..1000 {
        //overflow_u8 += 1
        overflow_u8 = overflow_u8.saturating_add(1);
    }

    println!("{overflow_u8}");
}

/// IEEE 754 floating-point numbers:
/// | Type   | Size (bits) | Precision      | Notes               |
/// |--------|-------------|----------------|---------------------|
/// | `f32`  | 32          | Single         | ~6 decimal digits   |
/// | `f64`  | 64          | Double         | Default; ~15 digits |
///
/// Literal suffixes: `3.14f32`, `6.28f64`.
/// Special values: `inf`, `-inf`, `NaN` (e.g., `f64::NAN`).
#[test]
fn scalar_data_types_float() {
    // floating points must be defined with a dot even for round values
    let round = 42f32;
    let another_round: f32 = 42.0;

    assert_eq!(round, another_round);
}

/// | Type   | Values      | Size (bits)    | Notes               |
/// |--------|-------------|----------------|---------------------|
/// | `bool` | `true`/`false` | 8 (1 byte)   | No implicit casting |
#[test]
fn scalar_data_types_boolean() {
    let is_rust_awesome = true;
    let is_java_better = false;

    assert!(!(is_rust_awesome && is_java_better));
}

/// | Type   | Size (bits) | Range                  | Notes                     |
/// |--------|-------------|------------------------|---------------------------|
/// | `char` | 32          | Unicode scalar (U+0 to U+D7FF, U+E000 to U+10FFFF) | UTF-8 encoded; 4 bytes |
#[test]
fn scalar_data_types_char() {
    // chars must be single quoted, double quotes are for strings
    let c = 'C';
    assert_eq!(0x43 as char, c);

    // chars can be unicode characters
    let ferry = '\u{1f980}';
    assert_eq!(ferry, '🦀');
}

/// A **tuple** is a fixed-size, heterogeneous collection of values.
/// - **Fixed-size**: The number of elements is part of the type (e.g., `(i32, String)` is different from `(i32, String, bool)`).
/// - **Heterogeneous**: Elements can have different types.
/// - **Access**: Elements are accessed by index (e.g., `tuple.0`, `tuple.1`).
/// - **Use Case**: Ideal for grouping related but distinct values temporarily.
///
/// # Notes
/// - Tuples are stack-allocated and have no runtime overhead.
/// - Use structs for named fields if the data has semantic meaning.
#[test]
fn compound_types_tuple() {
    // Tuples are heterogenous so they can hold different data types
    let tup = (500, 6.4, 1);

    println!("tup={tup:?}");
}

/// An **array** is a fixed-size, homogeneous collection of elements stored contiguously in memory.
/// - **Fixed-size**: The length is part of the type (`[T; N]`), known at compile time.
/// - **Homogeneous**: All elements must be of the same type `T`.
/// - **Stack-allocated**: Stored directly on the stack (unlike `Vec`, which is heap-allocated).
/// - **Access**: Elements are accessed by index (e.g., `array[0]`), with bounds checked at runtime.
///
/// # Key Notes
/// - **Bounds Safety**: Accessing out-of-bounds indices causes a runtime panic.
/// - **Use Case**: Ideal for performance-critical code where size is known and fixed.
/// - **Iteration**: Can be iterated with `.iter()` or `.into_iter()`.
#[test]
fn compound_types_array() {
    // Here the length is 6
    let mut integers = [1, 2, 3, 4, 5, 6];
    // We can access array by index
    println!("first item={}", integers[0]);

    // Even though the length is immutable, values inside the array may be muted if needed.
    integers[0] = 42;
    println!("first item={}", integers[0]);

    // Array initialization
    let zeros = [0u8; 7]; // notation: [DEFAULT_VALUE; ARRAY_SIZE]
    println!("length of zeros={}", zeros.len());
    println!("zeros={zeros:?}");

    for i in 0..10 {
        let r = integers.get_mut(i);
    }

    // EXERCISE: initialize a u64 array with all values being 42
}

/// # Destructuring introduction in Rust
///
/// Destructuring is the process of **breaking down complex data types** into their individual components.
/// It allows you to extract values from structs, enums, tuples, and arrays by pattern matching their structure.
///
#[test]
fn destructuring_introduction() {
    let tup = (500, 6.4, 1);

    // we destructure tup
    // the compiler doesn't warn about unused variables/parameters prefixed with `_`
    let (_, y, _) = tup;

    println!("destructured only y={y}");

    // we can do partial destructuring
    // this means take x and ignore the rest
    let (x, ..) = tup;
    println!("destructured only x={x}");

    let array = [42, 1337];
    let [a, b] = array;
    // one can also destructure arrays
    println!("destructured array a={a} b={b}");
}

/// # `const` in Rust
///
/// A `const` is an **immutable value** that's known at compile time and inlined wherever it's used.
///
/// ## Key Characteristics:
/// - **Compile-time evaluation**: Value must be computable at compile time
/// - **Inlined**: Copied directly into every place it's used (no memory address)
/// - **Global scope**: Can be declared at module level or inside functions
/// - **Immutable**: Cannot be modified after declaration
/// - **No runtime overhead**: Value is baked into the binary
///
/// ## Common Use Cases:
/// - Mathematical constants
/// - Array sizes
/// - Configuration values
/// - Bit flags and masks
/// - Any value that's known at compile time and never changes
const BUFFER_SIZE: usize = 42;

#[test]
fn use_const() {
    // here we initialize with 0 a buffer of u8
    let array = [0u8; BUFFER_SIZE];
    println!("array length: {}", array.len())
}

// Mutability is an important concept in Rust as
// it helps the Borrow Checker verifying rules
// are applicable (we'll see that later).
// The idea is that we can mutate only what has been
// declared mutable.
#[test]
fn variable_mutability() {
    let x = 5;
    // EXERCISE: uncomment the following line
    // x = 2;
    println!("{x}");
}

#[test]
fn variable_shadowing() {
    let x = 5;

    {
        // this is a new code block coming with its own
        // variables
        let x = "hello world";
        println!("x in block x={x}");
        // this is a concept we will see later
        // but at the end of this block the x
        // we defined here goes out of scope and
        // gets dropped
    }

    println!("outer x={x}");
}

#[test]
fn scalar_data_types_integer() {
    // Signed (i) and unsigned (u) integers in fixed/variable sizes:
    // | Type   | Size (bits) | Range                          | Notes                          |
    // |--------|-------------|--------------------------------|--------------------------------|
    // | `i8`   | 8           | -128 to 127                    |                                |
    // | `u8`   | 8           | 0 to 255                       | Byte-sized; used for raw data  |
    // | `i16`  | 16          | -32,768 to 32,767              |                                |
    // | `u16`  | 16          | 0 to 65,535                    |                                |
    // | `i32`  | 32          | -2³¹ to 2³¹-1                  | Default for integers           |
    // | `u32`  | 32          | 0 to 2³²-1                     |                                |
    // | `i64`  | 64          | -2⁶³ to 2⁶³-1                  |                                |
    // | `u64`  | 64          | 0 to 2⁶⁴-1                     |                                |
    // | `i128` | 128         | -2¹²⁷ to 2¹²⁷-1                |                                |
    // | `u128` | 128         | 0 to 2¹²⁸-1                    |                                |
    // | `isize`| 32/64*      | Platform-dependent             | Pointer-sized (e.g., indexing) |
    // | `usize`| 32/64*      | 0 to 2³²-1 or 2⁶⁴-1            | Used for collection indices    |
    //
    // *Depends on CPU architecture (e.g., 64-bit on x86_64).
    //
    // Literal suffixes: `42i8`, `42u16`, `42isize`.
    // Supports underscores for readability: `1_000_000`.

    // we declare the type
    let small: u8 = 42;
    // this is equivalent to the above notation
    let another = 42u8;

    assert_eq!(another, small);

    let big: u64 = 1_000_000_000;
    assert_eq!(big, 1000000000)

    // EXERCISE: experiment with other data types

    // EXERCISE: do some operations on integers
}

#[test]
fn addressing_integer_under_overflow() {
    // By default when Rust compile in debug mode only
    // integer overflow are checked

    let mut overflow_u8 = 0u8;
    // We will overflow u8 by 4
    for _ in 0..260 {
        overflow_u8 += 1
    }
    println!("{overflow_u8}")
}

#[test]
fn scalar_data_types_float() {
    // IEEE 754 floating-point numbers:
    // | Type   | Size (bits) | Precision      | Notes               |
    // |--------|-------------|----------------|---------------------|
    // | `f32`  | 32          | Single         | ~6 decimal digits   |
    // | `f64`  | 64          | Double         | Default; ~15 digits |
    //
    // Literal suffixes: `3.14f32`, `6.28f64`.
    // Special values: `inf`, `-inf`, `NaN` (e.g., `f64::NAN`).

    // floating points must be defined with a dot even for round values
    let round = 42f32;
    let another_round: f32 = 42.0;

    assert_eq!(round, another_round);
}

#[test]
fn scalar_data_types_boolean() {
    // | Type   | Values      | Size (bits)    | Notes               |
    // |--------|-------------|----------------|---------------------|
    // | `bool` | `true`/`false` | 8 (1 byte)   | No implicit casting |

    let is_rust_awesome = true;
    let is_java_better = false;

    assert!(!(is_rust_awesome && is_java_better));
}

#[test]
fn scalar_data_types_char() {
    // | Type   | Size (bits) | Range                  | Notes                     |
    // |--------|-------------|------------------------|---------------------------|
    // | `char` | 32          | Unicode scalar (U+0 to U+D7FF, U+E000 to U+10FFFF) | UTF-8 encoded; 4 bytes |

    // chars must be single quoted, double quotes are for strings
    let c = 'C';
    assert_eq!(0x43 as char, c);

    // chars can be unicode characters
    let ferry = '\u{1f980}';
    assert_eq!(ferry, '🦀');
}

#[test]
fn compound_types_tuple() {
    let tup = (500, 6.4, 1);

    // Here we DESTRUCTURE tup. DESTRUCTURING is something
    // used quite intensively in Rust and applies to other
    // types such as structures and enums (seen later).
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // one can also access tuple items by index
    println!("x={} y={} z={}", tup.0, tup.1, tup.2)
}

#[test]
fn compound_types_array() {
    // A fixed-size, contiguous collection of elements of the SAME TYPE, with FIXED length known at compile time.
    // Arrays are stack-allocated (unless boxed) and have a strict, immutable length.

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

    // EXERCISE: initialize a u64 array with all values being 42
}

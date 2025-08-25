# Primitive Types

## Scalar Types
| Type      | Description                                                                 | Example Values          |
|-----------|-----------------------------------------------------------------------------|-------------------------|
| Integers  | Signed (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`) and unsigned (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`). | `42`, `-5i8`, `0xFFu8`   |
| Floating  | `f32` and `f64` for floating-point numbers.                                 | `3.14`, `-0.001f32`     |
| Boolean   | `bool` for true/false values.                                               | `true`, `false`         |
| Character | `char` for a single Unicode scalar value (4 bytes).                        | `'a'`, `'😊'`, `'Z'`     |

## Text Types
| Type            | Description                                                                 | Example Values          |
|-----------------|-----------------------------------------------------------------------------|-------------------------|
| String Literal  | `&'static str`: Immutable, UTF-8 encoded string slice (e.g., `"hello"`).   | `"hello"`, `"Rust"`     |
| String Slice    | `&str`: Borrowed string data (e.g., substring or string literal).           | `&"hello"[1..3]`        |
| String          | `String`: Growable, heap-allocated UTF-8 string.                            | `String::from("hello")` |

## Compound Types
| Type      | Description                                                                 | Example Values          |
|-----------|-----------------------------------------------------------------------------|-------------------------|
| Tuples    | Fixed-size, heterogeneous collections.                                      | `(42, "hello")`         |
| Arrays    | Fixed-size, homogeneous collections.                                        | `[1, 2, 3]`, `["a", "b"]`|

## Other Primitives
| Type      | Description                                                                 | Example Values          |
|-----------|-----------------------------------------------------------------------------|-------------------------|
| Unit      | `()`: Represents an empty value or return type.                             | `()`                    |
| Pointers  | Raw pointers (`*const T`, `*mut T`) and references (`&T`, `&mut T`).       | `&x`, `&mut y`          |

### Key Notes:
- **String Literals**: The type of `"hello"` is `&'static str` (a string slice with a static lifetime).
- **String vs. &str**: `String` is owned and heap-allocated; `&str` is a borrowed view into string data.
- **Arrays vs. Slices**: Arrays (`[T; N]`) have a fixed size; slices (`&[T]`) are dynamically sized views.
- **Default Types**: `i32` for integers, `f64` for floats.

### Example Usage:
```rust
let integer: i32 = 42;
let float: f64 = 3.14;
let string_literal: &str = "hello"; // String literal
let owned_string: String = String::from("Rust");
let tuple: (i32, &str) = (42, "hello");
let array: [i32; 3] = [1, 2, 3];
```

# `Option` and `Result`

## `Option<T>`
**Purpose**: Represents an optional value—either `Some(T)` or `None`.

### Use Cases
- When a value **might be absent** (e.g., searching for an item in a list).
- Safer alternative to `null` or `nil`.

### Syntax
```rust
enum Option<T> {
    Some(T), // A value exists
    None,    // No value
}
```

### Common Methods
| Method               | Description                                                                 |
|----------------------|-----------------------------------------------------------------------------|
| `unwrap()`           | Returns the value inside `Some` or **panics** if `None`.                   |
| `unwrap_or(default)` | Returns the value or a `default` if `None`.                                |
| `expect(msg)`        | Like `unwrap()`, but with a custom panic message.                          |
| `map(f)`             | Applies function `f` to the contained `Some` value.                        |
| `and_then(f)`        | Chains `Option`-returning functions (like `flat_map`).                     |
| `is_some()`          | Returns `true` if the `Option` is `Some`.                                   |
| `is_none()`          | Returns `true` if the `Option` is `None`.                                   |
| `as_ref()`           | Converts `Option<T>` to `Option<&T>`.                                       |
| `ok_or(E)`           | Converts `Option<T>` to `Result<T, E>`, mapping `None` to `Err(E)`.         |

### Example
```rust
fn find_index(haystack: &[i32], needle: i32) -> Option<usize> {
    haystack.iter().position(|&x| x == needle)
}

let numbers = vec![1, 2, 3];
assert_eq!(find_index(&numbers, 2), Some(1));
assert_eq!(find_index(&numbers, 5), None);
```

## `Result<T, E>`
**Purpose**: Represents success (`Ok(T)`) or failure (`Err(E)`).

### Use Cases
- Error handling (e.g., file I/O, parsing).
- Explicitly forces handling of both success and failure cases.

### Syntax
```rust
enum Result<T, E> {
    Ok(T),  // Success, contains a value
    Err(E), // Failure, contains an error
}
```

### Common Methods
| Method               | Description                                                                 |
|----------------------|-----------------------------------------------------------------------------|
| `unwrap()`           | Returns the value inside `Ok` or **panics** if `Err`.                       |
| `unwrap_or(default)` | Returns the value or a `default` if `Err`.                                 |
| `expect(msg)`        | Like `unwrap()`, but with a custom panic message.                          |
| `map(f)`             | Applies function `f` to the contained `Ok` value.                          |
| `map_err(f)`         | Applies function `f` to the contained `Err` value.                         |
| `and_then(f)`        | Chains `Result`-returning functions (like `flat_map`).                     |
| `is_ok()`            | Returns `true` if the `Result` is `Ok`.                                     |
| `is_err()`           | Returns `true` if the `Result` is `Err`.                                    |
| `?` operator         | Propagates errors: returns `Ok` value or early-returns `Err`.               |

### Example
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

assert_eq!(divide(4.0, 2.0), Ok(2.0));
assert_eq!(divide(4.0, 0.0), Err(String::from("Division by zero")));
```

## Pattern Matching
Both `Option` and `Result` work seamlessly with `match` and `if let`.

### `match` Example
```rust
match find_index(&numbers, 2) {
    Some(index) => println!("Found at index: {}", index),
    None => println!("Not found"),
}

match divide(4.0, 0.0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

### `if let` Example
```rust
if let Some(index) = find_index(&numbers, 2) {
    println!("Found at index: {}", index);
}
```

## Combining `Option` and `Result`
- **`Option` to `Result`**:
  ```rust
  let some_value: Option<i32> = Some(42);
  let result: Result<i32, &str> = some_value.ok_or("No value");
  ```
- **`Result` to `Option`**:
  ```rust
  let ok_result: Result<i32, &str> = Ok(42);
  let option: Option<i32> = ok_result.ok();
  ```

## The `?` Operator
- **Purpose**: Simplifies error propagation.
- **Behavior**:
  - Unwraps `Ok` values.
  - Early-returns `Err` values (in functions returning `Result`).
  - Works similarly for `Option` (early-returns `None`).

### Example with `Result`
```rust
fn compute() -> Result<i32, String> {
    let a = divide(4.0, 2.0)?;
    let b = divide(a, 0.5)?;
    Ok(b as i32)
}
```

### Example with `Option`
```rust
fn first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&x| x % 2 == 0).copied()
}
```

## Why `Option` and `Result` Matter
- **Explicitness**: Forces developers to handle absence or failure.
- **Safety**: Eliminates null pointer exceptions and unchecked errors.
- **Composability**: Methods like `map`, `and_then`, and `?` enable clean, functional-style code.

## Best Practices
- **Avoid `unwrap()`**: Prefer `match`, `if let`, or `?` for robust error handling.
- **Use `?` for Propagation**: Simplifies chaining fallible operations.
- **Custom Error Types**: Use enums to define rich error types for `Result`.
  ```rust
  #[derive(Debug)]
  enum MathError {
      DivisionByZero,
      NegativeRoot,
  }
  ```


# Control Flow

## `if` Expressions
```rust
if condition {
    // code block
} else if another_condition {
    // code block
} else {
    // code block
}
```
- **Key Points**:
  - Conditions **must** be `bool` (no implicit conversion).
  - `if` is an **expression** (can return a value).
  - All branches must return the same type.

```rust
let number = if condition { 5 } else { 6 };
```

## `loop`
Infinite loop:
```rust
loop {
    println!("Loop forever!");
    break; // Exit the loop
}
```
- **Loop Labels**: Break/continue specific loops:
  ```rust
  'outer: loop {
      loop {
          break 'outer;
      }
  }
  ```
- **Return Values**: Use `break` with a value:
  ```rust
  let result = loop {
      break 42;
  };
  ```

## `while` Loops
```rust
while condition {
    // code block
}
```
- **Example**:
  ```rust
  while x < 10 {
      x += 1;
  }
  ```

## `for` Loops
```rust
for element in collection {
    // code block
}
```
- **Ranges**:
  ```rust
  for i in 0..5 { // 0 to 4
      println!("{}", i);
  }
  ```
- **Iterators**:
  ```rust
  for item in vec![1, 2, 3].iter() {
      println!("{}", item);
  }
  ```

## `match` Expressions
Powerful pattern matching:
```rust
match value {
    Pattern1 => expression1,
    Pattern2 => expression2,
    _ => default_expression, // Catch-all
}
```
- **Key Features**:
  - **Exhaustive**: All possible values must be handled.
  - **Patterns**: Match literals, ranges, structs, enums, etc.
  - **Destructuring**:
    ```rust
    match some_option {
        Some(x) => println!("Got: {}", x),
        None => println!("Got nothing"),
    }
    ```
  - **Guards**:
    ```rust
    match num {
        x if x % 2 == 0 => println!("Even"),
        _ => println!("Odd"),
    }
    ```

## `if let` Syntax
Concise pattern matching:
```rust
if let Some(x) = some_option {
    println!("Got: {}", x);
}
```
- **Equivalent** to:
  ```rust
  match some_option {
      Some(x) => println!("Got: {}", x),
      _ => (),
  }
  ```

## `while let` Loops
Loop while a pattern matches:
```rust
while let Some(x) = iterator.next() {
    println!("{}", x);
}
```

## Early Returns
- **`return`**: Exits the function.
- **`break`**: Exits the loop.
- **`continue`**: Skips to the next iteration.

## Control Flow in Expressions
- **`if`/`match`** can be used in assignments:
  ```rust
  let result = if condition { "yes" } else { "no" };
  let num = match some_option {
      Some(n) => n,
      None => 0,
  };
  ```

## Common Patterns
1. **Looping with Indices**:
   ```rust
   for (index, value) in vec![1, 2, 3].iter().enumerate() {
       println!("Index: {}, Value: {}", index, value);
   }
   ```

2. **Conditional Loops**:
   ```rust
   while let Ok(line) = reader.read_line(&mut buffer) {
       println!("{}", line);
   }
   ```

3. **Pattern Matching with Enums**:
   ```rust
   match some_result {
       Ok(value) => println!("Success: {}", value),
       Err(e) => println!("Error: {}", e),
   }
   ```

## Key Concepts
- **Expressions vs. Statements**:
  - `if`, `match`, and blocks (`{}`) are **expressions** (evaluate to a value).
  - `let`, `loop`, `while`, `for` are **statements** (no value).
- **No Parentheses**: Conditions don't need parentheses (unlike C-style languages).
- **No Fallthrough**: `match` doesn't fall through (unlike C `switch`).

## Example: FizzBuzz
```rust
for i in 1..=100 {
    match (i % 3, i % 5) {
        (0, 0) => println!("FizzBuzz"),
        (0, _) => println!("Fizz"),
        (_, 0) => println!("Buzz"),
        _ => println!("{}", i),
    }
}
```
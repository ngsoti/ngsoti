# Structs and Enums

## Structs
Structs let you **group related data** into a custom type.

### Defining Structs
```rust
struct User {
    username: String,
    email: String,
    active: bool,
}
```

### Tuple Structs
```rust
struct Point(i32, i32, i32); // No named fields
```

### Unit Structs
```rust
struct Empty; // No fields
```

### Usage
- **Instantiation**:
  ```rust
  let user = User {
      email: String::from("example@example.com"),
      username: String::from("user1"),
      active: true,
  };
  ```
- **Accessing Fields**:
  ```rust
  println!("Username: {}", user.username);
  ```

### Methods
- **Implementation**:
  ```rust
  impl User {
      fn greet(&self) {
          println!("Hello, {}!", self.username);
      }
  }
  ```
- **Associated Functions** (like static methods):
  ```rust
  impl User {
      fn new(email: String, username: String) -> Self {
          Self { email, username, active: true }
      }
  }
  ```

## Enums
Enums let you **define a type by enumerating its possible variants**.

### Defining Enums
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

### Usage
- **Pattern Matching**:
  ```rust
  match message {
      Message::Quit => println!("Quit"),
      Message::Move { x, y } => println!("Move to {}, {}", x, y),
      Message::Write(text) => println!("Text: {}", text),
      Message::ChangeColor(r, g, b) => println!("RGB: {}, {}, {}", r, g, b),
  }
  ```

### Enums with Data
- Each variant can hold different types and amounts of data.

### Option and Result
- **`Option<T>`**:
  ```rust
  enum Option<T> {
      Some(T),
      None,
  }
  ```
- **`Result<T, E>`**:
  ```rust
  enum Result<T, E> {
      Ok(T),
      Err(E),
  }
  ```

## Key Concepts
- **Structs**: Ideal for grouping data with a fixed set of fields.
- **Enums**: Ideal for representing a type that could be one of several possible variants.
- **Pattern Matching**: Use `match` or `if let` to handle enum variants.
- **Exhaustiveness**: Rust ensures all possible variants are handled in `match` expressions.

## Example: Struct and Enum Together
```rust
struct Circle {
    radius: f64,
}

enum Shape {
    Circle(Circle),
    Rectangle { width: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(c) => std::f64::consts::PI * c.radius * c.radius,
        Shape::Rectangle { width, height } => width * height,
    }
}
```

## Why Structs and Enums Matter
- **Structs**: Model data with clear, named fields.
- **Enums**: Model types with multiple possible states or forms.
- **Type Safety**: Compile-time checks for correct usage.
- **Expressiveness**: Combine with `impl` for methods and traits for shared behavior.

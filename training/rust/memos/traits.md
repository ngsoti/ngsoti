# Traits

## What Are Traits?
**Traits** define shared behavior that types can implement, similar to interfaces in other languages. They enable polymorphism and code reuse without inheritance.

## Defining Traits
```rust
trait Summary {
    // Required method (no default implementation)
    fn summarize(&self) -> String;

    // Default implementation
    fn describe(&self) -> String {
        format!("This is a summary: {}", self.summarize())
    }
}
```

## Implementing Traits
```rust
struct Article {
    headline: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} - {}", self.headline, self.content)
    }
}
```

## Trait Bounds
Restrict generic types to those implementing a trait.

### Syntax
```rust
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

### Multiple Trait Bounds
```rust
fn notify<T: Summary + Display>(item: &T) { ... }
```

### `where` Clause (Cleaner Syntax)
```rust
fn notify<T>(item: &T)
where
    T: Summary + Display,
{ ... }
```

## Default Implementations
Traits can provide default method implementations:
```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

## Trait Inheritance
```rust
trait DisplaySummary: Summary {
    fn display(&self);
}
```

## Derivable Traits
Common traits Rust can auto-implement:
- `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`

```rust
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}
```

## Trait Objects
Enable dynamic dispatch with `dyn Trait`:

```rust
fn dynamic_notify(item: &dyn Summary) {
    println!("{}", item.summarize());
}
```

- **Use Case**: When you need heterogeneous collections.
- **Trade-off**: Slight runtime overhead (vs. static dispatch).

## Associated Types
Define placeholder types in traits:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

## Generic Trait Implementation
```rust
impl<T: Display> Summary for T {
    fn summarize(&self) -> String {
        format!("Displayable value: {}", self)
    }
}
```

## Common Standard Library Traits

| Trait          | Purpose                                                                 |
|----------------|-------------------------------------------------------------------------|
| `Debug`        | Enables formatting for debugging (`{:?}`).                             |
| `Display`      | Enables user-friendly formatting (`{}`).                                |
| `Clone`        | Enables explicit duplication with `.clone()`.                           |
| `PartialEq`    | Enables equality comparisons (`==`, `!=`).                              |
| `Iterator`     | Defines iterator behavior (`next()`, `map()`, etc.).                    |
| `From`/`Into`  | Enables type conversions.                                               |
| `Default`      | Provides a default value (`T::default()`).                              |

## Trait Methods
- **Required Methods**: Must be implemented by types.
- **Default Methods**: Optional to override.

## Example: Custom Trait
```rust
trait Greet {
    fn greet(&self) -> String;
}

impl Greet for String {
    fn greet(&self) -> String {
        format!("Hello, {}!", self)
    }
}

fn say_hello<T: Greet>(item: T) {
    println!("{}", item.greet());
}
```

## Why Traits Matter
- **Polymorphism**: Write generic code that works with multiple types.
- **Code Reuse**: Share behavior across unrelated types.
- **Abstraction**: Define interfaces without inheritance.
- **Zero-Cost Abstraction**: Static dispatch (generics) has no runtime overhead.

## Best Practices
- Use traits for **shared behavior**, not data.
- Prefer **static dispatch** (generics) over dynamic dispatch (`dyn`) for performance.
- Use **default implementations** to reduce boilerplate.
- Document trait purposes and invariants with `///` comments.

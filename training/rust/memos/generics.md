# Generics

## What Are Generics?
Generics allow you to write **flexible, reusable code** that works with multiple types without sacrificing performance or type safety. They abstract over types, enabling functions, structs, enums, and traits to operate on any type that meets specified constraints.

## Generic Functions
- **Syntax**: Use angle brackets `<>` to declare generic type parameters.
- **Example**:
  ```rust
  fn largest<T: PartialOrd>(list: &[T]) -> &T {
      let mut largest = &list[0];
      for item in list {
          if item > largest {
              largest = item;
          }
      }
      largest
  }
  ```
  - `T` is a placeholder for any type that implements `PartialOrd`.

## Generic Structs and Enums
- **Structs**:
  ```rust
  struct Point<T> {
      x: T,
      y: T,
  }
  ```
  - `Point<i32>` and `Point<f64>` are different types.

- **Enums**:
  ```rust
  enum Option<T> {
      Some(T),
      None,
  }
  ```
  - `Option<T>` can hold any type `T`.

## Generic Methods
- **Implementation**:
  ```rust
  impl<T> Point<T> {
      fn x(&self) -> &T {
          &self.x
      }
  }
  ```
  - Methods can use the struct's generic types.

## Trait Bounds
- **Purpose**: Restrict generic types to those that implement specific traits.
- **Syntax**: `T: Trait` or `where T: Trait`.
- **Example**:
  ```rust
  fn print_and_return<T: Display>(value: T) -> T {
      println!("{}", value);
      value
  }

  // The above syntax is equivalent to 
  fn print_and_return<T>(value: T) -> T
  where T: Display
  {
      println!("{}", value);
      value
  }
  ```
  - `T` must implement the `Display` trait.

## Multiple Generic Types
- **Syntax**: Separate types with commas.
- **Example**:
  ```rust
  struct Pair<T, U> {
      first: T,
      second: U,
  }
  ```
  - `Pair<i32, String>` and `Pair<&str, f64>` are valid.

## Generic Traits
- **Example**:
  ```rust
  trait Summary<T> {
      fn summarize(&self, data: T) -> String;
  }
  ```
  - Traits can be generic over types or lifetimes.

## Performance
- **Zero-Cost Abstraction**: Generics are resolved at compile time, so there's no runtime overhead.
- **Monomorphization**: The compiler generates specific implementations for each concrete type used.

## Common Use Cases
1. **Data Structures**: Vectors (`Vec<T>`), hash maps (`HashMap<K, V>`).
2. **Algorithms**: Functions that work on any type (e.g., sorting, searching).
3. **Traits**: Abstract behavior for multiple types.

## Key Concepts
- **Type Safety**: Generics ensure compile-time checks for correct types.
- **Code Reuse**: Write once, use with any compatible type.
- **Flexibility**: Combine with traits to constrain behavior.

## Example: Generic Function
```rust
fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}
```
- Swaps values of any type `T`.

## Why Generics Matter
- **Avoid Duplication**: Write one implementation for many types.
- **Type Safety**: Catch type errors at compile time.
- **Efficiency**: No runtime cost compared to concrete types.

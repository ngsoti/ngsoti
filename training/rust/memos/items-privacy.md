# Items Privacy

## Basic Privacy Rules
- **Default Privacy**: All items are **private** by default in Rust.
- **Visibility**: Items are only accessible within their defining module.
- **Public Access**: Use `pub` to expose items to parent modules or externally.

## Privacy for Types

### Structs
```rust
// Private struct (default)
struct PrivatePoint {
    x: i32,
    y: i32,
}

// Public struct
pub struct Point {
    x: i32,
    y: i32,
}

// Public struct with mixed visibility fields
pub struct MixedPoint {
    pub x: i32,    // Public field
    y: i32,        // Private field
}
```
- Structs and their fields are private by default.
- Fields must be explicitly marked `pub` to be accessible.

### Enums
```rust
// Private enum (default)
enum PrivateColor {
    Red,
    Green,
    Blue,
}

// Public enum
pub enum Color {
    Red,
    Green,
    Blue,
}
```
- Enums are private by default.
- All variants of a public enum are public.

### Traits
```rust
// Private trait (default)
trait PrivateGreeter {
    fn greet(&self);
}

// Public trait
pub trait Greeter {
    fn greet(&self);
}
```
- Traits are private by default.
- Methods in public traits are public by default.

## Constants and Static Variables

### Constants
```rust
// Private constant (default)
const PRIVATE_PI: f64 = 3.14159;

// Public constant
pub const PI: f64 = 3.14159;
```
- Constants are private by default.

### Static Variables
```rust
// Private static (default)
static PRIVATE_COUNTER: u32 = 0;

// Public static
pub static COUNTER: u32 = 0;
```
- Static variables are private by default.
- Modifying public statics requires `unsafe` or interior mutability.

## Functions and Methods

### Functions
```rust
// Private function (default)
fn private_function() {}

// Public function
pub fn public_function() {}
```
- Functions are private by default.

### Methods
```rust
pub struct Point { x: i32, y: i32 }

impl Point {
    // Private method (default)
    fn private_method(&self) {}

    // Public method
    pub fn public_method(&self) {}
}
```
- Methods are private by default.

## Advanced Visibility
```rust
// Crate-wide visibility
pub(crate) fn crate_visible() {}

// Module-specific visibility
pub(in path::to::module) fn limited_visible() {}

// Parent module visibility
pub(super) fn super_visible() {}
```

## Best Practices
1. **Minimize Public API**: Keep implementation details private.
2. **Use Modules**: Organize code with clear visibility boundaries.
3. **Document Public Items**: Use `///` for public API documentation.
4. **Use `pub(crate)`**: For crate-internal items.
5. **Use `pub(in path)`**: For fine-grained visibility control.
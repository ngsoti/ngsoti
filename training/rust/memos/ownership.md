# Ownership in Rust

Rust's ownership system is a set of rules that govern how memory is managed at compile time.
It enables memory safety without garbage collection by tracking ownership of values.

## Core Ownership Rules:

1. **Each value has a single owner**:
   - Every value in Rust has a variable that's its owner
   - There can only be one owner at a time

2. **When the owner goes out of scope, the value is dropped**:
   - Memory is automatically returned when variables go out of scope
   - The `Drop` trait defines what happens during cleanup

3. **Ownership can be moved**:
   - When assigned to another variable or passed to a function
   - After move, the original variable can no longer be used
   ```rust
   let s1 = String::from("hello");
   let s2 = s1;  // Ownership moves from s1 to s2
   // println!("{}", s1);  // ERROR: value borrowed here after move
   ```

## Key Concepts:

### 1. Move Semantics:
- For types that don't implement `Copy` (like String, Vec)
- Ownership is transferred rather than copied
- Original variable becomes invalid

### 2. Copy Semantics:
- For types that implement `Copy` (primitives, tuples of Copy types)
- Values are copied rather than moved
- Original variable remains valid
```rust
let x = 5;
let y = x;  // Copy occurs, x is still valid
```

### 3. Borrowing:
- **Immutable borrow (`&T`)**: Multiple allowed
- **Mutable borrow (`&mut T`)**: Exclusive (only one at a time)
- Borrow checker enforces these rules at compile time
```rust
let s = String::from("hello");
let r1 = &s;  // OK
let r2 = &s;  // OK
let r3 = &mut s;  // ERROR: cannot borrow as mutable while immutable borrows exist
```

### 4. Borrowing Rules:
- You can have either:
  - Any number of immutable references, OR
  - Exactly one mutable reference
- References must always be valid (no dangling references)

### 5. Lifetimes:
- Every reference has a lifetime ('a)
- Ensures references don't outlive the data they point to
- Mostly inferred by the compiler
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
```

## Ownership in Practice:

### Function Parameters:
- Passing values can move or copy ownership
- Return values can transfer ownership back
```rust
fn takes_ownership(s: String) { ... }  // s is moved
fn makes_copy(i: i32) { ... }          // i is copied
```

### Return Values:
- Returning values transfers ownership
- Multiple return values can be used with tuples
```rust
fn gives_ownership() -> String { ... }
```

## Special Cases:

### 1. Clone:
- Explicit deep copy when you need independent ownership
```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // Deep copy occurs
```

### 2. Copy Trait:
- Types that are stored on the stack implement `Copy`
- No ownership transfer occurs on assignment

### 3. Rc<T> and Arc<T>:
- Reference counting for shared ownership
- `Rc<T>` for single-threaded, `Arc<T>` for multi-threaded
```rust
use std::rc::Rc;
let a = Rc::new(5);
let b = Rc::clone(&a);  // Increments reference count
```

### 4. RefCell<T> and Interior Mutability:
- Runtime-enforced borrowing rules
- Allows mutation through immutable references
```rust
let x = RefCell::new(5);
let y = x.borrow_mut();  // Runtime borrow check
```

## Why Ownership Matters:
- Prevents data races at compile time
- Eliminates dangling pointers
- Enables fearless concurrency
- No garbage collector needed
- Memory safety without runtime overhead

## Common Patterns:
- Returning owned values from functions
- Using references to avoid ownership transfer
- Chaining method calls with ownership transfers
- Using `match` to handle ownership changes
- Implementing `Drop` for custom cleanup

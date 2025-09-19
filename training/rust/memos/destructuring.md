# Destructuring

Destructuring is the process of **breaking down complex data types** into their individual components.
It allows you to extract values from structs, enums, tuples, and arrays by pattern matching their structure.

## Main Forms of Destructuring:

1. **Tuple Destructuring**:
   ```rust
   let pair = (1, "hello");
   let (num, text) = pair;  // Destructures into num and text
   ```

2. **Struct Destructuring**:
   ```rust
   struct Point { x: i32, y: i32 };
   let point = Point { x: 10, y: 20 };
   let Point { x, y } = point;  // Destructures into x and y
   ```

3. **Array/Slice Destructuring**:
   ```rust
   let arr = [1, 2, 3];
   let [a, b, c] = arr;  // Destructures into a, b, c
   ```

4. **Enum Destructuring** (especially with `match`):
   ```rust
   enum Message { Quit, Move { x: i32, y: i32 } }
   match msg {
       Message::Move { x, y } => { /* use x and y */ }
       _ => (),
   }
   ```

## Where Destructuring Can Be Used:

- **Variable declarations with `let`**:
  ```rust
  let (a, b) = (1, 2);
  ```

- **Function parameters**:
  ```rust
  fn print_point(Point { x, y }: Point) { ... }
  ```

- **Match expressions**:
  ```rust
  match some_option {
      Some(value) => println!("{}", value),
      None => (),
  }
  ```

- **If-let expressions**:
  ```rust
  if let Some(x) = some_option { ... }
  ```

- **Closure parameters**:
  ```rust
  let closure = |(x, y)| x + y;
  ```

- **Loop patterns**:
  ```rust
  for (index, value) in vec.iter().enumerate() { ... }
  ```

## Advanced Destructuring Features:

1. **Partial Destructuring**:
   ```rust
   let (x, ..) = (1, 2, 3);  // Only extract first element
   let Point { x, .. } = point;  // Only extract x from struct
   ```

2. **Renaming Fields**:
   ```rust
   let Point { x: x_coord, y: y_coord } = point;
   ```

3. **Nested Destructuring**:
   ```rust
   let ((a, b), c) = ((1, 2), 3);
   ```

4. **Ignoring Values**:
   ```rust
   let (x, _) = (1, 2);  // Ignore second element
   ```

5. **Ref and Ref Mut Patterns**:
   ```rust
   let Point { ref x, ref mut y } = point;  // Borrow components
   ```

## Destructuring with Patterns:

Rust's pattern matching syntax enables powerful destructuring:
- Guard clauses
- Multiple patterns
- Range patterns

## Important Notes:
- Destructuring moves values by default (use `ref` to borrow)
- For structs, you don't need to destructure all fields
- Destructuring works with any type that implements the appropriate patterns
- The compiler checks for exhaustive patterns in matches
- Destructuring is zero-cost - it doesn't create intermediate copies

## Common Use Cases:
- Extracting fields from structs/tuples
- Handling enum variants (especially Option and Result)
- Iterating with indices and values
- Complex function parameter handling
- Concise error handling

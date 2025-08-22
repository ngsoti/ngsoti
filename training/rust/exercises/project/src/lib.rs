//! Implement a `Queue` trait and two structures implementing it `Fifo` (first in first out) and `Lifo` (last in first out).
//!
//! Constrains:
//!    - follow the trait implemented below (you can add methods if you like)
//!    - implement conversion from &[T]
//!    - implement conversion into Vec<T>

enum Error {}

trait Queue<T> {
    fn with_capacity(cap: usize) -> Self;
    // this function returns a reference to the next element to pop
    fn peek(&self) -> Option<&T>;
    // this function must pop the next item according
    // to the kind of queue it is
    fn pop(&mut self) -> Option<T>;
    // this function must put the item in the queue
    // Error has to be defined
    fn put(&mut self, item: T) -> Result<(), Error>;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
}

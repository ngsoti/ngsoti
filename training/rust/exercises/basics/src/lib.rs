// Mutability is an important concept in Rust as
// it helps the Borrow Checker verifying rules
// are applicable (we'll see that later).
// The idea is that we can mutate only what has been
// declared mutable.
#[test]
fn variable_mutability() {
    let x = 5;
    // EXERCISE: uncomment the following line
    // x = 2;
    println!("{x}");
}

#[test]
fn variable_shadowing() {
    let x = 5;

    {
        // this is a new code block coming with its own
        // variables
        let x = "hello world";
        println!("x in block x={x}");
        // this is a concept we will see later
        // but at the end of this block the x
        // we defined here goes out of scope and
        // gets dropped
    }

    println!("outer x={x}");
}

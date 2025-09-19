use std::f64::consts::PI;

/// # Rust Enums Explained
///
/// **Enums (enumerations) let you define a type by listing possible variants:**
#[test]
fn enums_1() {
    // this is C style enum where we
    // can define possible values
    #[derive(Debug)]
    enum ShapeKind {
        Circle,
        Rectangle,
    }

    // in order to use an enum variant
    // we use the following notation
    let ck = ShapeKind::Circle;
    let rk = ShapeKind::Rectangle;
    println!("ck={ck:?} rk={rk:?}");

    // we can implement enums as we do with
    // structures
    impl ShapeKind {
        fn new_circle() -> Self {
            Self::Circle
        }

        fn new_rectangle() -> Self {
            Self::Rectangle
        }
    }

    let ck = ShapeKind::new_circle();
    let rk = ShapeKind::new_rectangle();
    println!("ck={ck:?} rk={rk:?}");
}

/// The enums we have seen above was just an introduction.
/// Most of the use of enums in Rust is done with data carrying
/// enums. Those can be used to carry data for each variant.
/// Think about all the previous example about geometrical shapes,
/// and how you would solve the following: I want a Shape type that
/// can either be Circle or a Rectangle.
///
/// Rust isn't an Object Oriented Programming language so there
/// is not concept such as inheritance. Instead Rust provides
/// the necessary to achieve the same end functionality as an OOP.
/// Enum is one of them we can leverage.
///
/// Enum is an important concept in Rust and mastering it helps a lot
/// in creating both clear and performant code.
#[test]
fn data_carrying_enums() {
    // an enum can be a mix of data carrying and non data carrying variants
    #[derive(Debug)]
    enum Shape {
        // Dot doesn't need to carry info
        Dot,
        // Circle just needs a radius
        Circle(u32),
        // Rectangle is defined in a struct style
        // notation not to confuse width and height
        // another valid notation would be
        // Rectangle(u32, u32),
        Rectangle { width: u32, height: u32 },
    }

    let ds = Shape::Dot;
    println!("ds={ds:?}");

    let cs = Shape::Circle(5);
    println!("cs={cs:?}");

    let rs = Shape::Rectangle {
        width: 3,
        height: 5,
    };
    println!("rs={rs:?}");
}

#[test]
fn data_carrying_enums2() {
    // this example is just to show alternative enum
    // definition. This does not really make sense in
    // this simple example but when structures becomes
    // more complex it may be clearer to define enums
    // as follow.

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    #[derive(Debug)]
    struct Circle {
        radius: u32,
    }

    #[derive(Debug)]
    enum Shape {
        Dot,
        Circle(Circle),
        Rectangle(Rectangle),
    }

    let ds = Shape::Dot;
    println!("ds={ds:?}");

    // the following notation is pretty verbose but this
    // could definitely be shortened by implementing methods
    let cs = Shape::Circle(Circle { radius: 2 });
    println!("cs={cs:?}");

    let rs = Shape::Rectangle(Rectangle {
        width: 3,
        height: 5,
    });
    println!("rs={rs:?}");
}

/// # Pattern Matching
///
/// **Pattern matching lets you compare values against patterns and execute code based on which pattern matches.**
///
/// ## Basic Syntax
/// ```rust
/// match VALUE {
///     PATTERN1 => EXPRESSION1,
///     PATTERN2 => EXPRESSION2,
///     _ => DEFAULT_EXPRESSION,  // Catch-all
/// }
/// ```
///
/// ## Key Features
/// 1. **Exhaustive**: Must handle all possible cases
/// 2. **Expressions**: Each arm returns a value
/// 3. **Powerful patterns**: Can destructure and bind variables
#[test]
fn enum_pattern_matching() {
    #[derive(Debug)]
    enum Shape {
        Dot,
        Circle(u32),
        Rectangle { width: u32, height: u32 },
    }

    // to illustrate pattern matching lets implement
    // a function for the Shape enum
    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Dot => 0.0,
                // as we have seen earlier with tuples what we do
                // here is called DESTRUCTURING
                Shape::Circle(r) => PI * (*r as f64).powi(2),
                Shape::Rectangle { width, height } => *width as f64 * (*height) as f64,
            }
        }
    }

    let dot = Shape::Dot;
    println!("dot area={}", dot.area());

    let circle = Shape::Circle(2);
    println!("circle area={}", circle.area());

    let rect = Shape::Rectangle {
        width: 5,
        height: 3,
    };
    println!("rectangle area={}", rect.area());

    // This demonstrates how enums with pattern matching can achieve
    // polymorphism similar to inheritance in OOP languages, but with key differences:
    //
    // Similarities to inheritance:
    // - Different "types" (variants) share common behavior (methods)
    // - You can add methods to the enum that work differently for each variant
    // - It provides runtime polymorphism (different behavior based on type)
    //
    // Differences from classical inheritance:
    // - No shared base class with virtual methods
    // - No method overriding - all methods are defined in one impl block
    // - No dynamic dispatch by default (unless you use trait objects)
    // - More type safety - compiler checks all cases are handled
    // - Data and behavior are more explicitly separated
}

/// Pattern matching must always handle ALL the cases
/// otherwise the compiler will complain about it
#[test]
fn pattern_matching_exhaustiveness() {
    #[derive(Debug)]
    enum Shape {
        Dot,
        Circle(u32),
        Rectangle { width: u32, height: u32 },
    }

    impl Shape {
        fn print_shape(&self) {
            print!("We're all shapes but ");
            match self {
                // EXERCISE: try to comment one of the matches
                Self::Dot => println!("I'm a dot"),
                Self::Circle(_) => println!("I'm a circle"),
                Self::Rectangle {
                    width: _,
                    height: _,
                } => println!("I'm a rectangle"),
            }
        }
    }

    Shape::Dot.print_shape();
    Shape::Circle(0).print_shape();
    Shape::Rectangle {
        width: 0,
        height: 0,
    }
    .print_shape();

    println!();

    impl Shape {
        fn print_only_dot(&self) {
            match self {
                Self::Dot => println!("I'm a dot"),
                // This is the default case: the compiler
                // knows we deliberately want to default to that.
                // There is no place for doubt.
                //
                // !!! WE HAVE TO BE CAREFUL WITH THAT NOTATION !!!
                // Especially in case we create new variants. We
                // could easily forget to update match cases
                _ => {
                    println!("we don't care about the others")
                }
            }
        }
    }

    Shape::Dot.print_only_dot();
    Shape::Circle(0).print_only_dot();

    impl Shape {
        fn print_only_circle(&self) {
            match self {
                Self::Circle(_) => println!("I am a circle"),
                // This is the preferred way to handle default case,
                // by explicitly matching the variants we want to
                // ignore. This prevents from forgetting implementations
                // when adding new variants.
                //
                // EXERCISE: add a new variant to the Shape enum and
                // see what is happening.
                Self::Dot
                | Self::Rectangle {
                    width: _,
                    height: _,
                } => {
                    println!("we don't care about the others")
                }
            }
        }
    }

    println!();

    Shape::Dot.print_only_circle();
    Shape::Circle(0).print_only_circle();
}

/// Pattern matching a powerful construct we can
/// use for different types in Rust, not only enums
#[test]
fn pattern_matching_beyond_enums_1() {
    // we can use pattern matching to match integers
    fn what_s_this_int(i: i32) {
        match i {
            0 => println!("{i} is zero"),
            42 => {
                println!(
                    "{i} is The Answer to the Ultimate Question of Life, The Universe, and Everything"
                )
            }
            _ => {
                println!("{i} is not important")
            }
        }
    }

    what_s_this_int(0);
    what_s_this_int(42);
    what_s_this_int(1337);
}

#[test]
fn pattern_matching_beyond_enums_2() {
    // we can use pattern matching to check if a number
    // is within a range
    fn what_s_this_int(i: i32) {
        match i {
            // Notation to define a range from 0 to 9
            0..10 => println!("{i} is in [0; 9["),
            // Notation to define a range from 10 to 20
            10..=20 => println!("{i} is in [10; 20]"),
            42 => {
                println!(
                    "{i} is The Answer to the Ultimate Question of Life, The Universe, and Everything"
                )
            }
            // We always need to provide a default
            _ => {
                println!("{i} is not important")
            }
        }
    }

    what_s_this_int(0);
    what_s_this_int(20);
    what_s_this_int(42);
    what_s_this_int(1337);
}

#[test]
fn pattern_matching_beyond_enums_3() {
    fn tell_me(s: &str) {
        println!("My answer to: {s}");

        match s {
            "What is the answer to the ultimate question of life, the universe, and everything ?" =>
            {
                println!("Wait I'm computing ");
                for _ in 0..100 {
                    print!(".")
                }
                println!("\n42")
            }
            "What is the color of the white horse of Henry the 4th?" => println!("white"),
            // We always need to provide a default
            _ => println!("I don't have an answer"),
        }
    }

    tell_me("What is the answer to the ultimate question of line, the universe, and everything ?");
    println!();
    tell_me("What is the color of the white horse of Henry the 4th?");
    println!();
    tell_me("What is your favorite color?");
}

/// EXERCISE:
/// - make an enum which can carry either an int or a string
/// - make a function named `tell_me` taking such an enum as parameter
///     - printing the range as above if it is an int
///     - answering to the question as above if it is a string
#[test]
fn enum_exercise() {
    enum About {}
}

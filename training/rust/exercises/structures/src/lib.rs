use std::fmt::{Debug, Display};

#[test]
fn simple_structure() {
    // derive calls a procedural macro (rust generating rust)
    // it implements the function needed to print a rectangle
    // with the ? (debug) modifier
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let r = Rectangle {
        width: 10,
        height: 30,
    };
    let area = r.width * r.height;

    // {?} is used to print a debug version of our struct
    println!("{r:?} area={area}");
}

#[test]
fn tuple_structure() {
    // a structure can also be defined as a tuple
    #[derive(Debug)]
    struct Rectangle(u32, u32); // (width, height)

    let r = Rectangle(10, 30);
    // we access structure fields by index
    let area = r.0 * r.1;

    println!("{r:?} area={area}");
}

#[test]
fn unit_structure() {
    // used to represent a structure without field
    // mostly used when only some methods needs to
    // be implemented traits (hared behavior interface contract)
    // on the structure.
    struct UnitStruct;

    // here we implement the Display trait used to println!
    // the struct with {}
    // NB: you don't need to understand this so far
    impl Display for UnitStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "this is our unit structure")
        }
    }

    let us = UnitStruct;

    println!("{us}");
}

#[test]
fn struct_field_init_shorthand() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let width = 5;
    let height = 10;

    // notice that we don't need to provide
    // the field names because our variables
    // already have the same names as the fields.
    let r = Rectangle { width, height };

    println!("{r:?}");
}

#[test]
fn struct_update_syntax() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let r1 = Rectangle {
        width: 5,
        height: 10,
    };
    println!("r1: {r1:?}");

    // this notation is quite heavy
    let r2 = Rectangle {
        width: r1.width,
        height: r1.height,
    };
    println!("r2: {r2:?}");

    // this is equivalent to
    let r3 = Rectangle { ..r1 };

    println!("r3: {r3:?}");

    // we can use partial update syntax
    // here width is different but height is the same
    let r4 = Rectangle { width: 10, ..r1 };

    println!("r4: {r4:?}");
}

/// # Struct Ownership in Rust (Simple)
///
/// **Structs own their data**:
/// - When you create a struct, it **takes ownership** of its fields
/// - When the struct is dropped, **all its fields are dropped too**
/// - If some fields are borrowed, the compiler needs to know that
///   every reference is valid until the structure is dropped. So
///   lifetime will have to be introduced but it is out of scope for
///   the moment.
#[test]
fn structure_and_ownership() {
    struct Person {
        name: String,
        age: u8,
    }

    let alice = Person {
        name: String::from("Alice"),
        age: 42,
    };

    // ownership rules apply the same on structures
    // if we move name out of alice, we cannot
    // use alice's name anymore.
    let n = alice.name;
    // remember u8 can be copied on the stack so
    // ownership doesn't apply to it
    let a = alice.age;
    println!("n={n} a={a}");
    println!("alice's age: {}", alice.age);

    // EXERCISE: uncomment below code to illustrate
    // println!("alice's name: {}", alice.name);

    // EXERCISE: propose a fix
}

/// In this exercise we show how we can define methods
/// for structures.
#[test]
fn structure_method() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // method defintion is done in an `impl` block
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let mut r = Rectangle {
        width: 5,
        height: 3,
    };

    // we call the method with a `.`
    println!("r area={}", r.area());

    // we can have several impl blocks for
    // the same struct. It might be useful
    // in some cases to structure the code.
    impl Rectangle {
        // if we want to modify the structure
        // we must declare it as muttable with `mut`
        fn modify_width(&mut self, width: u32) {
            self.width = width
        }
    }

    // the new width will be 10
    r.modify_width(10);

    println!("new r area={}", r.area());

    impl Rectangle {
        // one can define function taking OWNERSHIP
        // of the structure. This means that after
        // going into this method the structure
        // will not be usable anymore as the
        // function became the owner.
        fn into_tuple(self) -> (u32, u32) {
            (self.width, self.height)
        }
    }

    let rect_tuple = r.into_tuple();
    println!("rect_tuple={rect_tuple:?}");

    // EXERCISE: try to uncomment
    // println!("r area={}", r.area());

    impl Rectangle {
        // one can define structure creator
        // this notation is equivalent to:
        // fn from_tuple(t: (u32, u32)) -> Rectangle
        fn from_tuple(t: (u32, u32)) -> Self {
            Self {
                width: t.0,
                height: t.1,
            }
        }
    }

    let r2 = Rectangle::from_tuple(rect_tuple);
    println!("r2 area={}", r2.area());
    // NB: we can still use rect_tuple because both the fields
    // implement Copy thus from_tuple doesn't take OWNERSHIP
    // over it. It is using a copy instead.
    println!("rect_tuple={rect_tuple:?}");
}

/// EXERCISE: following what we have seen above
/// implement a Circle structure with the following
/// methods:
/// - a constructor taking a radius parameter
/// - a method to compute its area
/// - a modifier for the radius
/// - a method that returns true if a circle can hold another one
#[test]
fn circle_exercise() {}

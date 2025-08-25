# Rust Training

## About
This repository contains materials for a 2-3 day Rust training course.

The training is designed **not** to replace *The Rust Programming Language* book (commonly called "the Rust book"), but rather to provide you with solid foundational knowledge. After completing this training, you'll find the Rust book easier to understand because you'll already be familiar with the core concepts.

## Target Audience
This training is intended for developers who already have proficiency in at least one other programming language.

## Recommended Learning Path

Learning Rust effectively requires time and practice. While this training will accelerate your learning process, please note that it alone won't make you proficient in Rust.

We recommend the following learning path:

1. Complete this training course
2. Utilize official rust-lang resources:
   - Read *the entire* [Rust book](https://doc.rust-lang.org/stable/book/)
   - Simultaneously, practice with [Rustlings exercises](https://github.com/rust-lang/rustlings)
3. Apply your knowledge by working on a small Rust project of your choice


## Requirements

There is **no OS requirement** for this training so all examples should run on any OS/distribution combo.

1. Install Rust by downloading the `rustup` package of your Linux distribution or by following instructions at: https://www.rust-lang.org/learn/get-started  
1. Install **VSCode** or **VSCodium** 
1. Install **RustAnalyzer plugin** (official rust-lang plugin): https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
1. Clone this repository with `git clone` (so that you'll be able to refresh it easily to get all content at workshop time)
1. Open it in **VSCode**
    * Optional: `right-click` on the `README.md` file in the left panel and click on *Open Preview*
1. Navigate to [test](exercises/hello/src/main.rs)
    1. You are supposed to see a Rust `main` function with a little *play button* on top
    1. Click that *play button*
    1. You should see a bottom panel opening with the message to print
1. If the above step completed successfully, everything is ready

## Training Roadmap

1. [data-types](./exercises/data-types/src/lib.rs)
1. [functions](./exercises/functions/src/lib.rs)
1. [control-flow](./exercises/control-flow/src/lib.rs)
1. [ownership](./exercises/ownership/src/lib.rs)
1. [structures](./exercises/structures/src/lib.rs)
1. [enums](./exercises/enums/src/lib.rs)
1. [options](./exercises/options/src/lib.rs)
1. [result](./exercises/result/src/lib.rs)
1. [generics](./exercises/generics/src/lib.rs)
1. [traits](./exercises/traits/src/lib.rs)
1. [project](./exercises/project/src/lib.rs)

## Memos

Here are some memos you can use in order to help you to quickly
remember what we have seen in this training

1. [Rust guarantees](./memos/rust-guarantees.md)
1. [Primitive types](./memos/primitive-types.md)
1. [Control flow](./memos/control-flow.md)
1. [Destructuring](./memos/destructuring.md)
1. [Ownership](./memos/ownership.md)
1. [Items privacy](./memos/items-privacy.md)
1. [Struct enum](./memos/struct-enum.md)
1. [Option & Result enums](./memos/option-result.md)
1. [Generics](./memos/generics.md)
1. [Traits](./memos/traits.md)
1. [Lifetimes](./memos/lifetimes.md)

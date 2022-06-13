## Variables

- By default rust variables is immutable, but we can make the as a mutable
- Wecan make the variable mutable by adding `mut` in front of the variable name
- When we are using large data structure, mutable will be gonna useful as it will make our program run faster. and the opposite

## Constant, Variables, Shadow

- Constant will be always immutable. There’s no way to make them mutable
- Constant convention is in UPPER_CASE. We can use lower case but the compiler will show a warning. it still work ( [Constant Evaluation - The Rust Reference (rust-lang.org)](https://doc.rust-lang.org/reference/const_eval.html) )
- Constant is global scope variable
- We can `shadow` the variable by defining the same variable name
- Shadowing and `mut` are different things. By using `shadow` we can change the variable data type. But we can’t do that while using `mut`

## Data Types

- *Signed* and *unsigned* refer to whether it’s possible for the number to be negative
- Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive and Unsigned variants can store numbers from 0 to 2n - 1
- Integer types in Rust

![Untitled](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/e19e3b8e-924b-4dcf-bd37-b999866f4260/Untitled.png)

- Integer Literal in Rust

![Untitled](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/d661a715-d4a3-41eb-9e19-75c8655376be/Untitled.png)

- In Rust, Integer types is default at `u32`
- At development or debug mode, if the value is out of integer range, it will cause panicking in the compiler. But in `--release` mode it will `Integer Overflow`. In the case of a `u8`
, the value 256 becomes 0, the value 257 becomes 1, and so on.
- Rust has floating data type called `fnumber` :  `f32` and so on.
- Rust floating number by default is `f64` the smallest value is `f32`
- We specify `char` literal with single quote, and the opossite we specify string literal with double quotes.
- Rust’s `char`type is four bytes in size and represents a Unicode Scalar Value

### Array in Rust

- Element in array should have the same type
- Array in rust is fixed, if we want something similar but more flexible we can use concept called  `vector`
- A vector is a similar collection type provided by the standard library that *is*
 allowed to grow or shrink in size.
`uX` stands for an unsigned integer with X bits.

`f64` is a 64-bit float. To convert a `u8` to a `f64`, you can do:

```rust
let x: u8 = 5;
f64::from(x)
```

`let` allows variable declarations similar to JavaScript.

As you can see in `src/lib.rs`, no return statement is required because the value of the `if..else` block group is the value of the last expression in the respective block. that was chosen. Since this block group is the last expression in the function, the return value of the function is too. To convert an `f64` to a `u32`, you can do:

```rust
let x: f64 = 5.0;
x as u32
```

Semicolons are optional in Rust, but they modify the behavior of an expression. Adding a semicolon to the end of an expression suppresses the value of the expression.

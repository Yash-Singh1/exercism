Enums are allow you to define a type that has a limited set of possible options. This can be useful for `match` statements (`switch...case` of Rust) or options that want to limit what they take instead of validating a string.

Enums can be declared with the enum keyword:

```rust
enum Color {
    Red,
    Green,
    Blue,
}
```

I can now define a function that takes a color and returns a string:

```rust
pub fn draw(color: Color) -> String {
    if (color == Color::Red) {
      "red"
    } else if (color == Color::Green) {
      "green"
    } else if (color == Color::Blue) {
      "blue"
    }
    "unknown"
}
```

Enums can also be public:

```rust
pub enum Digits {
  Zero,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
}
```

The `unimplemented!()` macro allows you to specify that a function is not yet implemented without a compiler error.

The `Chars` type gives an iterator to a string's characters. The `rev` method on it reverses the iterator. The `collect` method gives you back teh string. All put together it looks like:

```rust
input.chars().rev().collect()
```

However, if you want to add support for more code points with multiple characters in one, in this example, we use the `unicode_reverse` crate.

To install the crate, you need to add a new line underneath the dependencies field in `Cargo.toml` with the following:

```toml
unicode-reverse = "1.0.8"
```

And then, you can run `cargo build` to install the crate and run `cargo test --features grapheme`, because the tests with multiple code points in one character are underneath that feature gate.

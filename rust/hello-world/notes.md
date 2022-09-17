The `fn` keyword represents a function. The `main` function is a special function because it runs on the entry of the program. The `pub` keyword means that it's exported. Here is an example program that returns the string it receives:

```rust
fn say_my_name(name: &str) -> &str {
  name
}
```

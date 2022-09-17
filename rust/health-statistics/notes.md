A struct is similar to an interface in TypeScript, it defines the type and shape of a data structure. To populate a struct with methods to interact with the data stored inside it, you use `impl`. Here is an example for food:

```rust
struct Food {
    name: String,
    calories: u32,
}

impl Food {
  pub fn new(name: String, calories: u32) -> Food {
    Food {
      name,
      calories
    }
  }
  pub fn isHealthy(&self) -> bool {
    self.calories < 100
  }
}
```

If I wanted to make the `Food` struct avaliable from this crate, I would use `pub struct` instead of `struct`. Public methods use `pub fn` underneath the `impl` block. The `new` method is a special method that is a constructor. In this example, we used it to create a new instance of the `Food` `struct` with a set name and calories.

You are allowed to leave off the curly braces to make a ZST (zero-sized type):

```rust
struct ZST;
```

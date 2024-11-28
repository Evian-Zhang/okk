# okk

This crate is yet another approach to replacing `Ok(())` across the code, following the idea discussed in [this IRLO thread](https://internals.rust-lang.org/t/killing-ok-with-ok/21915).

## Use case

This crate provides a function `ok()` and a macro `ok!()`, both can be used to replace `Ok(())`, and so you can write only one pair of parentheses:

```rust
use okk::*;

struct MyError;

fn try_foo() -> Result<(), MyError> {
    // ...
    ok()
}

fn try_bar() -> Result<(), MyError> {
    // ...
    ok!()
}
```

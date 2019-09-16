# ward

[![Crates.io](https://img.shields.io/crates/v/ward)](https://crates.io/crates/ward)
[![Crates.io](https://img.shields.io/crates/l/ward)](https://crates.io/crates/ward)
[![Crates.io](https://img.shields.io/crates/d/ward)](https://crates.io/crates/ward)
[![Docs.io](https://docs.rs/ward/badge.svg)](https://docs.rs/ward)
![no_std](https://img.shields.io/badge/no__std-yes-brightgreen)

This crate exports two macros, which are intended to replicate the functionality of Swift's
guard expression with `Option<T>`.

The `guard!` macro was created to emulate the `guard let` statement in Swift. This macro is only
really useful for moving values out of `Option<T>`s into variables.
The `ward!` macro, on the other hand, doesn't force the creation of a variable, it only returns
the value that the `guard!` variable would place into a variable. As such, it's a more flexible
version of the `guard!` macro; and probably also somewhat more Rustic.

## Examples

```rust
let sut = Some("test");

// This creates the variable res, which from an Option<T> will return a T if it is Some(T), and will
// otherwise return early from the function.
guard!(let res = sut);
assert_eq!(res, "test");
```

The `ward!` macro, by comparison, just returns the value, without forcing you to make a variable
from it (although we still do in this example):

```rust
let sut = Some("test");
let res = ward!(sut);
assert_eq!(res, "test");
```

Both macros also support an `else` branch, which will run if the `Option<T>` is `None`:

```rust
let sut = None;
guard!(let _res = sut, else {
    println!("This will be called!");

    // Because sut is None, the else branch will be run. When the else branch is invoked, guard!
    // no longer automatically returns early for you, so you must do so yourself if you want it.
    return;
});
unreachable!();
```

Both macros also support an alternative "early return statement", which will let you e.g.
`break` within loops:

```rust
// Not that you couldn't (and probably should) do this case with `while let Some(res) = sut`...
let mut sut = Some(0);
loop {
    let res = ward!(sut, break);
    sut = if res < 5 {
        Some(res + 1)
    } else {
        None
    }
}
assert_eq!(sut, None);
```

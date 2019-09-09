# ward

This crate exports two macros, which are intended to replicate the functionality of Swift's guard
expression with `Option<T>` usage. They both do similar things, but the `ward!` macro technically
has more use cases than the `guard!` macro, because it returns a value instead of creating a
variable.

## Examples

```rust
let sut = Some("test");

// This creates the variable res, which from an Option<T> will return a T if it is Some(T), and will
// otherwise return early from the function.
guard!(let res = sut);
assert_eq!("test", res);
```

The `ward!` macro, by comparison, just returns the value, without forcing you to make a variable
from it (although we still do in this example):

```rust
let sut = Some("test");
let res = ward!(sut);
assert_eq!("test", res);
```

Both macros also support an `else` branch, which will run before the method returns early:

```rust
let sut = None;
guard!(let _res = sut, else {
    println!("This will be called!");
});
unreachable!();
```

Both macros also support an alternative "early return statement", which will let you e.g. `break` in loops:

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

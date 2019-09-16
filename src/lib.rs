#![no_std]

//! This crate exports two macros, which are intended to replicate the functionality of Swift's
//! guard expression with `Option<T>`.
//!
//! The `guard!` macro was created to emulate the `guard let` statement in Swift. This macro is only
//! really useful for moving values out of `Option<T>`s into variables.
//! The `ward!` macro, on the other hand, doesn't force the creation of a variable, it only returns
//! the value that the `guard!` variable would place into a variable. As such, it's a more flexible
//! version of the `guard!` macro; and probably also somewhat more Rustic.

/// Returns the contents of a `Option<T>`'s `Some(T)`, otherwise it returns early
/// from the function. Can alternatively have an `else` branch, or an alternative "early return"
/// statement, like `break` or `continue` for loops, e.g.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // res is set to sut's contents when sut is Some(...)
/// let sut = Some("test");
/// let res = ward!(sut);
/// assert_eq!(res, "test");
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // When sut is None, the ward! returns early
/// let sut = None;
/// let res = ward!(sut);
/// unreachable!();
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // Because sut is None, the else branch will be run. When the else branch is invoked, ward!
/// // no longer automatically returns early for you, so you must do so yourself if you want it.
/// let sut = None;
/// let _res = ward!(sut, else {
///     println!("This code will run!");
///     return;
/// });
/// unreachable!();
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // The lack of automatic early return for the else branch allows for alternative return values
/// // to be provided.
/// let sut = None;
/// let res = ward!(sut, else { 42 });
/// assert_eq!(res, 42);
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // You can use ward! with a different "early return" statement, such as break for loops
/// let mut sut = Some(0);
/// loop {
///     let res = ward!(sut, break);
///     sut = if res < 5 {
///         Some(res + 1)
///     } else {
///         None
///     }
/// }
/// assert_eq!(sut, None);
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // Unlike guard!, ward! can be used as an expression
/// let sut = Some("test");
/// print(ward!(sut));
///
/// fn print(text: &str) {
///     println!("{}", text);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! ward {
    ($o:expr) => ($crate::ward!($o, else { return; }));
    ($o:expr, else $body:block) => { if let Some(x) = $o { x } else { $body }; };
    ($o:expr, $early:stmt) => ($crate::ward!($o, else { $early }));
}

/// Creates a variable with the contents of a `Option<T>`'s `Some(T)`, otherwise it returns early
/// from the function. Can alternatively have an `else` branch, or an alternative "early return"
/// statement, like `break` or `continue` for loops, e.g.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // res is set to sut's contents when sut is Some(...)
/// let sut = Some("test");
/// guard!(let res = sut);
/// assert_eq!(res, "test");
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // When sut is None, the guard! returns early
/// let sut = None;
/// guard!(let mut res = sut);
/// unreachable!();
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // Because sut is None, the else branch will be run. When the else branch is invoked, guard!
/// // no longer automatically returns early for you, so you must do so yourself if you want it.
/// let sut = None;
/// guard!(let _res = sut, else {
///     println!("This code will run!");
///     return;
/// });
/// unreachable!();
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // The lack of automatic early return for the else branch allows for alternative return values
/// // to be provided.
/// let sut = None;
/// guard!(let res = sut, else { 42 });
/// assert_eq!(res, 42);
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // You can use guard! with a different "early return" statement, such as break for loops
/// let mut sut = Some(0);
/// loop {
///     guard!(let res = sut, break);
///     sut = if res < 5 {
///         Some(res + 1)
///     } else {
///         None
///     }
/// }
/// assert_eq!(sut, None);
/// # }
/// ```
#[macro_export]
macro_rules! guard {
    (let $result:ident = $o:expr) => ($crate::guard!(let $result = $o, else { return; }));
    (let $result:ident = $o:expr, else $body:block) => { let $result = ward!($o, else $body); };
    (let $result:ident = $o:expr, $early:stmt) => ($crate::guard!(let $result = $o, else { $early }));
    (let mut $result:ident = $o:expr) => ($crate::guard!(let mut $result = $o, else { return; }));
    (let mut $result:ident = $o:expr, else $body:block) => { let mut $result = ward!($o, else $body); };
    (let mut $result:ident = $o:expr, $early:stmt) => ($crate::guard!(let $result = $o, else { $early }));
}

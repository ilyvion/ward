//! This crate exports two macros, which are intended to replicate the functionality of Swift's
//! guard expression with `Option<T>`. They both do similar things, but the `ward!` macro
//! technically has more use cases than the `guard!` macro, because it returns a value instead of
//! creating a variable.

/// Returns the contents of a `Option<T>`'s `Some(T)`, otherwise it returns early from the function.
/// Can also be used with other "early returns" like `break` or `continue`.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // res is set to sut's contents
/// let sut = Some("test");
/// let res = ward!(sut);
/// assert_eq!("test", res);
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // Because sut is None, the ward! returns early
/// let sut = None;
/// let res = ward!(sut);
/// unreachable!();
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // Because sut is None, the ward!'s else branch will be run before it returns early
/// let sut = None;
/// let _res = ward!(sut, else {
///     # assert!(true);
///     println!("This code will run!");
/// });
/// unreachable!();
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // You can use ward! with a different "early return" statement, such as `break` for loops
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
    ($o:expr) => ($crate::ward!($o, else {}));
    ($o:expr, else $body:block) => ($crate::ward!($o, else $body, return));
    ($o:expr, else $body:block, $early:stmt) => {
        if let Some(x) = $o { x } else { $body; $early; };
    };
    ($o:expr, $early:stmt) => ($crate::ward!($o, else {}, $early));
}

/// Creates a variable with the contents of a `Option<T>`'s `Some(T)`, otherwise it returns early
/// from the function. Can also be used with other "early returns" like `break` or `continue`.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // res is set to sut's contents
/// let sut = Some("test");
/// guard!(let res = sut);
/// assert_eq!("test", res);
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // Because sut is None, the guard! returns early
/// let sut = None;
/// guard!(let mut res = sut);
/// unreachable!();
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // Because sut is None, the guard!'s else branch will be run before it returns early
/// let sut = None;
/// guard!(let _res = sut, else {
///     # assert!(true);
///     println!("This code will run!");
/// });
/// unreachable!();
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate ward;
/// #
/// # fn main() {
/// // You can use guard! with a different "early return" statement, such as `break` for loops
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
    (let $result:ident = $o:expr) => ($crate::guard!(let $result = $o, else {}));
    (let $result:ident = $o:expr, else $body:block) => {
        let $result = ward!($o, else $body);
    };
    (let $result:ident = $o:expr, else $body:block, $early:stmt) => {
        let $result = ward!($o, else $body, $early);
    };
    (let $result:ident = $o:expr, $early:stmt) => ($crate::guard!(let $result = $o, else {}, $early));
    (let mut $result:ident = $o:expr) =>  ($crate::guard!(let mut $result = $o, else {}));
    (let mut $result:ident = $o:expr, else $body:block) => {
        let mut $result = ward!($o, else $body);
    };
    (let mut $result:ident = $o:expr, else $body:block, $early:stmt) => {
        let mut $result = ward!($o, else $body, $early);
    };
    (let mut $result:ident = $o:expr, $early:stmt) => ($crate::guard!(let $result = $o, else {}, $early));
}

#[cfg(test)]
mod tests {
    #[test]
    fn guard_extracts_value_from_some() {
        let sut = Some("test");
        guard!(let res = sut);
        assert_eq!("test", res);
    }

    #[test]
    fn guard_returns_early_on_none() {
        let sut = None;
        guard!(let mut _res = sut);
        unreachable!();
    }

    #[test]
    fn guard_runs_else_on_none() {
        let sut = None;
        guard!(let _res = sut, else {
            assert!(true);
        });
        unreachable!();
    }
}

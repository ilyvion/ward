//! This crate exports two macros, which are intended to replicate the functionality of Swift's
//! guard expression with `Option<T>`. They both do similar things, but the `ward!` macro
//! technically has more use cases than the `guard!` macro, because it returns a value instead of
//! creating a variable.

/// Returns the contents of a `Option<T>`'s `Some(T)`, otherwise it returns early from the function.
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
/// // Unlike guard!, ward! can be used as an expression
/// let sut = Some("test");
/// print(ward!(sut));
/// # }
///
/// fn print(text: &str) {
///     println!("{}", text);
/// }
/// ```
#[macro_export]
macro_rules! ward {
    ($o:expr) => {
        if let Some(x) = $o { x } else { return; };
    };
    ($o:expr, else $body:block) => {
        if let Some(x) = $o { x } else { $body; return; };
    };
}

/// Creates a variable with the contents of a `Option<T>`'s `Some(T)`, otherwise it returns early
/// from the function.
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
///
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
///
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
#[macro_export]
macro_rules! guard {
    (let $result:ident = $o:expr) => {
        let $result = ward!($o);
    };
    (let $result:ident = $o:expr, else $body:block) => {
        let $result = ward!($o, else $body);
    };
    (let mut $result:ident = $o:expr) => {
        let mut $result = ward!($o);
    };
    (let mut $result:ident = $o:expr, else $body:block) => {
        let mut $result = ward!($o, else $body);
    };
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

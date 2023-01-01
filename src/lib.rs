//! A small toolbelt of macros that implement the [`Option::ok_or_else`] and
//! [`Result::map_err`] with macros instead of functions taking callbacks.
//!
//! This reduces the boilerplate when you can't use the abovementioned functions
//! because the error condition handling logic you need requires you that
//! you move certain values into the closures which you can't affort.
//!
//! People would normally work around it by `.clone()`-ing the value
//! to be passed into the closure, but that's bad because:
//!
//! - you need to `.clone()` the item, and that's not always possible;
//! - the `.clone()` is not really necessary, you can rewrite the code with
//!   a manual `match` that would not require and ownership transfers.
//!
//! But writing a match is vebose, and who wants that?!
//! This is where this crate comes to help.
//!
//! For best look and feel, combine with
//! [`postfix-macros`](https://docs.rs/postfix-macros) crate.

/// An analog to [`Result::map_err`] but without a closure.
///
/// ```
/// use macro_map::map_err;
///
/// let myresult: Result<&str, &str> = Err("hello");
///
/// let mapped = map_err!(myresult, |myerr| 123);
///
/// assert_eq!(mapped, Err(123));
/// ```
///
/// Or with [`postfix-macros`](https://docs.rs/postfix-macros):
///
/// ```
/// use macro_map::map_err;
/// use postfix_macros::postfix_macros;
///
/// let myresult: Result<&str, &str> = Err("hello");
///
/// postfix_macros! {
///   let mapped = myresult.map_err!(|myerr| 123);
/// }
///
/// assert_eq!(mapped, Err(123));
/// ```
#[macro_export]
macro_rules! map_err {
    ($result:expr, |$err:pat_param| $closure:expr) => {
        match $result {
            Ok(val) => Ok(val),
            Err($err) => Err($closure),
        }
    };
}

/// An analog to [`Option::ok_or_else`] but without a closure.
///
/// ```
/// use macro_map::ok_or_else;
///
/// let myoption: Option<&str> = None;
///
/// let mapped = ok_or_else!(myoption, || 123);
///
/// assert_eq!(mapped, Err(123));
/// ```
///
/// Or with [`postfix-macros`](https://docs.rs/postfix-macros):
///
/// ```
/// use macro_map::ok_or_else;
/// use postfix_macros::postfix_macros;
///
/// let myoption: Option<&str> = None;
///
/// postfix_macros! {
///   let mapped = ok_or_else!(myoption, || 123);
/// }
///
/// assert_eq!(mapped, Err(123));
/// ```
#[macro_export]
macro_rules! ok_or_else {
    ($result:expr, || $closure:expr) => {
        match $result {
            Some(val) => Ok(val),
            None => Err($closure),
        }
    };
}

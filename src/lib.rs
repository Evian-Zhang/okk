#![no_std]
#![doc = include_str!("../README.md")]

/// This macro will be expanded to `Ok(())`:
///
/// ```rust
/// # use okk::*;
/// # struct Error;
/// fn try_foo() -> Result<(), Error> {
///     ok!()
/// }
/// ```
#[macro_export]
macro_rules! ok {
    () => {
        Ok(())
    };
}

/// This function will always return `Ok(())`:
///
/// ```rust
/// # use okk::*;
/// # struct Error;
/// fn try_foo() -> Result<(), Error> {
///     ok()
/// }
/// ```
pub fn ok<E>() -> core::result::Result<(), E> {
    Ok(())
}

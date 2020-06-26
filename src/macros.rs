/*!
# Macros of Neunite

## Plan

- ~~Plan.~~
- Design.
- MVP.

## Code

```rust
//
*/

#[macro_export]
macro_rules! fail {
    () => ($crate::Error::new($crate::error::Exception));
    ($msg:literal $(,)?) => (fail!($crate::error::Exception, $msg));
    ($err:expr) => ($crate::Error::new($err));
    ($err:expr, $($arg:tt)*) => ($crate::Error::new($err).context(format!($($arg)*)));
}

#[macro_export]
macro_rules! scalar {
    ( $s:expr, $t:ty ) => {
        Scalar::<$t>::scalar($s)
    };
    ( $s:expr, is, $t:ty ) => {
        Scalar::<$t>::is_scalar($s)
    };
    ( $s:expr, to, $t:ty ) => {
        Scalar::<$t>::to_scalar($s)
    };
    ( $s:expr, as, $t:ty ) => {
        Scalar::<$t>::as_scalar($s)
    };
}

/*
```
*/

#![cfg_attr(not(test), no_std)]
//! `unreachable_static!()` is a compile-time variant of the `std::unreachable!()`
//! macro. The optimizer or linker must be able prove that the statement is
//! unreachable otherwise the program will fail to compile.

/// Compile-time variant of `std::unreachable!()`
///
/// Fail to compile if the compiler can't prove the control flow is unreachable
/// via dead code elimination, function inlining and optimizations, or LTO.
///
/// ```no_run
/// # // Can't doctest this due to https://github.com/rust-lang/cargo/issues/4251
/// if [0].is_empty() {
///   // Rust's type system can't assert this, but the compiler can...
///   reachability::unreachable_static!()
/// }
/// ```
///
/// ## Opt-in
///
/// This macro behaves like `std::unreachable!()` unless explicitly opted into
/// by a binary crate with the `static` feature, which should only be done when
/// building release binaries (LTO is recommended but not required). Libraries
/// depending on this crate **should not** enable this feature.
///
/// `unreachable_static!(!)` can be used to bypass this and always statically
/// assert.
///
/// ## Consistency
///
/// Note that eliminating unreachable code relies heavily on the compiler,
/// optimizer, and linker being able to inline functions and determine that it
/// cannot be executed. Changes in compiler versions, optimization levels, or
/// LTO settings may cause code that previously worked to fail with a linker
/// error, so be careful in how you use this!
#[macro_export]
macro_rules! unreachable_static {
    (!) => {
        $crate::unreachable_static! { !:"" }
    };
    (!: $msg:expr) => {
        {
            extern {
                // TODO include a message here in some way?
                #[link_name = "___unreachable_static___"]
                fn unreachable_static() -> !;
            }
            unsafe { unreachable_static(); }
        }
    };
    ($($tt:tt)*) => {
        $crate::internal_unreachable_static! { $($tt)* }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(any(not(feature = "static"), debug_assertions))]
macro_rules! internal_unreachable_static {
    ($($tt:tt)*) => {
        $crate::_core::unreachable! { $($tt)* }
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(all(feature = "static", not(debug_assertions)))]
macro_rules! internal_unreachable_static {
    ($($tt:tt)*) => {
        $crate::unreachable_static! { ! };
    };
}

/// Unsafe unreachable that panics in debug builds.
///
/// This is equivalent to `std::hint::unreachable_unchecked` in release builds.
///
/// Obeys the `debug-assertions` configuration option and uses
/// `std::hint::unreachable_unchecked` in release builds. Reaching this panic
/// means your code has *undefined behaviour* in release builds and must be
/// fixed.
#[macro_export]
macro_rules! unreachable_unchecked {
    (!) => {
        $crate::_core::hint::unreachable_unchecked()
    };
    ($($tt:tt)*) => {
        {
            #[cfg(debug_assertions)]
            {
                #[inline(always)]
                unsafe fn unreachable_() -> ! {
                    $crate::_core::unreachable!($($tt)*)
                }
                unreachable_()
            }
            #[cfg(not(debug_assertions))]
            $crate::unreachable_unchecked!(!)
        }
    };
}

/* TODO make these optional features and use a proc macro?

#[macro_export]
macro_rules! checked_ops {
    // TODO this
    ($expr:expr) => {
        $expr
    };
}

#[macro_export]
macro_rules! unchecked_ops {
    // TODO this
    ($expr:expr) => {
        $expr
    };
}*/

#[doc(hidden)]
pub use core as _core;

pub trait OptionExt {
    type Ok;

    fn unwrap_static(self) -> Self::Ok;
    unsafe fn unwrap_unchecked(self) -> Self::Ok;
}

pub trait ResultExt {
    type Err;

    fn unwrap_err_static(self) -> Self::Err;
    unsafe fn unwrap_err_unchecked(self) -> Self::Err;
}

impl<T> OptionExt for Option<T> {
    type Ok = T;

    #[inline(always)]
    fn unwrap_static(self) -> T {
        match self {
            None => unreachable_static!(),
            Some(v) => v,
        }
    }

    #[inline(always)]
    unsafe fn unwrap_unchecked(self) -> T {
        match self {
            None => unreachable_unchecked!(),
            Some(v) => v,
        }
    }
}

impl<T, E> OptionExt for Result<T, E> {
    type Ok = T;

    #[inline(always)]
    fn unwrap_static(self) -> T {
        match self {
            Err(_) => unreachable_static!(),
            Ok(v) => v,
        }
    }

    #[inline(always)]
    unsafe fn unwrap_unchecked(self) -> T {
        match self {
            Err(_) => unreachable_unchecked!(),
            Ok(v) => v,
        }
    }
}

impl<T, E> ResultExt for Result<T, E> {
    type Err = E;

    #[inline(always)]
    fn unwrap_err_static(self) -> E {
        match self {
            Ok(_) => unreachable_static!(),
            Err(v) => v,
        }
    }

    #[inline(always)]
    unsafe fn unwrap_err_unchecked(self) -> E {
        match self {
            Ok(_) => unreachable_unchecked!(),
            Err(v) => v,
        }
    }
}

#[doc(hidden)]
#[cfg(any(test, feature = "unstable-internal-test"))]
pub mod tests {
    #[test]
    #[allow(unreachable_code)]
    fn dead_code() {
        return;
        unreachable_static!(!);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn unchecked_assert() {
        unsafe {
            unreachable_unchecked!("intentional");
        }
    }

    pub fn grey_box(v: i32) -> i32 { v }
}

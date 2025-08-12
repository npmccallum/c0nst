//! # c0nst - Conditional Const Syntax Transformation
//!
//! This crate enables the sharing of code between const traits on nightly and
//! non-const traits on stable. It is small and lightweight. It works on a very
//! simple principle: everywhere you want to use const on nightly but not on
//! stable, just use the keyword `c0nst` instead of `const`. That's it!
//!
//! When the new `const` syntax is stabilized, you can convert the `c0nst`
//! keyword to `const` (`s/c0nst/const/g`) and remove the use of this crate.
//!
//! ## Example
//!
//! This is the canonical example of using the `c0nst` crate, [derived from the
//! RFC][rfc]. This ensures we can do everything designed by the RFC.
//!
//! [rfc]: https://github.com/rust-lang/rust/issues/143874
//!
//! ```rust
//! #![cfg_attr(feature = "nightly", feature(const_trait_impl))]
//!
//! c0nst::c0nst! {
//!     pub c0nst trait Default {
//!         fn default() -> Self;
//!     }
//!
//!     impl c0nst Default for () {
//!         fn default() -> Self {}
//!     }
//!
//!     pub struct Thing<T>(pub T);
//!
//!     impl<T: [c0nst] Default> c0nst Default for Thing<T> {
//!         fn default() -> Self {
//!             Self(T::default())
//!         }
//!     }
//!
//!     pub c0nst fn default<T: [c0nst] Default>() -> T {
//!         T::default()
//!     }
//!
//!     pub fn compile_time_default<T: c0nst Default>() -> T {
//!         c0nst { T::default() }
//!     }
//! }
//! ```

mod convert;
mod tests;

use proc_macro::TokenStream;

use crate::convert::Convert;

/// Target compilation environment
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
enum Target {
    /// Stable Rust - remove const syntax
    #[cfg_attr(not(feature = "nightly"), default)]
    Stable,

    /// Nightly Rust - use modern const syntax
    #[cfg_attr(feature = "nightly", default)]
    Nightly,
}

/// Emits conditionally const code.
///
/// On `feature = "nightly"`, it will convert `c0nst` to `const`.
/// Otherwise, it will remove `c0nst` and `[c0nst]` syntax.
///
/// Nothing more. Nothing less.
#[proc_macro]
pub fn c0nst(input: TokenStream) -> TokenStream {
    proc_macro2::TokenStream::from(input)
        .convert(Target::default())
        .into()
}

//! # c0nst - Conditional Const Syntax Transformation
//!
//! This crate provides macros that enable the sharing of code between const
//! traits on nightly and non-const traits on stable. It works like this.
//!
//! 1. You annotate your code with the desired const behavior using the syntax
//!    provided by this crate.
//!
//! 2. When you compile your code, the crate will automatically transform it
//!    based on your compilation target (stable or nightly). When the `nightly`,
//!    feature is enabled, the crate will use the new const trait syntax. When
//!    the `nightly` feature is disabled, it will fall back to the non-const
//!    trait syntax.
//!
//! Therefore, this enables library authors to provide both const and non-const
//! versions of their traits, depending on the compilation target. This is done
//! without:
//!
//! * additional constraints on library users -- they only ever see Rust syntax
//! * code duplication -- no need for parallel const and non-const traits
//!
//! Besides the main purpose of this crate detailed above, it also isolates you,
//! the library author, and your users from the churn of nightly syntax changes.
//! Most nightly changes can be simply absorbed by this crate.
//!
//! ## Macros
//!
//! - `#[c0nst]` - Transforms item while marking it as const.
//! - `#[m0rph]` - Transforms item without marking it as const.
//!
//! ## Example
//!
//! ```rust
//! #![cfg_attr(feature = "nightly", feature(const_trait_impl))]
//!
//! use c0nst::{c0nst, m0rph, bl0ck};
//!
//! // `const trait Default { ... }` => `#[c0nst] trait Default { ... }`
//! #[c0nst]
//! pub trait Default {
//!     fn default() -> Self;
//! }
//!
//! // `impl const Default for () { ... }` => `#[c0nst] impl Default for () { ... }`
//! #[c0nst]
//! impl Default for () {
//!     fn default() -> Self {}
//! }
//!
//! pub struct Thing<T>(pub T);
//!
//! // `impl<...> const Default for ...` => `#[c0nst] impl<...> Default for ...`
//! // `T: [const] Default` => `T: ?c0nst<Default>`
//! #[c0nst]
//! impl<T: ?c0nst<Default>> Default for Thing<T> {
//!     fn default() -> Self {
//!         Self(T::default())
//!     }
//! }
//!
//! // `const fn default<...>() ...` => `#[c0nst] fn default<...>() ...`
//! // `T: [const] Default` => `T: ?c0nst<Default>`
//! #[c0nst]
//! pub fn default<T: ?c0nst<Default>>() -> T {
//!     T::default()
//! }
//!
//! // `T: const Default` => `T: c0nst<Default>`
//! // `const { ... }` => `bl0ck! { ... }`
//! #[m0rph]
//! pub fn compile_time_default<T: c0nst<Default>>() -> T {
//!     bl0ck! { T::default() }
//! }
//! ```
//!
//! With `--features nightly`, this becomes native `const trait` syntax.
//! Without `--features nightly`, it falls back to regular (non-const) traits.

mod attrs;
mod tests;
mod xform;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};
use xform::{Annotation, Target, Transform};

/// Transforms an item without marking it as const.
///
/// With feature `nightly` enabled, transforms the item by resolving all
/// annotations to the nightly const trait syntax. With feature `nightly`
/// disabled, transforms the item to use regular (non-const) traits.
///
/// This is useful when you want to transform types that have inner markings
/// but are not themselves const. For example, you might have a function,
/// struct or module which contains const bounds, but is not, itself, const.
///
/// ## Supported Items
/// - traits, implementation blocks, functions (same as `#[c0nst]`)
/// - structs, enums, unions, type aliases, modules
///
/// ## Conditional Bounds
/// - `T: c0nst<Trait>` - Unconditionally const
/// - `T: ?c0nst<Trait>` - Conditionally const
#[proc_macro_attribute]
pub fn m0rph(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);
    match item.can_m0rph() {
        Err(err) => err.to_compile_error().into(),
        Ok(()) => item.transform(Target::default()).into(),
    }
}

/// Transforms an item while marking it as const.
///
/// With feature `nightly` enabled, transforms the item by resolving all
/// annotations to the nightly const trait syntax. With feature `nightly`
/// disabled, transforms the item to use regular (non-const) traits.
///
/// ## Supported Items
/// - traits, implementation blocks, functions
///
/// ## Conditional Bounds
/// - `T: c0nst<Trait>` - Unconditionally const (i.e. `const`)
/// - `T: ?c0nst<Trait>` - Conditionally const (i.e. `[const]`)
#[proc_macro_attribute]
pub fn c0nst(args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);
    match item.can_c0nst() {
        Err(err) => err.to_compile_error().into(),
        Ok(()) => m0rph(args, quote! { #[c0nst] #item }.into()),
    }
}

/// Transforms a block expression based on the target compilation environment.
///
/// With feature `nightly` enabled, emits `const { ... }` blocks.
/// With feature `nightly` disabled, emits regular `{ ... }` blocks.
///
/// ## Example
/// ```rust,ignore
/// use c0nst::bl0ck;
///
/// // This:
/// bl0ck! { T::default() }
///
/// // Becomes on nightly:
/// const { T::default() }
///
/// // Becomes on stable:
/// { T::default() }
/// ```
#[proc_macro]
pub fn bl0ck(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    #[cfg(feature = "nightly")]
    return quote! { const { #input } }.into();

    #[cfg(not(feature = "nightly"))]
    return quote! { { #input } }.into();
}

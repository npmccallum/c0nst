//! # c0nst - Conditional Const Syntax Transformation
//!
//! A procedural macro that enables the sharing of code between const traits
//! on nightly and non-const traits on stable. Your code will be annotated
//! to indicate const behavior. This code will then be transformed based on
//! your compilation target (stable or nightly).
//!
//!  * On nightly builds, the macro will emit const traits.
//!  * On stable builds, the the macro will emit non-const traits.
//!
//! ## Macros
//!
//! - `#[c0nst]` - Transforms item while marking it as const.
//! - `#[adapt]` - Transforms item without marking it as const.
//!
//! ## Example
//!
//! ```rust
//! #![cfg_attr(feature = "nightly", feature(const_trait_impl))]
//!
//! use c0nst::{c0nst, adapt};
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
//! // `T: ~const Default` => `T: ?c0nst<Default>`
//! #[c0nst]
//! impl<T: ?c0nst<Default>> Default for Thing<T> {
//!     fn default() -> Self {
//!         Self(T::default())
//!     }
//! }
//!
//! // `const fn default<...>() ...` => `#[c0nst] fn default<...>() ...`
//! // `T: ~const Default` => `T: ?c0nst<Default>`
//! #[c0nst]
//! pub fn default<T: ?c0nst<Default>>() -> T {
//!     T::default()
//! }
//!
//! // `T: const Default` => `T: c0nst<Default>`
//! // Always requires const implementation
//! #[adapt]
//! pub fn compile_time_default<T: c0nst<Default>>() -> T {
//!     T::default()
//! }
//! ```
//!
//! On nightly with `--features nightly`, this becomes native `const trait` syntax.
//! On stable, the `#[c0nst]` attributes are removed, generating regular (non-const) traits.

mod attrs;
mod tests;
mod xform;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};
use xform::{Adaptable, Target, Transform};

/// Transforms an item without marking it as const.
///
/// Transforms `#[c0nst]` annotations based on feature flags:
/// - **Nightly** (`nightly` feature): Native `const trait` syntax
/// - **Stable** (default): Non-const traits (removes `#[c0nst]` attributes)
///
/// ## Supported Items
/// - Traits, implementations, functions
/// - Structs, enums, unions, type aliases
/// - Modules (transforms all contained `#[c0nst]` items)
///
/// ## Conditional Bounds
/// - `T: c0nst<Trait>` - Unconditionally const
/// - `T: ?c0nst<Trait>` - Conditionally const
#[proc_macro_attribute]
pub fn adapt(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    // Check if the item can be adapted using the extension trait
    if item.can_adapt() {
        item.transform(Target::default()).into()
    } else {
        syn::Error::new_spanned(&item, "cannot adapt in this context")
            .to_compile_error()
            .into()
    }
}

/// Transforms an item while marking it as const.
///
/// Transforms `#[c0nst]` annotations based on feature flags:
/// - **Nightly** (`nightly` feature): Native `const trait` syntax
/// - **Stable** (default): Non-const traits (removes `#[c0nst]` attributes)
///
/// ## Supported Items
/// - Traits, implementations, functions
/// - Structs, enums, unions, type aliases
/// - Modules (transforms all contained `#[c0nst]` items)
///
/// ## Conditional Bounds
/// - `T: c0nst<Trait>` - Unconditionally const
/// - `T: ?c0nst<Trait>` - Conditionally const
#[proc_macro_attribute]
pub fn c0nst(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);

    // Create a new item with #[c0nst] attribute prepended
    let c0nst_attr = quote! { #[c0nst] };
    let item_with_attr = quote! {
        #c0nst_attr
        #item
    };

    // Parse the new item and transform it
    let new_item: Item = syn::parse2(item_with_attr).unwrap();

    if new_item.can_adapt() {
        new_item.transform(Target::default()).into()
    } else {
        syn::Error::new_spanned(&new_item, "cannot adapt in this context")
            .to_compile_error()
            .into()
    }
}

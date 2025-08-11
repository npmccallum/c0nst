mod attributes;
mod generics;
mod genparam;
mod implitem;
mod item;
mod itemenum;
mod itemfn;
mod itemimpl;
mod itemmod;
mod itemstruct;
mod itemtrait;
mod itemtype;
mod itemunion;
mod punctbound;
mod punctgenparam;
mod punctwhere;
mod signature;
mod traitbound;
mod traititem;
mod typebound;
mod typeparam;
mod vecimpl;
mod vectrait;
mod whereclause;
mod wherepred;

use proc_macro2::TokenStream;

/// Extension trait for determining if an Item can be adapted by c0nst::m0rph
pub trait Adaptable {
    /// Returns true if this item type can be processed by c0nst::m0rph
    fn can_adapt(&self) -> bool;
}

/// Target for transformation
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Target {
    /// Transform for stable Rust (remove const syntax)
    #[cfg_attr(not(feature = "nightly"), default)]
    Stable,

    /// Transform for nightly Rust (use modern const syntax)
    #[cfg_attr(feature = "nightly", default)]
    Nightly,
}

/// Trait for transforming syn AST nodes to handle modern const syntax
pub trait Transform {
    /// Transform this AST node for the specified target
    fn transform(&self, target: Target) -> TokenStream;
}

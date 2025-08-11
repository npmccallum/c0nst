use proc_macro2::TokenStream;
use syn::ItemFn;

use super::{Adaptable, Target, Transform};
use crate::attrs::HasAttribute;

impl Adaptable for ItemFn {
    fn can_adapt(&self) -> bool {
        true
    }
}

/// Implementation for ItemFn
impl Transform for ItemFn {
    fn transform(&self, target: Target) -> TokenStream {
        let c0nst = match (target, self.attrs.has_attribute("c0nst")) {
            (Target::Nightly, true) => Some(quote::quote! { const }),
            _ => None,
        };

        let attrs = self.attrs.transform(target);
        let vis = &self.vis;
        let sig = self.sig.transform(target);
        let block = &self.block;

        quote::quote! {
            #attrs
            #vis #c0nst #sig #block
        }
    }
}

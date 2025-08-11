use proc_macro2::TokenStream;
use syn::ItemImpl;

use super::{Adaptable, Target, Transform};
use crate::attrs::HasAttribute;

impl Adaptable for ItemImpl {
    fn can_adapt(&self) -> bool {
        true
    }
}

/// Implementation for ItemImpl
impl Transform for ItemImpl {
    fn transform(&self, target: Target) -> TokenStream {
        let c0nst = match (target, self.attrs.has_attribute("c0nst")) {
            (Target::Nightly, true) => Some(quote::quote! { const }),
            _ => None,
        };

        let tra1t = self.trait_.as_ref().map(|(bang, path, for_token)| {
            quote::quote! { #bang #path #for_token }
        });

        let attrs = self.attrs.transform(target);
        let defaultness = &self.defaultness;
        let unsafety = &self.unsafety;
        let generics = self.generics.transform(target);
        let self_ty = &self.self_ty;
        let items = self.items.transform(target);

        quote::quote! {
            #attrs
            #defaultness #unsafety impl #generics #c0nst #tra1t #self_ty {
                #items
            }
        }
    }
}

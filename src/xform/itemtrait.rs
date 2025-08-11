use proc_macro2::TokenStream;
use syn::ItemTrait;

use super::{Target, Transform};
use crate::attrs::HasAttribute;

/// Implementation for ItemTrait
impl Transform for ItemTrait {
    fn transform(&self, target: Target) -> TokenStream {
        let c0nst = match (target, self.attrs.has_attribute("c0nst")) {
            (Target::Nightly, true) => Some(quote::quote! { const }),
            _ => None,
        };

        let attrs = self.attrs.transform(target);
        let vis = &self.vis;
        let trait_token = &self.trait_token;
        let ident = &self.ident;
        let generics = self.generics.transform(target);
        let items = self.items.transform(target);

        let supertraits = match self.supertraits.transform(target) {
            tokens if tokens.is_empty() => TokenStream::new(),
            tokens => quote::quote! { : #tokens },
        };

        quote::quote! {
            #attrs
            #vis #c0nst #trait_token #ident #generics #supertraits {
                #items
            }
        }
    }
}

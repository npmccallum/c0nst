use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::TraitItem;

use super::{Target, Transform};
use crate::attrs::HasAttribute;

/// Implementation for TraitItem
impl Transform for TraitItem {
    fn transform(&self, target: Target) -> TokenStream {
        match self {
            TraitItem::Fn(method) => {
                let c0nst = match (target, method.attrs.has_attribute("c0nst")) {
                    (Target::Nightly, true) => Some(quote::quote! { const }),
                    _ => None,
                };

                let attrs = method.attrs.transform(target);
                let sig = method.sig.transform(target);
                let body = method
                    .default
                    .as_ref()
                    .map(|body| quote::quote! { #body })
                    .unwrap_or(quote::quote! { ; });

                quote::quote! {
                    #attrs
                    #c0nst #sig #body
                }
            }
            _ => self.to_token_stream(),
        }
    }
}
